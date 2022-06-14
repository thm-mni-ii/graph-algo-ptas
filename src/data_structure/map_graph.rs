use crate::data_structure::graph::Graph;
use crate::data_structure::graph_types::{Edge, Vertex};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct MapGraph {
    vertices: HashMap<Vertex, Vec<Vertex>>,
}

#[allow(unused)]
impl Graph for MapGraph {
    fn new() -> Self {
        todo!()
    }

    fn get_vertices(&self) -> &[Vertex] {
        todo!()
    }

    fn get_edges(&self) -> &[Edge] {
        todo!()
    }

    fn contains_vertex(&self, v: Vertex) -> bool {
        todo!()
    }

    fn contains_edge(&self, e: Edge) -> bool {
        todo!()
    }

    fn is_adjacent(&self, v: Vertex, u: Vertex) -> bool {
        todo!()
    }

    fn add_vertex(&mut self, v: Vertex) {
        todo!()
    }

    fn rem_vertex(&mut self, v: Vertex) {
        todo!()
    }

    fn add_edge(&mut self, e: Edge) {
        todo!()
    }

    fn rem_edge(&mut self, e: Edge) {
        todo!()
    }

    fn get_neighbours(&self, v: Vertex) -> Vec<Vertex> {
        todo!()
    }
}
