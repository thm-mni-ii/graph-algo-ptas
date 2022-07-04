use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};
use dot::{Edges, GraphWalk, Nodes};
use std::borrow::Cow;

impl<'a> GraphWalk<'a, LinkVertex, LinkDart> for LinkGraph {
    fn nodes(&'a self) -> Nodes<'a, LinkVertex> {
        Cow::Borrowed(self.get_vertices())
    }

    fn edges(&'a self) -> Edges<'a, LinkDart> {
        Cow::Borrowed(self.get_darts())
    }

    fn source(&'a self, edge: &LinkDart) -> LinkVertex {
        &edge
    }

    fn target(&'a self, edge: &LinkDart) -> LinkVertex {
        todo!()
    }
}
