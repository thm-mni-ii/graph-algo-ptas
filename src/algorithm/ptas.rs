use super::dynamic_programming::dp_max_independent_set;
use crate::utils::convert::UndirectedGraph;
use arboretum_td::graph::{HashMapGraph, MutableGraph};
use petgraph::{algo::kosaraju_scc, stable_graph::NodeIndex, visit::EdgeRef};
use std::collections::{HashSet, VecDeque};

pub fn ptas_max_independent_set(graph: &UndirectedGraph, eps: f64) -> HashSet<usize> {
    let mut sols: Vec<HashSet<usize>> = vec![];

    for ring_decomposition in get_ring_decompositions(&mut graph.clone(), eps) {
        let mut sol: HashSet<usize> = HashSet::new();

        for ring in get_component_graphs(&ring_decomposition.rings) {
            sol = sol
                .union(&dp_max_independent_set(&ring, None))
                .copied()
                .collect();
        }

        sols.push(sol);
    }

    sols.iter()
        .max_by(|s1, s2| s1.len().cmp(&s2.len()))
        .unwrap()
        .clone()
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
        algorithm::ptas::ptas_max_independent_set,
        generation::{erdos_renyi::generate_petgraph, planar::generate},
        utils::{
            convert::{petgraph_to_hash_map_graph, UndirectedGraph},
            max_independent_set::{brute_force_max_independent_set, is_independent_set},
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
                vertices = vertices
                    .union(&ring_decomposition.vertices_deleted)
                    .copied()
                    .collect();
            }

            assert!(vertices == graph.node_indices().collect());
            assert!(ring_decompositions
                .iter()
                .any(|rd| { rd.vertices_deleted.len() as f64 <= eps * graph.node_count() as f64 }));
        }
    }

    #[test]
    fn ptas_single_vertex() {
        let mut graph = UndirectedGraph::default();
        let v0 = graph.add_node(());
        let sol = ptas_max_independent_set(&graph, 0.5);

        assert!(sol.len() == 1);
        assert!(sol.contains(&v0.index()));
    }

    #[test]
    fn ptas_single_edge() {
        let mut graph = UndirectedGraph::default();
        let v0 = graph.add_node(());
        let v1 = graph.add_node(());
        graph.add_edge(v0, v1, ());
        let sol = ptas_max_independent_set(&graph, 0.5);

        assert!(sol.len() == 1);
        assert!(sol.contains(&v0.index()) || sol.contains(&v1.index()));
    }

    #[test]
    fn ptas_random() {
        for n in 2..30 {
            let graph: UndirectedGraph = generate(n, Some(n as u64)).to_pet_graph();
            let eps = 0.5;
            let sol = ptas_max_independent_set(&graph, eps);

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
}
