use arboretum_td::graph::{BaseGraph, HashMapGraph, MutableGraph};
use std::collections::HashSet;

pub fn is_vertex_cover(graph: &HashMapGraph, sol: &HashSet<usize>) -> bool {
    let mut graph = graph.clone();

    for v in sol {
        graph.remove_vertex(*v);
    }

    let mut vertices = graph.vertices();

    vertices.all(|v| graph.neighborhood_set(v).is_empty())
}

pub fn brute_force_min_vertex_cover(graph: &HashMapGraph) -> HashSet<usize> {
    let n = graph.order();

    for i in 0..n + 1 {
        let mut sol = HashSet::new();
        if brute_force_min_vertex_cover_rec(graph, &mut sol, i, 0) {
            return sol;
        }
    }

    panic!("should never happen")
}

fn brute_force_min_vertex_cover_rec(
    graph: &HashMapGraph,
    sol: &mut HashSet<usize>,
    mio_size: usize,
    current_vertex: usize,
) -> bool {
    if current_vertex == graph.order() {
        if sol.len() != mio_size {
            return false;
        }

        return is_vertex_cover(graph, sol);
    }

    sol.insert(current_vertex);
    if brute_force_min_vertex_cover_rec(graph, sol, mio_size, current_vertex + 1) {
        return true;
    }

    sol.remove(&current_vertex);
    if brute_force_min_vertex_cover_rec(graph, sol, mio_size, current_vertex + 1) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{
        generation::erdos_renyi::generate_hash_map_graph,
        utils::min_vertex_cover::{brute_force_min_vertex_cover, is_vertex_cover},
    };
    use arboretum_td::graph::{HashMapGraph, MutableGraph};
    use std::collections::HashSet;

    #[test]
    fn isolated() {
        let sol = HashSet::new();

        for n in 1..10 {
            let graph = generate_hash_map_graph(n, 0.0, Some(n as u64));

            assert!(is_vertex_cover(&graph, &sol));
            assert!(is_vertex_cover(
                &graph,
                &brute_force_min_vertex_cover(&graph)
            ))
        }
    }

    #[test]
    fn single_edge() {
        let mut graph = HashMapGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_edge(0, 1);

        let mut sol = HashSet::new();

        assert!(!is_vertex_cover(&graph, &sol));

        sol.insert(0);
        assert!(is_vertex_cover(&graph, &sol));

        sol.insert(1);
        assert!(is_vertex_cover(&graph, &sol));

        assert!(is_vertex_cover(
            &graph,
            &brute_force_min_vertex_cover(&graph)
        ))
    }

    #[test]
    fn clique() {
        let mut sol = HashSet::new();
        sol.insert(0);

        for n in 2..15 {
            let graph = generate_hash_map_graph(n, 0.3, Some(n as u64));

            assert!(is_vertex_cover(
                &graph,
                &brute_force_min_vertex_cover(&graph)
            ))
        }
    }
}
