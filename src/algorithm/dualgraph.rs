use std::collections::HashSet;
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};

fn dual_graph(
    g: &impl GraphDCEL<LinkVertex, LinkDart, LinkFace,
        LinkGraphIter<LinkVertex>, LinkGraphIter<LinkDart>, LinkGraphIter<LinkFace>>,
    span: HashSet<(LinkVertex, LinkVertex)>,
) -> Vec<(LinkFace, LinkFace)> { // TODO: change return type for tree decomposition
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

            if visited.insert(next_face.clone()) &&
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
    (g.dart_target(d), g.dart_target(&g.twin(d)))
}

#[cfg(test)]
mod tests {
    use crate::algorithm::dualgraph::dual_graph;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::algorithm::spantree::span;

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

        let span = span(&lg, lv1);
        let dual = dual_graph(&lg, span);

        println!("[RESULT]: {:?}", dual);
        assert_eq!(dual.len(), 1);
        assert_eq!(dual, vec![(lf.clone(), lf)]); // FIXME: dual graph should contain at least the root (outer face)
    }

    #[test]
    fn triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(),
                              None, None, None, None);
        let lf = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(),
                              Some(ld0.clone()), None,
                              None, Some(lf.clone()));
        let ld2 = lg.new_dart(lv2.clone(), lv0.clone(),
                              Some(ld1.clone()), Some(ld0.clone()),
                              None, Some(lf.clone()));

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(),
                              None, None,
                              Some(ld0), None);
        let lof = lg.new_face(lt0.clone());
        let lt2 = lg.new_dart(lv0, lv2.clone(),
                              Some(lt0.clone()), None,
                              Some(ld2), Some(lof.clone()));
        lg.new_dart(lv2, lv1.clone(),
                    Some(lt2), Some(lt0),
                    Some(ld1), Some(lof.clone()));

        let span = span(&lg, lv1);
        let dual = dual_graph(&lg, span);

        println!("[RESULT]: {:?}", dual);
        assert_eq!(dual.len(), 1);
        assert_eq!(dual, vec![(lf, lof)]);
    }
}