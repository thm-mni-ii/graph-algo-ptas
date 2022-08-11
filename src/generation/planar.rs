use crate::data_structure::list_graph::ListGraph;
use rand::rngs::StdRng;
use rand::{seq::SliceRandom, Rng, SeedableRng};

pub fn generate(mut n: usize, seed: Option<u64>) -> ListGraph {
    #[cfg(debug_graph_generation)]
    let mut counter = 0;
    let max_edges = 5 * n;
    let mut graph = ListGraph::k4();
    let mut urn = Vec::with_capacity(max_edges);
    let mut active = vec![false; max_edges];
    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    for edge in graph.edge_indexes() {
        urn.push(edge);
        active[edge] = true;
    }

    while n > 4 {
        let mut edge;
        while {
            edge = *urn.choose(&mut rng).unwrap();
            urn.remove(urn.iter().position(|e| &edge == e).unwrap());
            !active[edge]
        } {}
        urn.push(edge);

        let (v_a, v_b) = graph.edge(edge).unwrap();
        let vertex = if rng.gen_bool(0.5) { v_a } else { v_b };
        let typ = *if graph.edges(vertex).unwrap().len() == 3 {
            [3, 4].choose(&mut rng)
        } else {
            [3, 4, 5].choose(&mut rng)
        }
        .unwrap();
        if typ >= 4 {
            let succ_edge = graph.cyclic_incident_succ(edge, vertex).unwrap();
            graph.remove_edge(succ_edge);
            active[succ_edge] = false;
        }
        if typ == 5 {
            let succ_edge = graph.cyclic_incident_succ(edge, vertex).unwrap();
            graph.remove_edge(succ_edge);
            active[succ_edge] = false;
        }

        let new_vertex = graph.new_vertex();
        let mut current_edge = edge;
        let mut current_vertex = vertex;
        while {
            let new_edge = graph.add_edge(new_vertex, current_vertex, None, Some(current_edge));
            #[cfg(debug_graph_generation)]
            debug_graph(
                &graph,
                vertex,
                current_vertex,
                new_vertex,
                edge,
                current_edge,
                new_edge,
                &mut counter,
            );
            active[new_edge] = true;
            urn.push(new_edge);

            current_vertex = graph.opposite(current_vertex, current_edge).unwrap();
            current_edge = graph
                .cyclic_incident_prev(current_edge, current_vertex)
                .unwrap();
            current_vertex != vertex
        } {}
        n -= 1;
    }

    graph
}

#[cfg(debug_graph_generation)]
use crate::data_structure::list_graph::{EdgeId, NodeId};
#[cfg(debug_graph_generation)]
fn debug_graph(
    graph: &ListGraph,
    vertex: NodeId,
    current_vertex: NodeId,
    new_vertex: EdgeId,
    edge: EdgeId,
    current_edge: EdgeId,
    new_edge: EdgeId,
    counter: &mut usize,
) {
    let mut node_color = std::collections::HashMap::new();
    let mut edge_color = std::collections::HashMap::new();
    node_color.insert(vertex, "green".to_string());
    node_color.insert(current_vertex, "blue".to_string());
    node_color.insert(new_vertex, "red".to_string());
    edge_color.insert(edge, "green".to_string());
    edge_color.insert(current_edge, "blue".to_string());
    edge_color.insert(new_edge, "red".to_string());
    crate::debug::list_graph::write_as_files(&graph, &node_color, &edge_color, counter);
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    fn test_graph_generation_base() {
        let graph = generate(4, Some(0));

        assert_eq!(graph.node_indexes().count(), 4);
    }

    #[test]
    fn test_graph_generation_min() {
        let graph = generate(5, Some(0));

        assert_eq!(graph.node_indexes().count(), 5);
    }

    #[test]
    fn test_graph_generation_small() {
        let graph = generate(10, Some(0));

        assert_eq!(graph.node_indexes().count(), 10);
    }

    #[test]
    fn test_graph_generation_medium() {
        let graph = generate(50, Some(0));

        assert_eq!(graph.node_indexes().count(), 50);
    }

    #[test]
    fn test_graph_generation_large() {
        let graph = generate(100, Some(0));

        assert_eq!(graph.node_indexes().count(), 100);
    }
}
