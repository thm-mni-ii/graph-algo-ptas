use crate::data_structure::graph_dcel::GraphDCEL;
use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};
use dot::{Edges, GraphWalk, Id, Labeller, Nodes};

impl<'a> GraphWalk<'a, LinkVertex, LinkDart> for LinkGraph {
    fn nodes(&'a self) -> Nodes<'a, LinkVertex> {
        self.get_vertexes().collect()
    }

    fn edges(&'a self) -> Edges<'a, LinkDart> {
        self.get_darts().collect()
    }

    fn source(&'a self, edge: &LinkDart) -> LinkVertex {
        self.dart_target(&self.twin(edge))
    }

    fn target(&'a self, edge: &LinkDart) -> LinkVertex {
        self.dart_target(edge)
    }
}

impl<'a> Labeller<'a, LinkVertex, LinkDart> for LinkGraph {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("Test").unwrap()
    }

    fn node_id(&'a self, vertex: &LinkVertex) -> Id<'a> {
        let x = vertex.get_id() as u32;
        let name = format!("N{}", x.clone());
        let id = Id::new(name);
        id.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::link_graph::LinkGraph;
    use crate::utils::single_face::generate_single_face;
    use std::borrow::Borrow;
    use std::fs::File;

    #[test]
    fn test_two_vertices() {
        let mut f = File::create("two_vertices.dot").unwrap();

        let mut lg = LinkGraph::new();
        lg.new_vertex();
        lg.new_vertex();

        dot::render(lg.borrow(), &mut f).unwrap()
    }

    #[test]
    fn circle() {
        let mut f = File::create("circle.dot").unwrap();

        let cg = generate_single_face(3);

        dot::render(cg.borrow(), &mut f).unwrap()
    }
}
