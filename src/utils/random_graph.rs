use arboretum_td::graph::{HashMapGraph, MutableGraph};
use rand::Rng;

pub fn random_hashmap_graph<R: Rng>(n: usize, p: f64, rng: &mut R) -> HashMapGraph {
    let mut graph = HashMapGraph::new();

    for v in 0..n {
        graph.add_vertex(v);
    }

    for a in 0..n {
        for b in 0..n {
            if a != b && rng.gen_bool(p) {
                graph.add_edge(a, b);
            }
        }
    }

    graph
}
