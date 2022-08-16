//! Contains the triangulate function
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::HashSet;

/// Returns the edges of a graph that need to be added to be fully triangulated.
/// The graph needs to be connected.
pub fn triangulate(
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
) -> HashSet<(LinkVertex, LinkVertex)> {
    let mut edges: HashSet<(LinkVertex, LinkVertex)> = HashSet::new();

    for face in graph.get_faces() {
        edges.extend(triangulate_face(graph, &face));
    }

    edges
}

/// Returns the edges of a face that need to be added to be fully triangulated.
fn triangulate_face(
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    face: &LinkFace,
) -> HashSet<(LinkVertex, LinkVertex)> {
    let mut edges: HashSet<(LinkVertex, LinkVertex)> = HashSet::new();

    let mut current = graph.dart_face(face);

    if graph.next(&graph.next(&current)) == current {
        // single edge (|v| < 3)
        return edges;
    } else if graph.dart_target(&graph.next(&current)) == graph.dart_target(&graph.twin(&current)) {
        // outgoing edge
        current = graph.next(&current);
    }

    let start_vertex = &graph.dart_target(&graph.twin(&current));

    loop {
        let next = graph.next(&current);

        if graph.dart_target(&graph.next(&next)) == *start_vertex {
            break;
        }

        let from = graph.dart_target(&next);

        edges.insert((from, start_vertex.clone()));
        current = next;
    }

    edges
}

#[cfg(test)]
mod tests {
    use crate::algorithm::triangulation::triangulate;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::utils::single_face::generate_single_face;

    #[test]
    fn single_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        lg.new_face(ld1.clone());
        lg.new_dart(
            lv2,
            lv1,
            Some(ld1.clone()),
            Some(ld1.clone()),
            Some(ld1),
            None,
        );

        let edges = triangulate(&lg);
        assert!(edges.is_empty())
    }

    #[test]
    fn single_face() {
        for x in 3..100 {
            let graph = generate_single_face(x);

            let edges = triangulate(&graph);

            assert_eq!(edges.len(), (x - 3) * 2)
        }
    }

    #[test]
    fn outgoing_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
        let lv4 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        let lf = lg.new_face(ld1.clone());
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(ld1.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld3 = lg.new_dart(
            lv3.clone(),
            lv1.clone(),
            Some(ld2.clone()),
            Some(ld1.clone()),
            None,
            Some(lf),
        );

        let ld4 = lg.new_dart(lv3.clone(), lv4.clone(), None, None, None, None);
        let lof = lg.new_face(ld4.clone());
        let lt4 = lg.new_dart(
            lv4.clone(),
            lv3.clone(),
            Some(ld4.clone()),
            None,
            Some(ld4.clone()),
            Some(lof.clone()),
        );
        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(lt4),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            None,
            Some(ld1),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv1.clone(),
            lv3.clone(),
            Some(lt1),
            Some(ld4),
            Some(ld3),
            Some(lof),
        );

        let edges = triangulate(&lg);
        println!("\n[RESULT]: {:?}", edges);
        assert!(!edges.is_empty());
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(lv2.clone(), lv4.clone())));
        assert!(edges.contains(&(lv1, lv4)) || edges.contains(&(lv2, lv3)));
    }

    #[test]
    fn three_edges() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(), None, None, None, None);
        let lof = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lof.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(ld1.clone()),
            None,
            None,
            Some(lof.clone()),
        );

        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(ld2.clone()),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            None,
            Some(ld1),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv1,
            lv0.clone(),
            Some(lt1),
            Some(ld0.clone()),
            Some(ld0),
            Some(lof),
        );

        let edges = triangulate(&lg);
        println!("\n[RESULT]: {:?}", edges);
        assert!(!edges.is_empty());
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(lv0.clone(), lv2.clone())) || edges.contains(&(lv2, lv0.clone())));
        assert!(edges.contains(&(lv0.clone(), lv3.clone())) || edges.contains(&(lv3, lv0)));
    }
}
