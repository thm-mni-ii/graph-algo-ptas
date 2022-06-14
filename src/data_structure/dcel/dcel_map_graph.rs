use crate::data_structure::dcel::graph_dcel::GraphDCEL;
use crate::data_structure::graph_types::{Dart, Edge, Face, Vertex};
use crate::data_structure::map_graph::MapGraph;

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
