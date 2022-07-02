use crate::data_structure::{
    graph_dcel::GraphDCEL,
    graph_types::{Dart, Face, Vertex},
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};

pub fn triangulate(
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
) -> Vec<(LinkVertex, LinkVertex)> {
    let mut edges: Vec<(LinkVertex, LinkVertex)> = vec![];

    for face in graph.get_faces() {
        edges.append(&mut triangulate_face(graph, &face));
    }

    edges
}

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
) -> Vec<(LinkVertex, LinkVertex)> {
    let mut edges: Vec<(LinkVertex, LinkVertex)> = vec![];

    let mut current = graph.dart_face(face);
    let start_vertex = &graph.target(&graph.twin(&current));

    if graph.target(&graph.next(&current)) == *start_vertex {
        return edges;
    }

    loop {
        let next = graph.next(&current);

        if graph.target(&graph.next(&next)) == *start_vertex {
            break;
        }

        let from = graph.target(&next);

        edges.push((from, start_vertex.clone()));
        current = next;
    }

    edges
}

#[cfg(test)]
mod tests {
    use crate::algorithm::triangulation::triangulate;
    use crate::data_structure::{
        graph_dcel::GraphDCEL,
        graph_types::{Face, Vertex},
        link_graph::LinkGraph,
    };

    #[test]
    fn triangle() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
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
            Some(lf.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            None,
            None,
            Some(ld1.clone()),
            None,
        );
        let lof = lg.new_face(lt1.clone());
        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(lt1.clone()),
            None,
            Some(ld2.clone()),
            Some(lof.clone()),
        );
        let _lt3 = lg.new_dart(
            lv1.clone(),
            lv3.clone(),
            Some(lt2.clone()),
            Some(lt1.clone()),
            Some(ld3.clone()),
            Some(lof.clone()),
        );

        let edges = triangulate(&lg);

        assert_eq!(edges, vec![])
    }

    #[test]
    fn single_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        lg.new_face(ld1.clone());
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(ld1.clone()),
            Some(ld1.clone()),
            Some(ld1.clone()),
            None,
        );

        let edges = triangulate(&lg);
        assert_eq!(edges, vec![])
    }

    #[test]
    fn rectangle() {
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
            lv4.clone(),
            Some(ld2.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld4 = lg.new_dart(
            lv4.clone(),
            lv1.clone(),
            Some(ld3.clone()),
            Some(ld1.clone()),
            None,
            Some(lf.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            None,
            None,
            Some(ld1.clone()),
            None,
        );
        let lof = lg.new_face(lt1.clone());
        let lt2 = lg.new_dart(
            lv1.clone(),
            lv4.clone(),
            Some(lt1.clone()),
            None,
            Some(ld4.clone()),
            Some(lof.clone()),
        );
        let lt3 = lg.new_dart(
            lv4.clone(),
            lv3.clone(),
            Some(lt2.clone()),
            None,
            Some(ld3.clone()),
            Some(lof.clone()),
        );
        let _lt4 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(lt3.clone()),
            Some(lt1.clone()),
            Some(ld2.clone()),
            Some(lof.clone()),
        );

        let edges = triangulate(&lg);
        println!("{:?}", edges);
        assert_eq!(edges.len(), 2)
    }
}
