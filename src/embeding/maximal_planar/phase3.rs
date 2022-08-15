use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::graph_dcel::GraphDCEL;
use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkVertex};

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
            let new = self.get_or_create_vertex(v);
            let hs = self.pop_edges_to_vextex(hc, new.clone());
            let mut loop_dart: Option<LinkDart> = None;

            for e in es {
                let (a_node, b_node) = e;
                let vertex = self.node_id_mapper.get(&b_node).unwrap().clone();
                let vertex2 = self.node_id_mapper.get(&a_node).unwrap().clone();
                let dart = self.dcel.get_dart(&vertex, &vertex2);

                loop_dart = Some(self.dcel.next(&dart.clone().unwrap()));

                self.dcel.remove_edge(&vertex, dart.unwrap());
            }

            let mut dart = loop_dart.unwrap_or_else(|| self.get_dart_in_face(hs.clone()));
            let last_dart = self.dcel.prev(&dart.clone());
            let mut last_twin: Option<LinkDart> = None;
            let mut first_twin: Option<LinkDart> = None;

            while {
                let target = self.dcel.dart_target(&dart.clone());
                let source = self.dcel.dart_target(&self.dcel.twin(&dart.clone()));
                let next = self.dcel.next(&dart.clone());
                let is_last = last_dart == dart;
                let t1 = if is_last { first_twin.clone() } else { None };
                dart.change_face(None, None, None);

                let (d0, d1, f) =
                    self.create_face(target, new.clone(), source, dart.clone(), t1, last_twin);

                last_twin = Some(d0.clone());
                if first_twin.is_none() {
                    first_twin = Some(d1.clone());
                }

                dart.change_face(Some(f), Some(d1), Some(d0));
                dart = next;

                !is_last
            } {}
        }
    }

    fn pop_edges(&mut self, count: i32) -> Vec<(NodeIndex, NodeIndex)> {
        (0..count)
            .map(|_| self.stack.pop().unwrap().unwrap_edge())
            .collect::<Vec<_>>()
    }

    fn pop_edges_to_vextex(&mut self, count: i32, new: LinkVertex) -> Vec<LinkVertex> {
        self.pop_edges(count)
            .iter()
            .map(|e| -> LinkVertex {
                let (a_node, b_node) = e;
                self.get_outer_vertex(*a_node, *b_node, new.clone())
            })
            .collect::<Vec<LinkVertex>>()
    }

    fn get_or_create_vertex(&mut self, key: NodeIndex) -> LinkVertex {
        self.node_id_mapper
            .entry(key)
            .or_insert_with(|| self.dcel.new_vertex())
            .clone()
    }

    fn get_outer_vertex(
        &mut self,
        a_node: NodeIndex,
        b_node: NodeIndex,
        new: LinkVertex,
    ) -> LinkVertex {
        let a_vertex = self.get_or_create_vertex(a_node);
        let b_vertex = self.get_or_create_vertex(b_node);

        if a_vertex == new {
            return b_vertex;
        };

        a_vertex
    }

    fn get_dart_in_face(&self, hs: Vec<LinkVertex>) -> LinkDart {
        let dart = self.dcel.get_dart(&hs[0], &hs[1]).unwrap();
        let target = self.dcel.dart_target(&dart);
        let next_target = self.dcel.dart_target(&self.dcel.next(&dart));

        if hs.contains(&target) && hs.contains(&next_target) {
            return dart;
        }

        self.dcel.twin(&dart)
    }

    fn create_face(
        &mut self,
        v0: LinkVertex,
        v1: LinkVertex,
        v2: LinkVertex,
        d3: LinkDart,
        t1: Option<LinkDart>,
        t2: Option<LinkDart>,
    ) -> (LinkDart, LinkDart, LinkFace) {
        let d0 = self
            .dcel
            .new_dart(v0, v1.clone(), Some(d3.clone()), None, t1, None);
        let f0 = self.dcel.new_face(d0.clone());
        let d1 = self
            .dcel
            .new_dart(v1, v2, Some(d0.clone()), Some(d3), t2, Some(f0.clone()));

        (d0, d1, f0)
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
