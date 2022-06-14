use crate::data_structure::{
    graph_dcel::GraphDCEL,
    graph_types::{Face, Vertex},
};

fn triangulate(graph: &impl GraphDCEL) -> Vec<(Vertex, Vertex)> {
    let mut edges: Vec<(Vertex, Vertex)> = vec![];

    for face in graph.get_faces() {
        edges.append(&mut triangulate_face(graph, face));
    }

    edges
}

fn triangulate_face(graph: &impl GraphDCEL, face: &Face) -> Vec<(Vertex, Vertex)> {
    let mut edges: Vec<(Vertex, Vertex)> = vec![];

    let mut current = graph.dart_face(*face);
    let start_vertex = graph.target(graph.twin(current));

    if graph.target(graph.next(current)) == start_vertex {
        return edges;
    }

    loop {
        let next = graph.next(current);

        if graph.target(graph.next(next)) == start_vertex {
            break;
        }

        let from = graph.target(next);

        edges.push((from, start_vertex));
        current = next;
    }

    edges
}

#[cfg(test)]
mod tests {
    #[test]
    fn triangle() {}

    fn single_edge() {}
}
