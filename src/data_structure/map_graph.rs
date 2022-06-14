use std::collections::HashMap;
use crate::data_structure::graph_dcel::{Graph, GraphDCEL};
use crate::data_structure::graph_types::{Dart, Edge, Face, Vertex};

#[allow(dead_code)]
struct MapGraph {
    vertices: HashMap<Vertex, Vec<Vertex>>
}

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

impl GraphDCEL for MapGraph {
    fn get_vertices(&self) -> &[Vertex] {
        todo!()
    }

    fn get_edges(&self) -> &[Edge] {
        todo!()
    }

    fn get_faces(&self) -> &[Face] {
        todo!()
    }

    fn dart_vertex(&self, vertex: Vertex) -> Dart {
        todo!()
    }

    fn dart_face(&self, face: Face) -> Dart {
        todo!()
    }

    fn source(&self, dart: Dart) -> Dart {
        todo!()
    }

    fn twin(&self, dart: Dart) -> Dart {
        todo!()
    }

    fn target(&self, dart: Dart) -> Vertex {
        todo!()
    }

    fn face(&self, dart: Dart) -> Face {
        todo!()
    }

    fn next(&self, current: Dart) -> Dart {
        todo!()
    }

    fn prev(&self, current: Dart) -> Dart {
        todo!()
    }
}