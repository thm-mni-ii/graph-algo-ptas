use std::collections::HashSet;
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};

fn dual_graph(
    g: &impl GraphDCEL<LinkVertex, LinkDart, LinkFace,
        LinkGraphIter<LinkVertex>, LinkGraphIter<LinkDart>, LinkGraphIter<LinkFace>>,
    span: HashSet<(LinkVertex, LinkVertex)>) -> Vec<(LinkFace, LinkFace)> {
    let mut result = vec![];
    let mut visited = HashSet::new();
    for face in g.get_faces() {
        visited.insert(face.clone());
        let first = g.dart_face(&face);
        let mut current_dart = first.clone();
        loop {
            let next_dart = g.next(&current_dart);
            if first == next_dart {
                break;
            }
            let next_face = g.face(&g.twin(&current_dart));

            if !visited.contains(&next_face) &&
                (span.contains(&dart_as_tuple(g, &current_dart))
                    || span.contains(&dart_as_tuple(g, &g.twin(&current_dart)))) {
                result.push((face.clone(), next_face.clone()));
            }
            current_dart = next_dart;
        }
    }
    result
}

fn dart_as_tuple(
    g: &impl GraphDCEL<LinkVertex, LinkDart, LinkFace,
        LinkGraphIter<LinkVertex>, LinkGraphIter<LinkDart>, LinkGraphIter<LinkFace>>,
    d: &LinkDart,
) -> (LinkVertex, LinkVertex) {
    (g.dart_target(&d), g.dart_target(&g.twin(&d)))
}