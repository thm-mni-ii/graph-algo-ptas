//! Contains the ring_segment function

use crate::data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Returns a new graph containing the nodes of ring i and the nodes less then i combined in a single node.
pub fn ring_segment<
    V: Vertex + Eq + Hash + Clone + Debug,
    D: Dart + Eq + Hash + Clone + Debug,
    F: Face + Eq + Hash + Clone + Debug,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
    G: GraphDCEL<V, D, F, VI, DI, FI> + Default,
>(
    input_graph: &G,
    rings: Vec<HashSet<V>>,
    i: usize,
) -> G {
    let mut output_graph = G::default();

    let ring_map = rings
        .iter()
        .enumerate()
        .flat_map(|(i, ring)| ring.iter().map(move |vertex| (vertex.clone(), i)))
        .collect::<HashMap<_, _>>();
    let ring = rings.get(i).cloned().unwrap_or_default();
    let mut vertexes = HashMap::new();
    let mut faces = HashMap::new();

    let inner_rings_not_empty = rings
        .iter()
        .take(i)
        .collect::<Vec<_>>()
        .iter()
        .any(|ring| !ring.is_empty());
    let inner_vertex = if inner_rings_not_empty {
        Some(output_graph.add_vertex())
    } else {
        None
    };
    let mut outer_face: Option<F> = None;

    let mut stack: Vec<D> = Vec::new();
    let mut visited = HashSet::new();

    let start_vertex = match ring.iter().next() {
        Some(vertex) => vertex,
        None => return output_graph,
    };
    let mut current_dart = input_graph.dart_vertex(start_vertex);

    let mut prev = None;
    let mut prev_twin = None;
    loop {
        let input_twin = input_graph.twin(&current_dart);
        let input_current_vertex = input_graph.dart_target(&input_twin);

        let ring_of_current_vertex = *ring_map.get(&input_current_vertex).unwrap();
        if ring_of_current_vertex == i {
            let current_vertex = vertexes
                .entry(input_current_vertex)
                .or_insert_with(|| output_graph.add_vertex())
                .clone();
            let input_target_vertex = input_graph.dart_target(&current_dart);

            let ring_of_target_vertex = *ring_map.get(&input_target_vertex).unwrap();
            let target_vertex_option = match ring_of_target_vertex.cmp(&i) {
                Ordering::Equal => Some(
                    vertexes
                        .entry(input_target_vertex)
                        .or_insert_with(|| output_graph.add_vertex())
                        .clone(),
                ),
                Ordering::Less => inner_vertex.clone(),
                Ordering::Greater => {
                    if let Some(last) = stack.last() {
                        let outer_face = match &outer_face {
                            Some(outer_face) => outer_face.clone(),
                            None => {
                                let new_face = output_graph.add_face(last.clone());
                                outer_face = Some(new_face.clone());
                                new_face
                            }
                        };
                        output_graph.set_face(last, outer_face);
                    }
                    None
                },
            };

            if let Some(target_vertex) = target_vertex_option {
                let face = faces.get(&input_graph.face(&current_dart)).cloned();
                let twin_face = faces.get(&input_graph.face(&input_twin)).cloned();

                prev = Some(output_graph.add_dart(
                    current_vertex.clone(),
                    target_vertex.clone(),
                    prev,
                    None,
                    None,
                    face.clone(),
                ));
                prev_twin = Some(output_graph.add_dart(
                    target_vertex,
                    current_vertex,
                    None,
                    prev_twin,
                    prev.clone(),
                    twin_face.clone(),
                ));

                if face.is_none() {
                    faces.insert(
                        input_graph.face(&current_dart),
                        output_graph.add_face(prev.clone().unwrap()),
                    );
                }
                if twin_face.is_none() {
                    faces.insert(
                        input_graph.face(&input_twin),
                        output_graph.add_face(prev_twin.clone().unwrap()),
                    );
                }
            }
        }

        visited.insert(current_dart.clone());
        stack.push(current_dart.clone());

        current_dart = input_graph.next(&current_dart);
        while visited.contains(&current_dart) {
            let mut potential_dart = current_dart.clone();
            while {
                potential_dart = input_graph.next(&input_graph.twin(&potential_dart));
                visited.contains(&potential_dart) && potential_dart != current_dart
            } {}

            current_dart = if !visited.contains(&potential_dart) {
                potential_dart
            } else {
                match stack.pop() {
                    Some(dart) => dart,
                    None => return output_graph,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::example::three_ring_graph;
    use crate::data_structure::link_graph::{LinkGraph, LinkVertex};
    use crate::data_structure::ring_segment::ring_segment;
    use std::collections::HashSet;

    fn generate_ring_graph() -> (LinkGraph, Vec<HashSet<LinkVertex>>) {
        let (graph, vertexes, _) = three_ring_graph();
        let ring_one: HashSet<_> = [vertexes[0].clone()].into_iter().collect();
        let ring_two: HashSet<_> = [
            vertexes[1].clone(),
            vertexes[2].clone(),
            vertexes[3].clone(),
            vertexes[4].clone(),
        ]
        .into_iter()
        .collect();
        let ring_three: HashSet<_> = [
            vertexes[5].clone(),
            vertexes[6].clone(),
            vertexes[7].clone(),
            vertexes[8].clone(),
            vertexes[9].clone(),
            vertexes[10].clone(),
            vertexes[11].clone(),
            vertexes[12].clone(),
        ]
        .into_iter()
        .collect();
        let rings = vec![ring_one, ring_two, ring_three];
        (graph, rings)
    }

    #[test]
    fn test_ring_one() {
        let (graph, rings) = generate_ring_graph();
        let segmented = ring_segment(&graph, rings, 0);
        assert_eq!(segmented.vertex_count(), 1);
    }

    #[test]
    fn test_ring_two() {
        let (graph, rings) = generate_ring_graph();
        let segmented = ring_segment(&graph, rings, 1);
        assert_eq!(segmented.vertex_count(), 5);
    }

    #[test]
    fn test_ring_three() {
        let (graph, rings) = generate_ring_graph();
        let segmented = ring_segment(&graph, rings, 2);
        assert_eq!(segmented.vertex_count(), 9);
    }

    #[test]
    fn test_ring_four() {
        let (graph, rings) = generate_ring_graph();
        let segmented = ring_segment(&graph, rings, 3);
        assert_eq!(segmented.vertex_count(), 1);
    }
}
