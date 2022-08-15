//! Contains the ring_segment function

use crate::data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

struct RingSegmentationState<
    'g,
    V: Vertex + Eq + Hash + Clone + Debug,
    D: Dart + Eq + Hash + Clone + Debug,
    F: Face + Eq + Hash + Clone + Debug,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
    G: GraphDCEL<V, D, F, VI, DI, FI> + Default,
> {
    input_graph: &'g G,
    output_graph: G,
    start_vertex: V,
    i: usize,
    ring_map: HashMap<V, usize>,
    ring: HashSet<V>,
    vertexes: HashMap<V, V>,
    darts: HashMap<D, D>,
    faces: HashMap<F, F>,
    stack: Vec<D>,
    visited: HashSet<D>,
    inner_vertex: Option<V>,
    outer_face: Option<F>,
    // PhantomData to make the compiler happy
    _vi: PhantomData<VI>,
    _di: PhantomData<DI>,
    _fi: PhantomData<FI>,
}

impl<
        'g,
        V: Vertex + Eq + Hash + Clone + Debug,
        D: Dart + Eq + Hash + Clone + Debug,
        F: Face + Eq + Hash + Clone + Debug,
        VI: Iterator<Item = V>,
        DI: Iterator<Item = D>,
        FI: Iterator<Item = F>,
        G: GraphDCEL<V, D, F, VI, DI, FI> + Default,
    > RingSegmentationState<'g, V, D, F, VI, DI, FI, G>
{
    fn init(
        input_graph: &'g G,
        start_vertex: V,
        rings: Vec<HashSet<V>>,
        i: usize,
    ) -> RingSegmentationState<'g, V, D, F, VI, DI, FI, G> {
        let mut output_graph = G::default();

        let ring_map = rings
            .iter()
            .enumerate()
            .flat_map(|(i, ring)| ring.iter().map(move |vertex| (vertex.clone(), i)))
            .collect::<HashMap<_, _>>();
        let ring = rings.get(i).cloned().unwrap_or_default();
        let vertexes = HashMap::new();
        let darts = HashMap::new();
        let faces = HashMap::new();

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
        let outer_face: Option<F> = None;

        let stack: Vec<D> = Vec::new();
        let visited = HashSet::new();

        RingSegmentationState {
            input_graph,
            output_graph,
            start_vertex,
            i,
            ring_map,
            ring,
            vertexes,
            darts,
            faces,
            stack,
            visited,
            inner_vertex,
            outer_face,
            _vi: Default::default(),
            _di: Default::default(),
            _fi: Default::default(),
        }
    }

    fn dfs(&mut self) {
        let mut current_dart = self.input_graph.dart_vertex(&self.start_vertex);

        loop {
            self.visit(&current_dart);

            self.visited.insert(current_dart.clone());
            self.stack.push(current_dart.clone());

            current_dart = self.input_graph.next(&current_dart);
            while self.visited.contains(&current_dart) {
                let mut potential_dart = current_dart.clone();
                while {
                    potential_dart = self
                        .input_graph
                        .next(&self.input_graph.twin(&potential_dart));
                    self.visited.contains(&potential_dart) && potential_dart != current_dart
                } {}

                current_dart = if !self.visited.contains(&potential_dart) {
                    potential_dart
                } else {
                    match self.stack.pop() {
                        Some(dart) => dart,
                        None => return,
                    }
                }
            }
        }
    }

    fn visit(&mut self, current_dart: &D) {
        let input_twin = self.input_graph.twin(current_dart);
        let input_current_vertex = self.input_graph.dart_target(&input_twin);

        let current_vertex = self.get_vertex(input_current_vertex);
        if let Some(current_vertex) = current_vertex {
            let input_target_vertex = self.input_graph.dart_target(current_dart);
            let target_vertex_option = self.get_vertex(input_target_vertex);

            if let Some(target_vertex) = target_vertex_option {
                let (input_prev, prev_skipped) =
                    self.get_first_dart_in_ring(current_dart, Mode::Prev);
                let (input_next, next_skipped) =
                    self.get_first_dart_in_ring(current_dart, Mode::Next);
                let skipped = prev_skipped || next_skipped;

                let prev = self.darts.get(&input_prev).cloned();
                let next = self.darts.get(&input_next).cloned();
                let twin = self
                    .darts
                    .get(&self.input_graph.twin(current_dart))
                    .cloned();
                let face = if !skipped {
                    self.faces
                        .get(&self.input_graph.face(current_dart))
                        .cloned()
                } else {
                    self.outer_face.clone()
                };

                let new_dart = self.output_graph.add_dart(
                    current_vertex,
                    target_vertex,
                    prev,
                    next,
                    twin,
                    face.clone(),
                );

                if face.is_none() {
                    if !skipped {
                        self.faces.insert(
                            self.input_graph.face(current_dart),
                            self.output_graph.add_face(new_dart.clone()),
                        );
                    } else {
                        self.outer_face = Some(self.output_graph.add_face(new_dart.clone()));
                    }
                }

                self.darts.insert(current_dart.clone(), new_dart);
            }
        }
    }

    fn get_vertex(&mut self, vertex: V) -> Option<V> {
        let ring_of_vertex = *self.ring_map.get(&vertex).unwrap();
        match ring_of_vertex.cmp(&self.i) {
            Ordering::Equal => Some(
                self.vertexes
                    .entry(vertex)
                    .or_insert_with(|| self.output_graph.add_vertex())
                    .clone(),
            ),
            Ordering::Less => Some(
                self.vertexes
                    .entry(vertex)
                    .or_insert_with(|| self.inner_vertex.clone().unwrap())
                    .clone(),
            ),
            Ordering::Greater => None,
        }
    }

    fn in_or_below_i_ring(&self, vertex: &V) -> bool {
        self.ring_map.get(vertex).unwrap() <= &self.i
    }

    fn get_first_dart_in_ring(&self, dart: &D, mode: Mode) -> (D, bool) {
        let mut dart_skipped = false;
        let mut next_dart = dart.clone();

        loop {
            next_dart = match mode {
                Mode::Next => GraphDCEL::next,
                Mode::Prev => GraphDCEL::prev,
            }(self.input_graph, &next_dart);
            let vertex = match mode {
                Mode::Next => GraphDCEL::dart_target,
                Mode::Prev => |graph: &G, dart: &D| graph.dart_target(&graph.prev(dart)),
            }(self.input_graph, &next_dart);
            if self.in_or_below_i_ring(&vertex) {
                return (next_dart, dart_skipped);
            }
            dart_skipped = true;
            next_dart = self.input_graph.twin(&next_dart)
        }
    }

    fn get_outer_face(&mut self, dart: &D) -> F {
        match &self.outer_face {
            Some(outer_face) => outer_face.clone(),
            None => {
                let new_face = self.output_graph.add_face(dart.clone());
                self.outer_face = Some(new_face.clone());
                new_face
            }
        }
    }
}

#[derive(Debug)]
enum Mode {
    Next,
    Prev,
}

/// The first element of the return tuple is the new graph containing the nodes of ring i and the nodes less then i combined in a single node.
/// The second element is start_vertex in the segmented graph.
/// The third element is the combined node.
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
    start_vertex: V,
    rings: Vec<HashSet<V>>,
    i: usize,
) -> (G, Option<V>, Option<V>) {
    let mut state = RingSegmentationState::init(input_graph, start_vertex, rings, i);
    state.dfs();
    let new_start_vertex = state.vertexes.get(&state.start_vertex).cloned();
    (state.output_graph, new_start_vertex, state.inner_vertex)
}

#[cfg(test)]
mod test {
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::example::three_ring_graph;
    use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};
    use crate::data_structure::ring_segment::ring_segment;
    use std::collections::HashSet;

    fn get_ring_graph() -> (LinkGraph, LinkVertex, Vec<HashSet<LinkVertex>>) {
        let (graph, vertexes, _darts) = three_ring_graph();
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
        (graph, vertexes[0].clone(), rings)
    }

    fn ensure_is_ring_of_length(graph: &LinkGraph, dart: LinkDart, length: usize) {
        let mut current_dart = dart.clone();
        let start_face = graph.face(&current_dart);
        for _ in 0..length {
            let last_dart = current_dart;
            current_dart = graph.next(&last_dart);
            if current_dart == last_dart {
                panic!("one loop detected");
            }
            let face = graph.face(&current_dart);
            if face != start_face {
                panic!("invalid faces");
            }
        }
        assert_eq!(current_dart, dart);
    }

    #[test]
    fn test_ring_one() {
        let (graph, start_vertex, rings) = get_ring_graph();
        let (segmented, start_vertex, inner_vertex) = ring_segment(&graph, start_vertex, rings, 0);
        assert_ne!(start_vertex, inner_vertex);
        assert_eq!(segmented.vertex_count(), 1);
        assert_eq!(segmented.edge_count(), 0);
        assert!(inner_vertex.is_none());
    }

    #[test]
    fn test_ring_two() {
        let (graph, start_vertex, rings) = get_ring_graph();
        let (segmented, start_vertex, inner_vertex) = ring_segment(&graph, start_vertex, rings, 1);
        assert_eq!(segmented.vertex_count(), 5);
        assert_eq!(segmented.edge_count(), 8);
        assert_eq!(start_vertex, inner_vertex);
        assert!(inner_vertex.is_some());
        let dart_on_outer_face =
            segmented.twin(&segmented.next(&segmented.dart_vertex(&start_vertex.unwrap())));
        ensure_is_ring_of_length(&segmented, dart_on_outer_face, 4);
    }

    #[test]
    fn test_ring_three() {
        let (graph, start_vertex, rings) = get_ring_graph();
        let (segmented, start_vertex, inner_vertex) = ring_segment(&graph, start_vertex, rings, 2);
        assert_eq!(segmented.vertex_count(), 9);
        assert_eq!(segmented.edge_count(), 28);
        assert_eq!(start_vertex, inner_vertex);
        assert!(inner_vertex.is_some());
        let dart_on_outer_face =
            segmented.twin(&segmented.next(&segmented.dart_vertex(&start_vertex.unwrap())));
        ensure_is_ring_of_length(&segmented, dart_on_outer_face, 8);
    }

    #[test]
    fn test_ring_four() {
        let (graph, start_vertex, rings) = get_ring_graph();
        let (segmented, start_vertex, inner_vertex) = ring_segment(&graph, start_vertex, rings, 3);
        assert_eq!(segmented.vertex_count(), 1);
        assert_eq!(start_vertex, inner_vertex);
        assert!(inner_vertex.is_some());
    }
}
