use arboretum_td::graph::{BaseGraph, HashMapGraph};
use std::collections::HashSet;

pub fn is_independent_set(graph: &HashMapGraph, sol: &HashSet<usize>) -> bool {
    let sol: Vec<usize> = sol.iter().copied().collect();

    for i in 0..sol.len() {
        let u = sol[i];

        if !graph.has_vertex(u) {
            continue;
        }

        #[allow(clippy::needless_range_loop)]
        for j in i + 1..sol.len() {
            let v = sol[j];

            if !graph.has_vertex(v) {
                continue;
            }

            if graph.has_edge(u, v) {
                return false;
            }
        }
    }

    true
}

pub fn brute_force_max_independent_set(graph: &HashMapGraph) -> HashSet<usize> {
    let n = graph.order();

    for i in 0..n + 1 {
        let mut sol = HashSet::new();
        if brute_force_max_independent_set_rec(graph, &mut sol, n - i, 0) {
            return sol;
        }
    }

    panic!("should never happen")
}

fn brute_force_max_independent_set_rec(
    graph: &HashMapGraph,
    sol: &mut HashSet<usize>,
    mio_size: usize,
    current_vertex: usize,
) -> bool {
    if current_vertex == graph.order() {
        if sol.len() != mio_size {
            return false;
        }

        return is_independent_set(graph, sol);
    }

    sol.insert(current_vertex);
    if brute_force_max_independent_set_rec(graph, sol, mio_size, current_vertex + 1) {
        return true;
    }

    sol.remove(&current_vertex);
    if brute_force_max_independent_set_rec(graph, sol, mio_size, current_vertex + 1) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        max_independent_set::{brute_force_max_independent_set, is_independent_set},
        random_graph::random_hashmap_graph,
    };
    use arboretum_td::graph::{HashMapGraph, MutableGraph};
    use std::collections::HashSet;

    #[test]
    fn empty_set() {
        let sol = HashSet::new();

        for n in 1..10 {
            let graph = random_hashmap_graph(n, 0.3, Some(n as u64));

            assert!(is_independent_set(&graph, &sol));
            assert!(is_independent_set(
                &graph,
                &brute_force_max_independent_set(&graph)
            ))
        }
    }

    #[test]
    fn isolated() {
        let mut sol = HashSet::new();

        for n in 1..10 {
            let graph = random_hashmap_graph(n, 0., Some(n as u64));
            sol.insert(n - 1);

            assert!(is_independent_set(&graph, &sol));
            assert!(is_independent_set(
                &graph,
                &brute_force_max_independent_set(&graph)
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
        sol.insert(0);
        assert!(is_independent_set(&graph, &sol));

        sol.insert(1);
        assert!(!is_independent_set(&graph, &sol));

        assert!(is_independent_set(
            &graph,
            &brute_force_max_independent_set(&graph)
        ))
    }

    #[test]
    fn clique() {
        let mut sol = HashSet::new();
        sol.insert(0);

        for n in 2..15 {
            let graph = random_hashmap_graph(n, 1., Some(n as u64));

            let v = n - 1;

            sol.insert(v);
            assert!(!is_independent_set(&graph, &sol));

            sol.remove(&v);
            assert!(is_independent_set(&graph, &sol));

            assert!(is_independent_set(
                &graph,
                &brute_force_max_independent_set(&graph)
            ))
        }
    }
}
