use arboretum_td::graph::{HashMapGraph, MutableGraph};
use rand::Rng;

pub fn random_hashmap_graph<R: Rng>(n: usize, p: f64, rng: &mut R) -> HashMapGraph {
    let mut graph = HashMapGraph::new();

    for v in 0..n {
        graph.add_vertex(v);
    }

    for v in 0..n {
        for w in v + 1..n {
            if rng.gen_bool(p) {
                graph.add_edge(v, w);
            }
        }
    }

    graph
}
