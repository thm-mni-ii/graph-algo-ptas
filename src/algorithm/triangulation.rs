use crate::data_structure::{
    graph_dcel::GraphDCEL,
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
    fn single_face() {
        for x in 3..100 {
            let graph = generate_single_face(x);

            let edges = triangulate(&graph);

            assert_eq!(edges.len(), (x - 3) * 2)
        }
    }

    #[test]
    fn two_face() {
        //TODO
    }
}
