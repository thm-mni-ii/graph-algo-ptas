pub trait Dart {}

pub trait Face {}

pub trait Vertex {}

pub trait GraphDCEL<
    V: Vertex,
    D: Dart,
    F: Face,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
>
{
    fn get_vertexes(&self) -> VI;
    fn get_darts(&self) -> DI;
    fn get_faces(&self) -> FI;
    fn vertex_count(&self) -> usize;
    fn dart_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn face_count(&self) -> usize;
    fn face_vertex_count(&self, face: &F) -> usize;
    fn neighbors_count(&self, vertex: &V) -> usize;
    fn neighbors(&self, vertex: &V) -> Vec<V>;

    fn dart_vertex(&self, vertex: &V) -> D;
    fn dart_face(&self, face: &F) -> D;
    fn twin(&self, dart: &D) -> D;
    fn dart_target(&self, dart: &D) -> V;
    fn face(&self, dart: &D) -> F;
    fn next(&self, current: &D) -> D;
    fn prev(&self, current: &D) -> D;
}
