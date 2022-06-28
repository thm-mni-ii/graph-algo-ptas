pub trait Dart {}

pub trait Face {}

pub trait Vertex {}

pub trait GraphDCEL<V: Vertex, D: Dart, F: Face, VI: Iterator<Item=V>, DI: Iterator<Item=D>, FI: Iterator<Item=F>> {
    fn get_vertexes(&self) -> VI;
    fn get_darts(&self) -> DI;
    fn get_faces(&self) -> FI;
    
    fn dart_vertex(&self, vertex: V) -> D;
    fn dart_face(&self, face: F) -> D;
    fn twin(&self, dart: D) -> D;
    fn target(&self, dart: D) -> V;
    fn face(&self, dart: D) -> F;
    fn next(&self, current: D) -> D;
    fn prev(&self, current: D) -> D;
}
