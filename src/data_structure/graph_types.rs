#[allow(dead_code)]
pub type Vertex = u32;

#[allow(dead_code)]
pub struct Dart {
    from: Vertex,
    to: Vertex,
}

#[allow(dead_code)]
impl Dart {
    pub fn new(from: Vertex, to: Vertex) -> Dart {
        Dart { from, to }
    }
}

#[allow(dead_code)]
pub struct Edge {
    d1: Dart,
    d2: Dart,
}

#[allow(dead_code)]
impl Edge {
    pub fn new(d1: Dart, d2: Dart) -> Option<Edge> {
        if d1.to != d2.from || d2.to != d1.from {
            None
        } else {
            Some(Edge { d1, d2 })
        }
    }
}

#[allow(dead_code)]
pub type Face = u32; // Identifier for a face.
