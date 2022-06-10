pub type Vertex = u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::data_structure::graph_types::{Dart, Edge};

    #[test]
    fn create_dart() {
        let result = Dart::new(1, 2);

        assert_eq!(result, Dart { from: 1, to: 2 });
    }

    #[test]
    fn create_edge() {
        let d1 = Dart::new(1, 2);
        let d2 = Dart::new(2, 1);
        let result = Edge::new(d1, d2);

        assert_eq!(result, Some(Edge { d1, d2 }));
    }

    #[test]
    fn create_edge_invalid() {
        let d1 = Dart::new(10, 20);
        let d2 = Dart::new(5, 15);
        let result = Edge::new(d1, d2);

        assert_eq!(result, None);
    }

    #[test]
    fn create_edge_invalid_to() {
        let d1 = Dart::new(1, 2);
        let d2 = Dart::new(2, 5);
        let result = Edge::new(d1, d2);

        assert_eq!(result, None);
    }

    #[test]
    fn create_edge_invalid_from() {
        let d1 = Dart::new(1, 2);
        let d2 = Dart::new(5, 1);
        let result = Edge::new(d1, d2);

        assert_eq!(result, None);
    }
}
