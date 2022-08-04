use std::collections::HashMap;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::graph_dcel::GraphDCEL;
use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};
use crate::embeding::index::Embeding;

use super::phase1::Phase1;
use super::phase2::Phase2;

#[derive(Debug)]
pub enum StackItem {
    Node(NodeIndex),
    Edge(EdgeIndex),
    Degree(usize),
}

impl StackItem {
    fn unwrap_node(self) -> NodeIndex {
        match self {
            StackItem::Node(node) => node,
            _ => panic!("failed to unwrap node"),
        }
    }
    fn unwrap_edge(self) -> EdgeIndex {
        match self {
            StackItem::Edge(edge) => edge,
            _ => panic!("failed to unwrap edge"),
        }
    }
    fn unwrap_degree(self) -> usize {
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

struct Phase3<'a> {
    graph: StableGraph<u32, (), Undirected>,
    graph_copy: StableGraph<u32, (), Undirected>,
    stack: &'a mut Vec<StackItem>,
    dcel: &'a mut LinkGraph,
    node_id_mapper: HashMap<NodeIndex, LinkVertex>,
}

impl Phase3<'_> {
    fn new<'a>(
        graph: StableGraph<u32, (), Undirected>,
        graph_copy: StableGraph<u32, (), Undirected>,
        stack: &'a mut Vec<StackItem>,
        dcel: &'a mut LinkGraph,
    ) -> Phase3<'a> {
        let node_id_mapper = graph
            .node_indices()
            .zip(dcel.get_vertexes())
            .collect::<HashMap<NodeIndex, LinkVertex>>();

        Phase3 {
            graph,
            graph_copy,
            stack,
            dcel,
            node_id_mapper,
        }
    }

    fn execute(&mut self) {
        let mut last_dart: Option<LinkDart> = None;
        while let Some(entry) = self.stack.pop() {
            let k = entry.unwrap_degree();
            let mut create_face = false;
            let (ec, hc) = match k {
                3 => (0, 3),
                4 => (1, 4),
                5 => (2, 5),
                degree => {
                    println!("unexpected degree {:?}", degree);
                    continue;
                }
            };
            let es = self.pop_edges(ec);
            let v = self.stack.pop().unwrap().unwrap_node();
            let hs = self.pop_edges(hc);

            for e in es {
                let (a_node, _) = self.graph_copy.edge_endpoints(e).unwrap();
                let a_vertex = self.node_id_mapper.get(&a_node).unwrap().clone();
                self.dcel
                    .remove_edge(&a_vertex, self.dcel.dart_vertex(&a_vertex));
            }

            self.get_or_create_vertex(v);

            for h in hs {
                let (a_node, b_node) = self.graph_copy.edge_endpoints(h).unwrap();
                let a_vertex = self.node_id_mapper.get(&a_node).unwrap().clone();
                let b_vertex = self.get_or_create_vertex(b_node);

                let (new_dart, _) =
                    self.dcel
                        .new_edge(a_vertex.clone(), b_vertex.clone(), last_dart, None, None);

                if create_face {
                    self.dcel.new_face(new_dart.clone());
                } else {
                    create_face = true;
                }

                last_dart = Some(new_dart);
            }
        }
    }

    fn pop_edges(&mut self, count: i32) -> Vec<EdgeIndex> {
        (0..count)
            .map(|_| self.stack.pop().unwrap().unwrap_edge())
            .collect::<Vec<_>>()
    }

    fn get_or_create_vertex(&mut self, key: NodeIndex) -> LinkVertex {
        self.node_id_mapper
            .entry(key)
            .or_insert_with(|| self.dcel.new_vertex())
            .clone()
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
    fn phase_3() {
        let graph = large_graph();
        let mut f = File::create("circle.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::embed(graph);

        dot::render(&dcel, &mut f).unwrap();
        println!("FACE COUNT: {:?}", dcel.get_faces().count());
    }
}
