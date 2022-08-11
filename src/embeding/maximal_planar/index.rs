use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};
use crate::embeding::index::Embeding;

use super::phase1::Phase1;
use super::phase2::Phase2;
use super::phase3::Phase3;

pub struct MaximalPlanar {}

impl
    Embeding<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
        LinkGraph,
    > for MaximalPlanar
{
    fn embed(mut graph: StableGraph<u32, (), Undirected>) -> LinkGraph {
        let graph_copy = graph.clone();
        let mut stack = Vec::new();
        let mut dcel = LinkGraph::new();

        Phase1::new(&mut graph, &mut stack).execute();
        Phase2::new(&mut dcel).execute();
        Phase3::new(graph, graph_copy, &mut stack, &mut dcel).execute();

        dcel
    }
}

#[cfg(test)]
mod tests {
    use petgraph::{stable_graph::StableGraph, Undirected};

    use crate::{
        embeding::{index::Embeding, maximal_planar::index::MaximalPlanar},
        generation::planar::generate,
    };

    use crate::data_structure::graph_dcel::GraphDCEL;

    fn test_embend(graph: StableGraph<u32, (), Undirected>) {
        let dcel = MaximalPlanar::embed(graph.clone());

        assert_eq!(dcel.vertex_count(), graph.node_count());
        assert_eq!(dcel.edge_count(), graph.edge_count())
    }

    #[test]
    fn embend_k4_graph() {
        let graph = generate(4, Some(0)).to_pet_graph();
        test_embend(graph)
    }

    #[test]
    fn embend_min_graph() {
        let graph = generate(5, Some(0)).to_pet_graph();
        test_embend(graph)
    }

    #[test]
    fn embend_small_graph() {
        let graph = generate(10, Some(0)).to_pet_graph();
        test_embend(graph)
    }

    #[test]
    fn embend_medium_graph() {
        let graph = generate(50, Some(0)).to_pet_graph();
        test_embend(graph)
    }

    #[test]
    fn embend_large_graph() {
        let graph = generate(100, Some(0)).to_pet_graph();
        test_embend(graph)
    }
}
