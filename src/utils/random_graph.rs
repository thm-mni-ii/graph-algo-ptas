use super::convert::{petgraph_to_hash_map_graph, UndirectedGraph};
use arboretum_td::graph::HashMapGraph;
use petgraph::stable_graph::NodeIndex;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_graph(n: usize, p: f64, seed: Option<u64>) -> UndirectedGraph {
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

pub fn random_hashmap_graph(n: usize, p: f64, seed: Option<u64>) -> HashMapGraph {
    petgraph_to_hash_map_graph(&random_graph(n, p, seed))
}
