// Subgraphen erstellen --> Label für Vertices bzw. Übersetzungstabelle
// Subgraphen: Eingabe ist Menge von Knoten. Ausgabe sind
// Menge von Knoten, Labels und verbindende Kanten

// NetworkX Planarity

// Kanten markieren.

use crate::data_structure::graph_types::{Dart, Edge, Face, Vertex};

pub trait GraphDCEL {
    fn get_vertices(&self) -> &[Vertex];
    fn get_edges(&self) -> &[Edge];
    fn get_faces(&self) -> &[Face];

    fn dart_vertex(&self, vertex: Vertex) -> Dart;
    fn dart_face(&self, face: Face) -> Dart;

    fn source(&self, dart: Dart) -> Dart;
    fn twin(&self, dart: Dart) -> Dart;
    fn target(&self, dart: Dart) -> Vertex;
    fn face(&self, dart: Dart) -> Face;
    fn next(&self, current: Dart) -> Dart;
    fn prev(&self, current: Dart) -> Dart;
}
