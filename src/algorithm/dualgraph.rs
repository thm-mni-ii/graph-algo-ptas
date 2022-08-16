//! Contains the dual_graph function
use crate::algorithm::spantree::Span;
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashMap, HashSet};

/// Returns the dual graph that doesn't cross the edges of the span (face tree)
pub fn dual_graph(
    g: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    span: &Span<LinkVertex>,
) -> HashMap<LinkFace, HashSet<LinkFace>> {
    let mut result = HashMap::new();
    let mut visited = HashSet::new();
    if g.get_vertexes().count() <= 2 {
        let outer_face = g.get_faces().next();
        if let Some(face) = outer_face {
            result.insert(face, Default::default());
        }
        return result;
    }
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

            if visited.insert(next_face.clone())
                && (span.upwards.get(&g.dart_target(&current_dart))
                    == Some(&g.dart_target(&g.twin(&current_dart)))
                    || span.upwards.get(&g.dart_target(&g.twin(&current_dart)))
                        == Some(&g.dart_target(&current_dart)))
            {
                match result.get_mut(&face) {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(next_face.clone());
                        result.insert(face.clone(), set);
                    }
                    Some(set) => {
                        set.insert(next_face.clone());
                    }
                }
            }
            current_dart = next_dart;
        }
    }
    result
}

#[allow(dead_code)]
fn dart_as_tuple(
    g: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    d: &LinkDart,
) -> (LinkVertex, LinkVertex) {
    (g.dart_target(d), g.dart_target(&g.twin(d)))
}

#[cfg(test)]
mod tests {
    use crate::algorithm::dualgraph::dual_graph;
    use crate::algorithm::spantree::Span;
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
    use crate::utils::convert::UndirectedGraph;
    use petgraph::stable_graph::StableGraph;
    use std::collections::HashSet;

    #[test]
    fn single_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        let lf = lg.new_face(ld1.clone());
        lg.new_dart(
            lv2,
            lv1.clone(),
            Some(ld1.clone()),
            Some(ld1.clone()),
            Some(ld1),
            Some(lf.clone()),
        );

        let span = Span::compute(&lg, lv1);
        let dual = dual_graph(&lg, &span);

        println!("[RESULT]: {:?}", dual);
        assert_eq!(dual.len(), 1);
        assert_eq!(dual.get(&lf), Some(&HashSet::new()));
    }

    #[test]
    fn triangle() {
        let sg: UndirectedGraph = StableGraph::from_edges(&[(0, 1), (1, 2), (2, 0)]);

        let lg = MaximalPlanar::embed(sg);
        assert_eq!(lg.vertex_count(), 3);
        let lv0 = lg.vertex_by_id(0).unwrap();
        let lv1 = lg.vertex_by_id(1).unwrap();
        let ld1 = lg.get_dart(&lv1, &lv0).unwrap();
        let lf = lg.face(&ld1);
        let lof = lg.face(&lg.twin(&ld1));

        let span = Span::compute(&lg, lv1);
        let dual = dual_graph(&lg, &span);

        println!("[RESULT]: {:?}", dual);
        assert_eq!(dual.len(), 1);
        assert!(dual.get(&lof).unwrap_or(&HashSet::new()).contains(&lf));
    }
}
