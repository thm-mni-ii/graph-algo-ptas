use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::graph_dcel::GraphDCEL;
use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};

use super::stack_item::StackItem;

pub struct Phase3<'a> {
    graph: StableGraph<u32, (), Undirected>,
    graph_copy: StableGraph<u32, (), Undirected>,
    stack: &'a mut Vec<StackItem>,
    dcel: &'a mut LinkGraph,
    node_id_mapper: HashMap<NodeIndex, LinkVertex>,
}

impl Phase3<'_> {
    pub fn new<'a>(
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

    pub fn execute(&mut self) {
        let mut first_dart: Option<LinkDart> = None;
        let mut last_dart: Option<LinkDart> = None;
        while let Some(entry) = self.stack.pop() {
            let k = entry.unwrap_degree();
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
                let (_, b_node) = e;
                let b_vertex = self.node_id_mapper.get(&b_node).unwrap().clone();
                self.dcel
                    .remove_edge(&b_vertex, self.dcel.dart_vertex(&b_vertex));
            }

            self.get_or_create_vertex(v);

            for h in hs {
                let (a_node, b_node) = h;
                let a_vertex = self.get_or_create_vertex(a_node);
                let b_vertex = self.get_or_create_vertex(b_node);

                let (new_dart, new_twin) = self.dcel.new_edge(
                    a_vertex.clone(),
                    b_vertex.clone(),
                    last_dart,
                    first_dart,
                    None,
                    None,
                );

                self.dcel.new_face(new_dart.clone());
                self.dcel.new_face(new_twin.clone());

                last_dart = Some(new_dart.clone());
                first_dart = Some(new_dart);
            }
        }
    }

    fn pop_edges(&mut self, count: i32) -> Vec<(NodeIndex, NodeIndex)> {
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
