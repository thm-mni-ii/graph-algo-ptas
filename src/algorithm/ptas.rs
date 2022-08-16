//! Contains the main algorithm implementing the PTAS for planar graphs.

use super::dynamic_programming::solve::{dp_solve, DynamicProgrammingProblem, Objective};
use crate::utils::convert::UndirectedGraph;
use arboretum_td::graph::{HashMapGraph, MutableGraph};
use petgraph::{algo::kosaraju_scc, stable_graph::NodeIndex, visit::EdgeRef};
use std::collections::{HashSet, VecDeque};

/// Calculates an approximate solution for the given problem on the input graph.
/// The input graph is expected to be planar.
///
/// The solution is guaranteed to be (1 - eps) optimal for maximization problems
/// and (1 + eps) optimal for minimization problems.
pub fn ptas(graph: &UndirectedGraph, prob: &DynamicProgrammingProblem, eps: f64) -> HashSet<usize> {
    let mut sols: Vec<HashSet<usize>> = vec![];

    for ring_decomposition in get_ring_decompositions(&mut graph.clone(), eps) {
        let mut sol: HashSet<usize> = HashSet::new();

        for ring in get_component_graphs(&ring_decomposition.rings) {
            let ring_sol = dp_solve(&ring, None, prob);
            sol.extend(ring_sol.iter());
        }

        if prob.objective == Objective::Minimize {
            let vertices_deleted = ring_decomposition
                .vertices_deleted
                .iter()
                .map(|v| v.index());
            sol.extend(vertices_deleted);
        }

        sols.push(sol);
    }

    let best_sol = match prob.objective {
        Objective::Minimize => sols.iter().min_by(|s1, s2| s1.len().cmp(&s2.len())),
        Objective::Maximize => sols.iter().max_by(|s1, s2| s1.len().cmp(&s2.len())),
    };

    best_sol.unwrap().clone()
}

fn get_component_graphs(graph: &UndirectedGraph) -> Vec<HashMapGraph> {
    let mut component_graphs = vec![];

    for component in &kosaraju_scc(&graph) {
        let mut component_graph = HashMapGraph::new();

        for v in component {
            component_graph.add_vertex(v.index());
        }

        for v in component {
            for e in graph.edges(*v) {
                component_graph.add_edge(e.source().index(), e.target().index());
            }
        }

        component_graphs.push(component_graph);
    }

    component_graphs
}

struct RingDecomposition {
    rings: UndirectedGraph,
    vertices_deleted: HashSet<NodeIndex>,
}

fn get_ring_decompositions(graph: &mut UndirectedGraph, eps: f64) -> Vec<RingDecomposition> {
    let k = (1.0 / eps).ceil() as usize;
    assert!(kosaraju_scc(&graph.clone()).len() == 1);
    assert!(graph.node_count() > 0);

    let mut ring_decompositions: Vec<RingDecomposition> = vec![];

    for i in 0..k {
        let mut rings = graph.clone();
        let mut vertices_deleted = HashSet::new();
        let mut level = 1;
        let start = graph.node_indices().next().unwrap();
        let mut visited: HashSet<NodeIndex<u32>> = HashSet::new();
        let mut queue: VecDeque<NodeIndex<u32>> = VecDeque::new();
        let sep = NodeIndex::new(usize::max_value());
        queue.push_back(start);
        queue.push_back(sep);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            if current == sep {
                level += 1;

                if !queue.is_empty() {
                    queue.push_back(sep);
                }

                continue;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            if level % k == i {
                vertices_deleted.insert(current);
                rings.remove_node(current);
            }

            for n in graph.neighbors(current) {
                queue.push_back(n);
            }
        }

        ring_decompositions.push(RingDecomposition {
            rings,
            vertices_deleted,
        });
    }

    ring_decompositions
}

#[cfg(test)]
mod tests {
    use super::get_ring_decompositions;
    use crate::{
        algorithm::{dynamic_programming::solve::DynamicProgrammingProblem, ptas::ptas},
        generation::{erdos_renyi::generate_petgraph, planar::generate},
        utils::{
            convert::{petgraph_to_hash_map_graph, UndirectedGraph},
            max_independent_set::{brute_force_max_independent_set, is_independent_set},
            min_vertex_cover::{brute_force_min_vertex_cover, is_vertex_cover},
        },
    };
    use petgraph::algo::kosaraju_scc;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::HashSet;

    #[test]
    fn approximation_ratio() {
        let seed = [2; 32];
        let mut rng = StdRng::from_seed(seed);

        let mut i = 0;
        while i < 100 {
            let graph = generate_petgraph(
                rng.gen_range(1..100),
                rng.gen_range(0.1..1.0),
                Some(i as u64),
            );

            if kosaraju_scc(&graph).len() != 1 {
                continue;
            }

            i += 1;

            let eps = rng.gen_range(0.05..0.5) as f64;
            let ring_decompositions = get_ring_decompositions(&mut graph.clone(), eps);
            let mut vertices = HashSet::new();

            for ring_decomposition in &ring_decompositions {
                vertices.extend(ring_decomposition.vertices_deleted.iter());
            }

            assert!(vertices == graph.node_indices().collect());
            assert!(ring_decompositions
                .iter()
                .any(|rd| { rd.vertices_deleted.len() as f64 <= eps * graph.node_count() as f64 }));
        }
    }

    #[test]
    fn max_independent_set_single_vertex() {
        let mut graph = UndirectedGraph::default();
        let v0 = graph.add_node(());
        let sol = ptas(
            &graph,
            &DynamicProgrammingProblem::max_independent_set(),
            0.5,
        );

        assert!(sol.len() == 1);
        assert!(sol.contains(&v0.index()));
    }

    #[test]
    fn max_independent_set_single_edge() {
        let mut graph = UndirectedGraph::default();
        let v0 = graph.add_node(());
        let v1 = graph.add_node(());
        graph.add_edge(v0, v1, ());
        let sol = ptas(
            &graph,
            &DynamicProgrammingProblem::max_independent_set(),
            0.5,
        );
        assert!(sol.len() == 1);
        assert!(sol.contains(&v0.index()) || sol.contains(&v1.index()));
    }

    #[test]
    fn max_independent_set_random() {
        for n in 2..30 {
            let graph: UndirectedGraph = generate(n, Some(n as u64)).to_pet_graph();
            let eps = 0.5;
            let sol = ptas(
                &graph,
                &DynamicProgrammingProblem::max_independent_set(),
                eps,
            );

            assert!(is_independent_set(
                &petgraph_to_hash_map_graph(&graph),
                &sol
            ));

            // if n > 15 the brute force algorithm takes too long
            if n <= 15 {
                let sol2 = brute_force_max_independent_set(&petgraph_to_hash_map_graph(&graph));

                assert!(sol.len() as f64 >= (1.0 - eps) * sol2.len() as f64);
            }
        }
    }

    #[test]
    fn min_vertex_cover_single_vertex() {
        let mut graph = UndirectedGraph::default();
        graph.add_node(());
        let sol = ptas(&graph, &DynamicProgrammingProblem::min_vertex_cover(), 0.5);

        assert!(sol.is_empty());
    }

    #[test]
    fn min_vertex_cover_single_edge() {
        let mut graph = UndirectedGraph::default();
        let v0 = graph.add_node(());
        let v1 = graph.add_node(());
        graph.add_edge(v0, v1, ());
        let sol = ptas(&graph, &DynamicProgrammingProblem::min_vertex_cover(), 0.5);
        assert!(sol.len() == 1);
        assert!(sol.contains(&v0.index()) || sol.contains(&v1.index()));
    }

    #[test]
    fn min_vertex_cover_random() {
        for n in 2..30 {
            let graph: UndirectedGraph = generate(n, Some(n as u64)).to_pet_graph();
            let eps = 0.5;
            let sol = ptas(&graph, &DynamicProgrammingProblem::min_vertex_cover(), eps);

            assert!(is_vertex_cover(&petgraph_to_hash_map_graph(&graph), &sol));

            // if n > 15 the brute force algorithm takes too long
            if n <= 15 {
                let sol2 = brute_force_min_vertex_cover(&petgraph_to_hash_map_graph(&graph));

                assert!(sol.len() as f64 <= (1.0 + 0.5 * eps) * sol2.len() as f64);
            }
        }
    }
}
