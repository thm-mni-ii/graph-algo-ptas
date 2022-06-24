use std::collections::HashSet;
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    graph_types::{Face, Vertex, Dart},
};

fn dual_graph(g: &impl GraphDCEL, span: HashSet<(Vertex, Vertex)>) -> Vec<(Face, Face)> {
    let mut result = vec![];
    let mut visited = HashSet::new();
    for face in g.get_faces() {
        visited.insert(face);
        let first = g.dart_face(*face);
        let mut current_dart = first;
        loop {
            let next_dart = g.next(current_dart);
            if first == next_dart {
                break;
            }
            let next_face = g.face(g.twin(current_dart));

            if !visited.contains(&next_face) &&
                (span.contains(&dart_as_tuple(g, current_dart))
                    || span.contains(&dart_as_tuple(g, g.twin(current_dart)))) {
                result.push((*face, next_face));
            }
            current_dart = next_dart;
        }
    }
    result
}

fn dart_as_tuple(g: &impl GraphDCEL, d: Dart) -> (Vertex, Vertex) {
    (g.target(d), g.target(g.twin(d)))
}