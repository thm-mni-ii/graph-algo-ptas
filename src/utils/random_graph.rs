use super::convert::{petgraph_to_hash_map_graph, UndirectedGraph};
use arboretum_td::graph::HashMapGraph;
use petgraph::stable_graph::NodeIndex;
use rand::Rng;

pub fn random_graph<R: Rng>(n: usize, p: f64, rng: &mut R) -> UndirectedGraph {
    let mut graph = UndirectedGraph::default();

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

pub fn random_hashmap_graph<R: Rng>(n: usize, p: f64, rng: &mut R) -> HashMapGraph {
    petgraph_to_hash_map_graph(&random_graph(n, p, rng))
}
