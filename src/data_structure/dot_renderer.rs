use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};
use dot::{Edges, GraphWalk, Id, Labeller, Nodes};
use crate::data_structure::graph_dcel::{GraphDCEL};

trait Named {
    fn get_name() -> String;
}

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
    use std::borrow::Borrow;
    use crate::data_structure::{
        link_graph::{LinkGraph},
    };
    use std::fs::File;
    use crate::utils::circle::generate_circle;


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

        let cg = generate_circle(3);

        dot::render(cg.borrow(), &mut f).unwrap()
    }
}