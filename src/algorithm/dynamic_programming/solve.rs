//! Contains data structures and algorithms for dynamic programming on tree decompositions.
//!
//! ```rust
//! use graph_algo_ptas::generation::erdos_renyi::generate_petgraph;
//! use graph_algo_ptas::algorithm::dynamic_programming::solve::dp_solve;
//! use graph_algo_ptas::algorithm::dynamic_programming::solve::DpProblem;
//!
//! let graph = generate_petgraph(20, 0.1, None);
//! let sol = dp_solve(&graph, None, &DpProblem::max_independent_set());
//! ```

use super::{max_independent_set, min_vertex_cover};
use crate::{
    algorithm::{
        dynamic_programming::utils::remap_vertices,
        nice_tree_decomposition::{get_children, NiceTdNodeType, NiceTreeDecomposition},
    },
    utils::convert::{to_hash_map_graph, UndirectedGraph},
};
use arboretum_td::{graph::HashMapGraph, solver::Solver, tree_decomposition::TreeDecomposition};
use bitvec::vec::BitVec;
use fxhash::FxHashSet;
use std::collections::{HashMap, HashSet};

/// For each bag in the tree decomposition a table is calculated.
/// Such a table is represented by `HashMap`.
///
/// The `BitVec` key represents the subset to which the table entry belongs
pub type DpTable = HashMap<BitVec, DpTableEntry>;

/// Represents a single entry in a dynamic programming table.
///
/// Contains the value of the entry and additional information needed for
/// retrieving the actual solution at the end of the algorithm.
#[derive(Debug, Clone)]
pub struct DpTableEntry {
    /// Value of the table entry. Its meaning depends on the problem to be solved.
    pub val: i32,
    /// References to table entries of child nodes.
    pub children: HashSet<(usize, BitVec)>,
    /// The vertex which is used for calculating the table entry.
    pub vertex_used: Option<usize>,
}

impl DpTableEntry {
    /// Create a table entry for a Leaf node.
    pub fn new_leaf(val: i32, vertex_used: Option<usize>) -> Self {
        Self {
            val,
            children: HashSet::new(),
            vertex_used,
        }
    }

    /// Create a table entry for a Forget node.
    pub fn new_forget(val: i32, child_id: usize, child_subset: BitVec) -> Self {
        Self {
            val,
            children: vec![(child_id, child_subset)].into_iter().collect(),
            vertex_used: None,
        }
    }

    /// Create a table entry for an Introduce node.
    pub fn new_intro(
        val: i32,
        child_id: usize,
        child_subset: BitVec,
        vertex_used: Option<usize>,
    ) -> Self {
        Self {
            val,
            children: vec![(child_id, child_subset)].into_iter().collect(),
            vertex_used,
        }
    }

    /// Create a table entry for a Join node.
    pub fn new_join(val: i32, left_id: usize, right_id: usize, subset: BitVec) -> Self {
        Self {
            val,
            children: vec![(left_id, subset.clone()), (right_id, subset)]
                .into_iter()
                .collect(),
            vertex_used: None,
        }
    }
}

type LeafNodeHandler = fn(graph: &HashMapGraph, id: usize, tables: &mut [DpTable], vertex: usize);

type JoinNodeHandler = fn(
    graph: &HashMapGraph,
    id: usize,
    left_child_id: usize,
    right_child_id: usize,
    tables: &mut [DpTable],
    vertex_set: &FxHashSet<usize>,
);

type ForgetNodeHandler = fn(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut [DpTable],
    vertex_set: &FxHashSet<usize>,
    forgotten_vertex: usize,
);

type IntroduceNodeHandler = fn(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut [DpTable],
    vertex_set: &FxHashSet<usize>,
    child_vertex_set: &FxHashSet<usize>,
    introduced_vertex: usize,
);

/// Used for differentiating between minimization and maximization problems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpObjective {
    /// Minimization problem
    Minimize,
    /// Maximization problem
    Maximize,
}

/// Contains the neccessary information for solving a (hard) problem
/// using dynamic programming on tree decompositions.
pub struct DpProblem {
    /// Indicates whether the problem is a maximization or minimization problem.
    pub objective: DpObjective,
    /// Function for calculating the the table entries at a Leaf node.
    pub handle_leaf_node: LeafNodeHandler,
    /// Function for calculating the the table entries at a Join node.
    pub handle_join_node: JoinNodeHandler,
    /// Function for calculating the the table entries at a Forget node.
    pub handle_forget_node: ForgetNodeHandler,
    /// Function for calculating the the table entries at a Introduce node.
    pub handle_introduce_node: IntroduceNodeHandler,
}

impl DpProblem {
    /// Return a `DpProblem` instance for maximum independent set.
    pub fn max_independent_set() -> DpProblem {
        DpProblem {
            objective: DpObjective::Maximize,
            handle_leaf_node: max_independent_set::handle_leaf_node,
            handle_join_node: max_independent_set::handle_join_node,
            handle_forget_node: max_independent_set::handle_forget_node,
            handle_introduce_node: max_independent_set::handle_introduce_node,
        }
    }

    /// Return a `DpProblem` instance for minimum vertex cover.
    pub fn min_vertex_cover() -> DpProblem {
        DpProblem {
            objective: DpObjective::Minimize,
            handle_leaf_node: min_vertex_cover::handle_leaf_node,
            handle_join_node: min_vertex_cover::handle_join_node,
            handle_forget_node: min_vertex_cover::handle_forget_node,
            handle_introduce_node: min_vertex_cover::handle_introduce_node,
        }
    }
}

/// Solves the given problem on the input graph using dynamic programming.
///
/// When `td` is `None`, an optimal tree decomposition is calculated and used
/// for the algorithm.
///
/// The `prob` parameter specifies whether the problem is a minimization
/// or maximization problem and contains the "recipe" for how to calculate
/// the dynamic programming tables in order to arrive at the solution.
pub fn dp_solve(
    graph: &UndirectedGraph,
    td: Option<TreeDecomposition>,
    prob: &DpProblem,
) -> HashSet<usize> {
    dp_solve_hashmap_graph(&to_hash_map_graph(graph), td, prob)
}

/// For convenience.
pub fn dp_solve_hashmap_graph(
    graph: &HashMapGraph,
    td: Option<TreeDecomposition>,
    prob: &DpProblem,
) -> HashSet<usize> {
    let (graph, mapping) = remap_vertices(graph);
    let td = td.unwrap_or_else(|| Solver::auto(&graph).solve(&graph));
    let nice_td = NiceTreeDecomposition::new(td);

    assert!(nice_td.td.verify(&graph).is_ok());

    let mut tables: Vec<_> = vec![DpTable::new(); nice_td.td.bags().len()];
    let root = nice_td.td.root.unwrap();

    dp_solve_rec(
        &nice_td.td,
        &graph,
        prob,
        root,
        usize::max_value(),
        &nice_td.mapping,
        &mut tables,
    );

    let mut sol = HashSet::new();
    dp_read_solution_from_table(prob.objective, &tables, root, &mut sol);

    sol.iter()
        .map(|v| mapping.get(v).unwrap())
        .copied()
        .collect()
}

fn dp_solve_rec(
    td: &TreeDecomposition,
    graph: &HashMapGraph,
    prob: &DpProblem,
    id: usize,
    parent_id: usize,
    mapping: &[NiceTdNodeType],
    tables: &mut Vec<DpTable>,
) {
    let children = get_children(td, id, parent_id);

    for child_id in &children {
        dp_solve_rec(td, graph, prob, *child_id, id, mapping, tables);
    }

    let vertex_set = &td.bags()[id].vertex_set;

    match mapping[id] {
        NiceTdNodeType::Leaf => {
            let vertex = vertex_set.iter().next().unwrap();
            (prob.handle_leaf_node)(graph, id, tables, *vertex);
        }
        NiceTdNodeType::Join => {
            let mut it = children.iter();
            let left_child_id = *it.next().unwrap();
            let right_child_id = *it.next().unwrap();
            (prob.handle_join_node)(graph, id, left_child_id, right_child_id, tables, vertex_set);
        }
        NiceTdNodeType::Forget(v) => {
            let child_id = *children.iter().next().unwrap();
            (prob.handle_forget_node)(graph, id, child_id, tables, vertex_set, v);
        }
        NiceTdNodeType::Introduce(v) => {
            let child_id = *children.iter().next().unwrap();
            let child_vertex_set = &td.bags()[child_id].vertex_set;
            (prob.handle_introduce_node)(
                graph,
                id,
                child_id,
                tables,
                vertex_set,
                child_vertex_set,
                v,
            );
        }
    }
}

fn dp_read_solution_from_table(
    objective: DpObjective,
    tables: &[DpTable],
    root: usize,
    sol: &mut HashSet<usize>,
) {
    let root_entry = match objective {
        DpObjective::Maximize => tables[root].values().max_by(|e1, e2| e1.val.cmp(&e2.val)),
        DpObjective::Minimize => tables[root].values().min_by(|e1, e2| e1.val.cmp(&e2.val)),
    }
    .unwrap();
    dp_read_solution_from_table_rec(tables, root_entry, sol);
}

fn dp_read_solution_from_table_rec(
    tables: &[DpTable],
    entry: &DpTableEntry,
    sol: &mut HashSet<usize>,
) {
    if let Some(v) = entry.vertex_used {
        sol.insert(v);
    }

    for (v, subset) in &entry.children {
        dp_read_solution_from_table_rec(tables, tables[*v].get(subset).unwrap(), sol);
    }
}

#[cfg(test)]
mod tests {
    use super::dp_solve_hashmap_graph;
    use crate::{
        algorithm::dynamic_programming::{
            solve::{remap_vertices, DpProblem},
            utils::init_bit_vec,
        },
        generation::erdos_renyi::generate_hash_map_graph,
        utils::{
            max_independent_set::{brute_force_max_independent_set, is_independent_set},
            min_vertex_cover::{brute_force_min_vertex_cover, is_vertex_cover},
        },
    };
    use arboretum_td::graph::{BaseGraph, HashMapGraph, MutableGraph};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::HashSet;

    fn solve_max_independent_set(graph: &HashMapGraph) -> HashSet<usize> {
        dp_solve_hashmap_graph(graph, None, &DpProblem::max_independent_set())
    }

    fn solve_min_vertex_cover(graph: &HashMapGraph) -> HashSet<usize> {
        dp_solve_hashmap_graph(graph, None, &DpProblem::min_vertex_cover())
    }

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
        let mut bit_vec = init_bit_vec(65);
        bit_vec.set(127, true);
    }

    #[test]
    fn max_independent_set_isolated() {
        for n in 1..10 {
            let graph = generate_hash_map_graph(n, 0., Some(n as u64));

            let sol = solve_max_independent_set(&graph);

            assert!(sol.len() == n);
        }
    }

    #[test]
    fn max_independent_set_clique() {
        for n in 1..10 {
            let graph = generate_hash_map_graph(n, 1., Some(n as u64));
            let sol = solve_max_independent_set(&graph);

            assert!(sol.len() == 1);
        }
    }

    #[test]
    fn max_independent_set_random() {
        let seed = [1; 32];
        let mut rng = StdRng::from_seed(seed);

        for i in 0..30 {
            let graph = generate_hash_map_graph(
                rng.gen_range(1..15),
                rng.gen_range(0.05..0.1),
                Some(i as u64),
            );
            let sol = solve_max_independent_set(&graph);

            assert!(is_independent_set(&graph, &sol), "{:?} {:?}", graph, sol);

            let sol2 = brute_force_max_independent_set(&graph);
            assert!(sol.len() == sol2.len());
        }
    }

    #[test]
    fn min_vertex_cover_isolated() {
        for n in 1..10 {
            let graph = generate_hash_map_graph(n, 0., Some(n as u64));
            let sol = solve_min_vertex_cover(&graph);

            assert!(sol.is_empty());
        }
    }

    #[test]
    fn min_vertex_cover_clique() {
        for n in 1..10 {
            let graph = generate_hash_map_graph(n, 1., Some(n as u64));
            let sol = solve_min_vertex_cover(&graph);

            assert!(sol.len() == graph.order() - 1);
        }
    }

    #[test]
    fn min_vertex_cover_random() {
        let seed = [2; 32];
        let mut rng = StdRng::from_seed(seed);

        for i in 0..30 {
            let graph = generate_hash_map_graph(
                rng.gen_range(1..15),
                rng.gen_range(0.2..0.5),
                Some(i as u64),
            );
            let sol = solve_min_vertex_cover(&graph);

            assert!(is_vertex_cover(&graph, &sol));

            let sol2 = brute_force_min_vertex_cover(&graph);
            assert!(sol.len() == sol2.len());
        }
    }
}
