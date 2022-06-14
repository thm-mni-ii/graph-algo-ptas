use crate::data_structure::graph_types::{Dart, Edge, Face, Vertex};

// Subgraphen erstellen --> Label für Vertices bzw. Übersetzungstabelle
// Subgraphen: Eingabe ist Menge von Knoten. Ausgabe sind
// Menge von Knoten, Labels und verbindende Kanten

// NetworkX Planarity

// Kanten markieren.

trait GraphDCEL {
    fn get_vertices(&self) -> &[Vertex];
    fn get_edges(&self) -> &[Edge];
    fn get_faces(&self) -> &[Face];

    fn dart_vertex(&self, vertex: Vertex) -> Dart;
    fn dart_face(&self, face: Face) -> Dart;
    fn twin(&self, dart: Dart) -> Dart;
    fn target(&self, dart: Dart) -> Vertex;
    fn face(&self, dart: Dart) -> Face;
    fn next(&self, current: Dart) -> Dart;
    fn prev(&self, current: Dart) -> Dart;
}

trait Graph {
    fn new() -> Self;
    fn contains_vertex(&self, v: Vertex) -> bool;
    fn contains_edge(&self, e: Edge) -> bool;
    fn is_adjacent(&self, v: Vertex, u: Vertex) -> bool;
    fn add_vertex(&mut self, v: Vertex);
    fn rem_vertex(&mut self, v: Vertex);
    fn add_edge(&mut self, e: Edge);
    fn rem_edge(&mut self, e: Edge);
    fn get_neighbours(&self, v: Vertex) -> Vec<Vertex>;
}
