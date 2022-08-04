use super::nice_tree_decomposition::{get_child_bag_ids, NiceTdNodeType, NiceTreeDecomposition};
use arboretum_td::{
    graph::{BaseGraph, HashMapGraph},
    solver::Solver,
    tree_decomposition::TreeDecomposition,
};
use bitvec::vec::BitVec;
use fxhash::FxHasher;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};

type FxHashSet<T> = HashSet<T, BuildHasherDefault<FxHasher>>;

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

fn solve_max_independent_set(graph: &HashMapGraph) -> (Vec<bool>, usize) {
    let td = Solver::default().solve(graph);
    let nice_td = NiceTreeDecomposition::new(td);

    let mut tables: Vec<_> = vec![DynamicProgrammingTable::new(); nice_td.td.bags().len()];
    let root = nice_td.td.root.unwrap();

    solve_max_independent_set_rec(
        &nice_td.td,
        graph,
        root,
        &nice_td.td.bags()[root].neighbors.clone(),
        &nice_td.mapping,
        &mut tables,
    );

    let sol = vec![false; graph.order()];
    read_max_independent_set_solution(&tables, root, sol)
}

fn solve_max_independent_set_rec(
    td: &TreeDecomposition,
    graph: &HashMapGraph,
    id: usize,
    children: &FxHashSet<usize>,
    mapping: &[NiceTdNodeType],
    tables: &mut Vec<DynamicProgrammingTable>,
) {
    for child_id in children {
        solve_max_independent_set_rec(
            td,
            graph,
            *child_id,
            &get_child_bag_ids(td, *child_id, id),
            mapping,
            tables,
        );
    }

    match mapping[id] {
        NiceTdNodeType::Leaf => {
            tables[id].insert(
                BitVec::from_element(0),
                DynamicProgrammingTableEntry::new_leaf(0, None),
            );
            let v = *td.bags()[id].vertex_set.iter().next().unwrap();
            tables[id].insert(
                subset_with(&BitVec::from_element(0), v),
                DynamicProgrammingTableEntry::new_leaf(1, Some(v)),
            );
        }
        NiceTdNodeType::Join => {
            let vertex_set = &td.bags()[id].vertex_set;
            let mut it = children.iter();
            let left_id = it.next().unwrap();
            let right_id = it.next().unwrap();

            for subset in powerset(vertex_set) {
                let left_val = tables[*left_id].get(&subset).unwrap().val;
                let right_val = tables[*right_id].get(&subset).unwrap().val;
                let val = left_val
                    .checked_add(right_val)
                    .and_then(|x| x.checked_sub(subset.count_ones() as i32))
                    .unwrap_or(i32::min_value());

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

            for subset in powerset(vertex_set) {
                let val = tables[child_id].get(&subset).unwrap().val;
                let subset_with_v = subset_with(&subset, v);
                let val_with_v = tables[child_id].get(&subset_with_v).unwrap().val;
                let (max_val, subset_used) = if val > val_with_v {
                    (val, subset.clone())
                } else {
                    (val_with_v, subset_with_v)
                };
                tables[id].insert(
                    subset,
                    DynamicProgrammingTableEntry::new_forget(max_val, child_id, subset_used),
                );
            }
        }
        NiceTdNodeType::Introduce(v) => {
            let mut it = children.iter();
            let child_id = *it.next().unwrap();
            let child_vertex_set: &FxHashSet<usize> = &td.bags()[child_id].vertex_set;

            for subset in powerset(child_vertex_set) {
                let val = tables[child_id].get(&subset).unwrap().val;
                tables[id].insert(
                    subset.clone(),
                    DynamicProgrammingTableEntry::new_intro(val, child_id, subset.clone(), None),
                );

                let mut has_edge = false;
                for (w, is_included) in subset.iter().enumerate() {
                    if *is_included && graph.has_edge(v, w) {
                        has_edge = true;
                        break;
                    }
                }

                let (val, node_used) = if has_edge {
                    (i32::min_value(), None)
                } else {
                    (val + 1, Some(v))
                };

                let subset_with_v = subset_with(&subset, v);
                tables[id].insert(
                    subset_with_v,
                    DynamicProgrammingTableEntry::new_intro(val, child_id, subset, node_used),
                );
            }
        }
    }
}

fn read_max_independent_set_solution(
    tables: &[DynamicProgrammingTable],
    root: usize,
    mut sol: Vec<bool>,
) -> (Vec<bool>, usize) {
    let root_entry = tables[root]
        .values()
        .max_by(|e1, e2| e1.val.cmp(&e2.val))
        .unwrap();
    read_max_independent_set_solution_rec(tables, root_entry, &mut sol);

    (sol, root_entry.val as usize)
}

fn read_max_independent_set_solution_rec(
    tables: &[DynamicProgrammingTable],
    entry: &DynamicProgrammingTableEntry,
    sol: &mut Vec<bool>,
) {
    if let Some(v) = entry.node_used {
        sol[v] = true;
    }

    for (v, subset) in &entry.children {
        read_max_independent_set_solution_rec(tables, tables[*v].get(subset).unwrap(), sol);
    }
}

fn powerset(set: &FxHashSet<usize>) -> Vec<BitVec> {
    set.iter()
        .powerset()
        .map(|subset| to_bit_vec(subset.iter().copied()))
        .collect()
}

fn to_bit_vec<'a>(it: impl Iterator<Item = &'a usize>) -> BitVec {
    let mut bit_vec: BitVec = BitVec::from_element(0);

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
    use super::solve_max_independent_set;
    use crate::utils::random_graph::random_hashmap_graph;
    use arboretum_td::graph::{BaseGraph, HashMapGraph};
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn isolated() {
        for n in 1..10 {
            let seed = [1; 32];
            let mut rng = StdRng::from_seed(seed);
            let graph = random_hashmap_graph(n, 0., &mut rng);
            let (sol, size) = solve_max_independent_set(&graph);

            assert!(size == n);
            assert!(sol.iter().all(|b| { *b }));
        }
    }

    #[test]
    fn clique() {
        for n in 1..10 {
            let seed = [1; 32];
            let mut rng = StdRng::from_seed(seed);
            let graph = random_hashmap_graph(n, 1., &mut rng);
            let (sol, size) = solve_max_independent_set(&graph);

            assert!(size == 1);
            assert!(sol.iter().filter(|b| { **b }).count() == 1);
        }
    }

    #[test]
    fn random() {
        let seed = [1; 32];
        let mut rng = StdRng::from_seed(seed);

        for _ in 0..30 {
            let graph =
                random_hashmap_graph(rng.gen_range(1..30), rng.gen_range(0.05..0.1), &mut rng);
            let (sol, _) = solve_max_independent_set(&graph);
            assert!(is_independet_set(&graph, &sol));
        }
    }

    fn is_independet_set(graph: &HashMapGraph, sol: &[bool]) -> bool {
        for u in 0..sol.len() {
            if !sol[u] {
                continue;
            }

            for (v, is_included) in sol.iter().enumerate().skip(u + 1) {
                if !is_included {
                    continue;
                }

                if graph.has_edge(u, v) {
                    return false;
                }
            }
        }

        true
    }
}
