use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};
use crate::embeding::index::Embeding;

use super::phase1::Phase1;
use super::phase2::Phase2;
use super::phase3::Phase3;

#[derive(Debug)]
pub enum StackItem {
    Node(NodeIndex),
    Edge(EdgeIndex),
    Degree(usize),
}

impl StackItem {
    pub fn unwrap_node(self) -> NodeIndex {
        match self {
            StackItem::Node(node) => node,
            _ => panic!("failed to unwrap node"),
        }
    }
    pub fn unwrap_edge(self) -> EdgeIndex {
        match self {
            StackItem::Edge(edge) => edge,
            _ => panic!("failed to unwrap edge"),
        }
    }
    pub fn unwrap_degree(self) -> usize {
        match self {
            StackItem::Degree(degree) => degree,
            _ => panic!("failed to unwrap degree"),
        }
    }
}

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
    use std::fs::File;

    use petgraph::{
        dot::{Config, Dot},
        stable_graph::StableGraph,
        Undirected,
    };

    use crate::embeding::{index::Embeding, maximal_planar::index::MaximalPlanar};

    use crate::data_structure::graph_dcel::GraphDCEL;

    fn k4_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([(0, 1), (1, 2), (2, 0), (1, 3), (2, 3)])
    }

    fn other_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
        ])
    }

    fn large_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 5),
            (5, 1),
        ])
    }

    #[test]
    fn embend() {
        let graph = large_graph();
        let mut f = File::create("circle.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::embed(graph);

        dot::render(&dcel, &mut f).unwrap();
        println!("FACE COUNT: {:?}", dcel.get_faces().count());
    }
}
