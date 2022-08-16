//! Contains algorithms for generating graphs based on the Erdos-Renyi model

use crate::utils::convert::{petgraph_to_hash_map_graph, UndirectedGraph};
use arboretum_td::graph::HashMapGraph;
use petgraph::stable_graph::NodeIndex;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// Generates a random graph of type [UndirectedGraph]
pub fn generate_petgraph(n: usize, p: f64, seed: Option<u64>) -> UndirectedGraph {
    let mut graph = UndirectedGraph::default();
    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    for _ in 0..n {
        graph.add_node(());
    }

    for v in 0..n {
        for w in v + 1..n {
            if rng.gen_bool(p) {
                graph.add_edge(NodeIndex::new(v), NodeIndex::new(w), ());
            }
        }
    }

    graph
}

/// Generates a random graph of type [HashMapGraph]
pub fn generate_hashmap_graph(n: usize, p: f64, seed: Option<u64>) -> HashMapGraph {
    petgraph_to_hash_map_graph(&generate_petgraph(n, p, seed))
}
