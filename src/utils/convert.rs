use arboretum_td::graph::{HashMapGraph, MutableGraph};
use petgraph::{stable_graph::StableGraph, visit::EdgeRef, Undirected};

pub type UndirectedGraph = StableGraph<(), (), Undirected>;

pub fn petgraph_to_hash_map_graph(petgraph: &UndirectedGraph) -> HashMapGraph {
    let mut hash_map_graph = HashMapGraph::new();

    for v in petgraph.node_indices() {
        hash_map_graph.add_vertex(v.index());
    }

    for v in petgraph.node_indices() {
        for e in petgraph.edges(v) {
            hash_map_graph.add_edge(e.source().index(), e.target().index());
        }
    }

    hash_map_graph
}

#[cfg(test)]
mod tests {
    use crate::utils::convert::{petgraph_to_hash_map_graph, UndirectedGraph};
    use arboretum_td::graph::BaseGraph;

    #[test]
    fn isolated() {
        let mut petgraph = UndirectedGraph::default();
        let u = petgraph.add_node(());
        let v = petgraph.add_node(());
        let w = petgraph.add_node(());

        let hash_map_graph = petgraph_to_hash_map_graph(&petgraph);
        assert!(hash_map_graph.order() == petgraph.node_count());
        assert!(hash_map_graph.has_vertex(u.index()));
        assert!(hash_map_graph.has_vertex(v.index()));
        assert!(hash_map_graph.has_vertex(w.index()));
    }

    #[test]
    fn single_edge() {
        let mut petgraph = UndirectedGraph::default();
        let u = petgraph.add_node(());
        let v = petgraph.add_node(());
        petgraph.add_edge(u, v, ());

        let hash_map_graph = petgraph_to_hash_map_graph(&petgraph);
        assert!(hash_map_graph.order() == petgraph.node_count());
        assert!(hash_map_graph.has_edge(u.index(), v.index()));
    }

    #[test]
    fn triangle() {
        let mut petgraph = UndirectedGraph::default();
        let u = petgraph.add_node(());
        let v = petgraph.add_node(());
        let w = petgraph.add_node(());
        petgraph.add_edge(u, v, ());
        petgraph.add_edge(v, w, ());
        petgraph.add_edge(w, u, ());

        let hash_map_graph = petgraph_to_hash_map_graph(&petgraph);
        assert!(hash_map_graph.order() == petgraph.node_count());
        assert!(hash_map_graph.has_edge(u.index(), v.index()));
        assert!(hash_map_graph.has_edge(v.index(), w.index()));
        assert!(hash_map_graph.has_edge(w.index(), u.index()));
    }

    #[test]
    fn square() {
        let mut petgraph = UndirectedGraph::default();
        let t = petgraph.add_node(());
        let u = petgraph.add_node(());
        let v = petgraph.add_node(());
        let w = petgraph.add_node(());
        petgraph.add_edge(t, u, ());
        petgraph.add_edge(u, v, ());
        petgraph.add_edge(v, w, ());
        petgraph.add_edge(w, t, ());

        let hash_map_graph = petgraph_to_hash_map_graph(&petgraph);
        assert!(hash_map_graph.order() == petgraph.node_count());
        assert!(hash_map_graph.has_edge(t.index(), u.index()));
        assert!(hash_map_graph.has_edge(u.index(), v.index()));
        assert!(hash_map_graph.has_edge(v.index(), w.index()));
        assert!(hash_map_graph.has_edge(w.index(), t.index()));
        assert!(!hash_map_graph.has_edge(t.index(), v.index()));
        assert!(!hash_map_graph.has_edge(u.index(), w.index()));
    }
}
