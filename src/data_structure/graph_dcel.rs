//! Contains the traits used to represent a doubly connected edge list

/// Trait to mark a dart type
pub trait Dart {}

/// Trait to mark a face
pub trait Face {}

/// Trait to mark a vertex
pub trait Vertex {}

/// Trait to be implemented by every vertex implementation
pub trait GraphDCEL<
    V: Vertex,
    D: Dart,
    F: Face,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
>
{
    /// Returns all vertex in the graph as an iterator
    fn get_vertexes(&self) -> VI;
    /// Returns all darts in the graph as an iterator
    fn get_darts(&self) -> DI;
    /// Returns all faces in the graph as an iterator
    fn get_faces(&self) -> FI;
    /// Returns the number of vertexes in the graph
    fn vertex_count(&self) -> usize;
    /// Returns the number of darts in the graph
    fn dart_count(&self) -> usize;
    /// Returns the number of edges in the graph
    fn edge_count(&self) -> usize;
    /// Returns the number of faces in the graph
    fn face_count(&self) -> usize;
    /// Returns the number of vertexes adjacent to the given face
    fn face_vertex_count(&self, face: &F) -> usize;
    /// Returns the amount of vertex neighboring the given vertex
    fn neighbors_count(&self, vertex: &V) -> usize;
    /// Returns a vector of neighbors of the given vertex
    fn neighbors(&self, vertex: &V) -> Vec<V>;

    /// Returns the dart between to vertexes
    fn get_dart(&self, vertex: &V, target: &V) -> Option<D>;
    /// Returns the dart linked to the given vertex
    fn dart_vertex(&self, vertex: &V) -> D;
    /// Returns the the dart linked to the given face
    fn dart_face(&self, face: &F) -> D;
    /// Returns the twin dart of the given dart
    fn twin(&self, dart: &D) -> D;
    /// Returns the target vertex of the given dart
    fn dart_target(&self, dart: &D) -> V;
    /// Return the face adjacent to the given dart
    fn face(&self, dart: &D) -> F;
    /// Returns the next dart in the dart order
    fn next(&self, current: &D) -> D;
    /// Returns the previous dart in the dart order
    fn prev(&self, current: &D) -> D;
}
