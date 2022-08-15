use super::nice_tree_decomposition::{get_children, NiceTdNodeType, NiceTreeDecomposition};
use arboretum_td::{
    graph::{BaseGraph, HashMapGraph, MutableGraph},
    solver::Solver,
    tree_decomposition::TreeDecomposition,
};
use bitvec::vec::BitVec;
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// The BitVec key represents the subset to which the table entry belongs
type DynamicProgrammingTable = HashMap<BitVec, DynamicProgrammingTableEntry>;

#[derive(Debug, Clone)]
struct DynamicProgrammingTableEntry {
    val: i32,
    children: HashSet<(usize, BitVec)>,
    node_used: Option<usize>,
}

impl DynamicProgrammingTableEntry {
    fn new_leaf(val: i32, node_used: Option<usize>) -> Self {
        Self {
            val,
            children: HashSet::new(),
            node_used,
        }
    }

    fn new_forget(val: i32, child_id: usize, child_subset: BitVec) -> Self {
        Self {
            val,
            children: vec![(child_id, child_subset)].into_iter().collect(),
            node_used: None,
        }
    }

    fn new_intro(
        val: i32,
        child_id: usize,
        child_subset: BitVec,
        node_used: Option<usize>,
    ) -> Self {
        Self {
            val,
            children: vec![(child_id, child_subset)].into_iter().collect(),
            node_used,
        }
    }

    fn new_join(val: i32, left_id: usize, right_id: usize, subset: BitVec) -> Self {
        Self {
            val,
            children: vec![(left_id, subset.clone()), (right_id, subset)]
                .into_iter()
                .collect(),
            node_used: None,
        }
    }
}

// the result is a graph isomorphic to the input graph but with vertex IDs 0..n-1.
fn remap_vertices(graph: &HashMapGraph) -> (HashMapGraph, HashMap<usize, usize>) {
    let mut remapped_graph = HashMapGraph::new();
    let mut forward_mapping = HashMap::new();
    let mut backward_mapping = HashMap::new();

    for (i, v) in graph.vertices().enumerate() {
        remapped_graph.add_vertex(i);
        forward_mapping.insert(v, i);
        backward_mapping.insert(i, v);
    }

    for u in graph.vertices() {
        for v in graph.neighborhood(u) {
            let remapped_u = forward_mapping.get(&u).unwrap();
            let remapped_v = forward_mapping.get(&v).unwrap();
            remapped_graph.add_edge(*remapped_u, *remapped_v);
        }
    }

    (remapped_graph, backward_mapping)
}

pub fn dp_max_independent_set(
    graph: &HashMapGraph,
    td: Option<TreeDecomposition>,
) -> HashSet<usize> {
    let (graph, mapping) = remap_vertices(graph);
    let td = td.unwrap_or_else(|| Solver::default().solve(&graph));
    let nice_td = NiceTreeDecomposition::new(td);

    assert!(nice_td.td.verify(&graph).is_ok());

    let mut tables: Vec<_> = vec![DynamicProgrammingTable::new(); nice_td.td.bags().len()];
    let root = nice_td.td.root.unwrap();

    dp_max_independent_set_rec(
        &nice_td.td,
        &graph,
        root,
        usize::max_value(),
        &nice_td.mapping,
        &mut tables,
    );

    let mut sol = HashSet::new();
    read_max_independent_set_solution(&tables, root, &mut sol);

    sol.iter()
        .map(|v| mapping.get(v).unwrap())
        .copied()
        .collect()
}

fn dp_max_independent_set_rec(
    td: &TreeDecomposition,
    graph: &HashMapGraph,
    id: usize,
    parent_id: usize,
    mapping: &[NiceTdNodeType],
    tables: &mut Vec<DynamicProgrammingTable>,
) {
    let children = get_children(td, id, parent_id);

    for child_id in &children {
        dp_max_independent_set_rec(td, graph, *child_id, id, mapping, tables);
    }

    match mapping[id] {
        NiceTdNodeType::Leaf => {
            tables[id].insert(
                make_bit_vec(graph.order()),
                DynamicProgrammingTableEntry::new_leaf(0, None),
            );
            let v = *td.bags()[id].vertex_set.iter().next().unwrap();
            tables[id].insert(
                subset_with(&make_bit_vec(graph.order()), v),
                DynamicProgrammingTableEntry::new_leaf(1, Some(v)),
            );
        }
        NiceTdNodeType::Join => {
            let vertex_set = &td.bags()[id].vertex_set;

            let mut children_it = children.iter();
            let left_id = children_it.next().unwrap();
            let right_id = children_it.next().unwrap();

            for subset_vec in vertex_set.iter().powerset() {
                let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
                let left_val = tables[*left_id].get(&subset).unwrap().val;
                let right_val = tables[*right_id].get(&subset).unwrap().val;
                let new_val = if left_val == i32::min_value() || right_val == i32::min_value() {
                    i32::min_value()
                } else {
                    left_val + right_val - subset_vec.len() as i32
                };

                // left_val
                //     .checked_add(right_val)
                //     .and_then(|x| x.checked_sub(subset.count_ones() as i32))
                //     .unwrap_or(i32::min_value());

                tables[id].insert(
                    subset.clone(),
                    DynamicProgrammingTableEntry::new_join(new_val, *left_id, *right_id, subset),
                );
            }
        }
        NiceTdNodeType::Forget(v) => {
            let vertex_set = &td.bags()[id].vertex_set;
            let child_id = *children.iter().next().unwrap();

            for subset in powerset(vertex_set, graph.order()) {
                let val = tables[child_id].get(&subset).unwrap().val;
                let subset_with_v = subset_with(&subset, v);
                let val_with_v = tables[child_id].get(&subset_with_v).unwrap().val;
                let (new_val, subset_used) = if val > val_with_v {
                    (val, subset.clone())
                } else {
                    (val_with_v, subset_with_v)
                };
                tables[id].insert(
                    subset,
                    DynamicProgrammingTableEntry::new_forget(new_val, child_id, subset_used),
                );
            }
        }
        NiceTdNodeType::Introduce(v) => {
            let child_id = *children.iter().next().unwrap();
            let child_vertex_set: &FxHashSet<usize> = &td.bags()[child_id].vertex_set;

            for subset_vec in child_vertex_set.iter().powerset() {
                let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
                let val = tables[child_id].get(&subset).unwrap().val;
                tables[id].insert(
                    subset.clone(),
                    DynamicProgrammingTableEntry::new_intro(val, child_id, subset.clone(), None),
                );

                let has_edge = subset_vec.iter().any(|w| graph.has_edge(v, **w));
                //  for w in subset_vec {
                //      if graph.has_edge(v, *w) {
                //          has_edge = true;
                //          break;
                //      }
                //  }

                let (new_val, node_used) = if has_edge {
                    (i32::min_value(), None)
                } else {
                    (val + 1, Some(v))
                };

                let subset_with_v = subset_with(&subset, v);
                tables[id].insert(
                    subset_with_v,
                    DynamicProgrammingTableEntry::new_intro(new_val, child_id, subset, node_used),
                );
            }
        }
    }
}

fn read_max_independent_set_solution(
    tables: &[DynamicProgrammingTable],
    root: usize,
    sol: &mut HashSet<usize>,
) {
    let root_entry = tables[root]
        .values()
        .max_by(|e1, e2| e1.val.cmp(&e2.val))
        .unwrap();
    read_max_independent_set_solution_rec(tables, root_entry, sol);
}

fn read_max_independent_set_solution_rec(
    tables: &[DynamicProgrammingTable],
    entry: &DynamicProgrammingTableEntry,
    sol: &mut HashSet<usize>,
) {
    if let Some(v) = entry.node_used {
        sol.insert(v);
    }

    for (v, subset) in &entry.children {
        read_max_independent_set_solution_rec(tables, tables[*v].get(subset).unwrap(), sol);
    }
}

//////////////////////////////////////////////////////////////
/// vertex covert
//////////////////////////////////////////////////////////////

pub fn dp_min_vertex_cover(graph: &HashMapGraph, td: Option<TreeDecomposition>) -> HashSet<usize> {
    let (graph, mapping) = remap_vertices(graph);
    let td = td.unwrap_or_else(|| Solver::default_exact().solve(&graph));

    let nice_td = NiceTreeDecomposition::new(td);
    println!(
        "TW: {}, node_count: {}",
        nice_td.td.max_bag_size - 1,
        nice_td.td.bags().len()
    );

    assert!(nice_td.td.verify(&graph).is_ok());

    let mut tables: Vec<_> = vec![DynamicProgrammingTable::new(); nice_td.td.bags().len()];
    let root = nice_td.td.root.unwrap();

    dp_min_vertex_cover_rec(
        &nice_td.td,
        &graph,
        root,
        usize::max_value(),
        &nice_td.mapping,
        &mut tables,
    );

    let mut sol = HashSet::new();
    read_min_vertex_cover_solution(&tables, root, &mut sol);

    sol.iter()
        .map(|v| mapping.get(v).unwrap())
        .copied()
        .collect()
}

fn dp_min_vertex_cover_rec(
    td: &TreeDecomposition,
    graph: &HashMapGraph,
    id: usize,
    parent_id: usize,
    mapping: &[NiceTdNodeType],
    tables: &mut Vec<DynamicProgrammingTable>,
) {
    let children = get_children(td, id, parent_id);

    for child_id in &children {
        dp_min_vertex_cover_rec(td, graph, *child_id, id, mapping, tables);
    }

    match mapping[id] {
        NiceTdNodeType::Leaf => {
            tables[id].insert(
                make_bit_vec(graph.order()),
                DynamicProgrammingTableEntry::new_leaf(0, None),
            );
            let v = *td.bags()[id].vertex_set.iter().next().unwrap();
            tables[id].insert(
                subset_with(&make_bit_vec(graph.order()), v),
                DynamicProgrammingTableEntry::new_leaf(1, Some(v)),
            );
        }
        NiceTdNodeType::Join => {
            let vertex_set = &td.bags()[id].vertex_set;
            let mut it = children.iter();
            let left_id = it.next().unwrap();
            let right_id = it.next().unwrap();

            for subset in powerset(vertex_set, graph.order()) {
                let left_val = tables[*left_id].get(&subset).unwrap().val;
                let right_val = tables[*right_id].get(&subset).unwrap().val;
                let val = left_val
                    .checked_add(right_val)
                    .and_then(|x| x.checked_sub(subset.count_ones() as i32))
                    .unwrap_or(i32::max_value());

                tables[id].insert(
                    subset.clone(),
                    DynamicProgrammingTableEntry::new_join(val, *left_id, *right_id, subset),
                );
            }
        }
        NiceTdNodeType::Forget(v) => {
            let vertex_set = &td.bags()[id].vertex_set;
            let mut it = children.iter();
            let child_id = *it.next().unwrap();

            for subset in powerset(vertex_set, graph.order()) {
                let val = tables[child_id].get(&subset).unwrap().val;
                let subset_with_v = subset_with(&subset, v);
                let val_with_v = tables[child_id].get(&subset_with_v).unwrap().val;
                let (min_val, subset_used) = if val < val_with_v {
                    (val, subset.clone())
                } else {
                    (val_with_v, subset_with_v)
                };
                tables[id].insert(
                    subset,
                    DynamicProgrammingTableEntry::new_forget(min_val, child_id, subset_used),
                );
            }
        }
        NiceTdNodeType::Introduce(v) => {
            let mut it = children.iter();
            let child_id = *it.next().unwrap();
            let child_vertex_set: &FxHashSet<usize> = &td.bags()[child_id].vertex_set;

            for subset_vec in child_vertex_set.iter().powerset() {
                let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
                let neighbors = graph
                    .neighborhood_set(v)
                    .iter()
                    .filter(|w| child_vertex_set.contains(w));

                let mut is_covered = true;
                for w in neighbors {
                    if !subset_vec.contains(&w) {
                        is_covered = false;
                        break;
                    }
                }

                let val = if is_covered {
                    tables[child_id].get(&subset).unwrap().val
                } else {
                    i32::max_value()
                };
                let mut children = HashSet::new();
                children.insert((child_id, subset.clone()));
                tables[id].insert(
                    subset.clone(),
                    DynamicProgrammingTableEntry {
                        val,
                        children: children.clone(),
                        node_used: None,
                    },
                );

                let child_val = tables[child_id].get(&subset).unwrap().val;
                let val = if child_val < i32::max_value() {
                    child_val + 1
                } else {
                    child_val
                };
                tables[id].insert(
                    subset_with(&subset, v),
                    DynamicProgrammingTableEntry {
                        val,
                        children,
                        node_used: Some(v),
                    },
                );
            }
        }
    }
}

fn read_min_vertex_cover_solution(
    tables: &[DynamicProgrammingTable],
    root: usize,
    sol: &mut HashSet<usize>,
) {
    let root_entry = tables[root]
        .values()
        .min_by(|e1, e2| e1.val.cmp(&e2.val))
        .unwrap();
    read_max_independent_set_solution_rec(tables, root_entry, sol);
}

fn read_min_vertex_cover_solution_rec(
    tables: &[DynamicProgrammingTable],
    entry: &DynamicProgrammingTableEntry,
    sol: &mut HashSet<usize>,
) {
    if let Some(v) = entry.node_used {
        sol.insert(v);
    }

    for (v, subset) in &entry.children {
        read_min_vertex_cover_solution_rec(tables, tables[*v].get(subset).unwrap(), sol);
    }
}

fn powerset(set: &FxHashSet<usize>, subset_size: usize) -> Vec<BitVec> {
    set.iter()
        .powerset()
        .map(|subset| to_bit_vec(subset.iter().copied(), subset_size))
        .collect()
}

fn make_bit_vec(size: usize) -> BitVec {
    let n = (size as f64 / 64.0).ceil() as usize;

    BitVec::from_vec(vec![0; n])
}

fn to_bit_vec<'a>(it: impl Iterator<Item = &'a usize>, size: usize) -> BitVec {
    let mut bit_vec: BitVec = make_bit_vec(size);

    for x in it {
        bit_vec.set(*x, true);
    }

    bit_vec
}

fn subset_with(subset: &BitVec, v: usize) -> BitVec {
    let mut subset = subset.clone();
    subset.set(v, true);
    subset
}

#[cfg(test)]
mod tests {
    use super::{dp_max_independent_set, make_bit_vec, remap_vertices};
    use crate::{
        algorithm::dynamic_programming::dp_min_vertex_cover,
        generation::erdos_renyi::generate_hashmap_graph,
        utils::{
            max_independent_set::{brute_force_max_independent_set, is_independent_set},
            min_vertex_cover::{brute_force_min_vertex_cover, is_vertex_cover},
        },
    };
    use arboretum_td::graph::{BaseGraph, HashMapGraph, MutableGraph};
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn remapping() {
        let mut graph = HashMapGraph::new();
        graph.add_vertex(10);
        graph.add_vertex(11);
        graph.add_vertex(12);
        graph.add_edge(10, 11);

        let (remapped_graph, _) = remap_vertices(&graph);

        assert!(remapped_graph.order() == graph.order());
        assert!(remapped_graph.has_vertex(0));
        assert!(remapped_graph.has_vertex(1));
        assert!(remapped_graph.has_vertex(2));
        assert!(remapped_graph.has_edge(0, 1) ^ remapped_graph.has_edge(1, 2));
    }

    #[test]
    fn large_bit_vec() {
        let mut bit_vec = make_bit_vec(65);
        bit_vec.set(127, true);
    }

    #[test]
    fn isolated() {
        for n in 1..10 {
            let graph = generate_hashmap_graph(n, 0., Some(n as u64));
            let sol = dp_max_independent_set(&graph, None);

            assert!(sol.len() == n);
        }
    }

    #[test]
    fn clique() {
        for n in 1..10 {
            let graph = generate_hashmap_graph(n, 1., Some(n as u64));
            let sol = dp_max_independent_set(&graph, None);

            assert!(sol.len() == 1);
        }
    }

    #[test]
    fn random() {
        let seed = [1; 32];
        let mut rng = StdRng::from_seed(seed);

        for i in 0..30 {
            let graph = generate_hashmap_graph(
                rng.gen_range(1..15),
                rng.gen_range(0.05..0.1),
                Some(i as u64),
            );
            let sol = dp_max_independent_set(&graph, None);

            assert!(is_independent_set(&graph, &sol), "{:?} {:?}", graph, sol);

            let sol2 = brute_force_max_independent_set(&graph);
            assert!(sol.len() == sol2.len());
        }
    }

    #[test]
    fn isolated_vertex_cover() {
        for n in 1..10 {
            let graph = generate_hashmap_graph(n, 0., Some(n as u64));
            let sol = dp_min_vertex_cover(&graph, None);

            assert!(sol.is_empty());
        }
    }

    #[test]
    fn clique_vertex_cover() {
        for n in 1..10 {
            let graph = generate_hashmap_graph(n, 1., Some(n as u64));
            let sol = dp_min_vertex_cover(&graph, None);

            assert!(sol.len() == graph.order() - 1);
        }
    }

    #[test]
    fn random_vertex_cover() {
        let seed = [2; 32];
        let mut rng = StdRng::from_seed(seed);

        // let graph = generate_hashmap_graph(100, 0.01, Some(1));
        // let sol = dp_min_vertex_cover(&graph, None);
        // println!("{:?}", sol);
        //
        //       // assert!(is_vertex_cover(&graph, &sol));
        //
        for i in 0..30 {
            let graph = generate_hashmap_graph(
                rng.gen_range(1..15),
                rng.gen_range(0.2..0.5),
                Some(i as u64),
            );
            let sol = dp_min_vertex_cover(&graph, None);

            assert!(is_vertex_cover(&graph, &sol));

            let sol2 = brute_force_min_vertex_cover(&graph);
            assert!(sol.len() == sol2.len());
        }
    }
}
