#[allow(dead_code)]
pub type Vertex = u32;

/*
struct Dart {
    from: Vertex,
    to: Vertex
}
*/
#[allow(dead_code)]
pub type Dart = (Vertex, Vertex); // directed edge from first to second (or use struct?)
#[allow(dead_code)]
pub type Edge = (Dart, Dart); // combination of two directed edges (darts) struct with constraints?
#[allow(dead_code)]
pub type Face = u32; // Identifier for a face.

trait GraphDCEL {
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_edges(&self) -> Vec<Edge>;
    fn get_faces(&self) -> Vec<u32>;

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
