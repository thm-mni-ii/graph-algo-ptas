//! Contains the implementation of the maximal planar embedding algorithm

use super::phase1::Phase1;
use super::phase2::Phase2;
use super::phase3::Phase3;
use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};
use crate::embedding::index::Embedding;
use crate::utils::convert::UndirectedGraph;

/// Contains the implementation of the maximal planar embedding algorithm
pub struct MaximalPlanar {}

impl
    Embedding<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
        LinkGraph,
    > for MaximalPlanar
{
    fn embed(mut graph: UndirectedGraph) -> LinkGraph {
        let graph_copy = graph.clone();
        let mut stack = Vec::new();
        let mut dcel = LinkGraph::new();
        let node_count = graph.node_count();

        if node_count < 3 {
            panic!("For embedding, a graph with at least 3 nodes is required")
        }

        if node_count == 3 {
            Phase2::new(&mut dcel).triangle_embedding();
            return dcel;
        }

        Phase1::new(&mut graph, &mut stack).execute();
        Phase2::new(&mut dcel).execute();
        Phase3::new(graph, graph_copy, &mut stack, &mut dcel).execute();

        dcel
    }
}

#[cfg(test)]
mod tests {
    use petgraph::stable_graph::StableGraph;

    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::{
        embedding::{index::Embedding, maximal_planar::index::MaximalPlanar},
        generation::planar::generate,
        utils::convert::UndirectedGraph,
    };

    fn test_embed(graph: UndirectedGraph) {
        let dcel = MaximalPlanar::embed(graph.clone());

        dcel.validate();
        assert_eq!(
            dcel.vertex_count(),
            graph.node_count(),
            "Invalid Vertex count. Expected {} got {}",
            graph.node_count(),
            dcel.vertex_count()
        );
        assert_eq!(
            dcel.edge_count(),
            graph.edge_count(),
            "Invalid Edge count. Expected {} got {}",
            graph.edge_count(),
            dcel.edge_count(),
        );
    }

    #[test]
    #[should_panic]
    fn embedd_to_small() {
        let graph: UndirectedGraph = StableGraph::from_edges(&[(0, 1)]);
        MaximalPlanar::embed(graph);
    }

    #[test]
    fn embedd_triangle_graph() {
        let graph: UndirectedGraph = StableGraph::from_edges(&[(0, 1), (0, 2), (1, 2)]);
        test_embed(graph);
    }

    #[test]
    fn embed_k4_graph() {
        let graph = generate(4, Some(0)).to_pet_graph();
        test_embed(graph)
    }

    #[test]
    fn embed_min_graph() {
        let graph = generate(5, Some(0)).to_pet_graph();
        test_embed(graph)
    }

    #[test]
    fn embed_small_graph() {
        let graph = generate(10, Some(0)).to_pet_graph();
        test_embed(graph)
    }

    #[test]
    fn embed_medium_graph() {
        let graph = generate(50, Some(0)).to_pet_graph();
        test_embed(graph)
    }

    #[test]
    fn embed_large_graph() {
        let graph = generate(100, Some(0)).to_pet_graph();
        test_embed(graph)
    }
}
