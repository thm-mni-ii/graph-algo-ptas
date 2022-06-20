use std::collections::hash_map::Keys;
use crate::data_structure::graph::Graph;
use crate::data_structure::graph_types::{Dart, Edge, Vertex};
use std::collections::{HashMap, HashSet};

#[derive(Default, Eq, PartialEq, Debug)]
pub struct MapGraph {
    vertices: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph for MapGraph {
    fn new() -> Self {
        MapGraph::default()
    }

    fn get_vertices(&self) -> Keys<Vertex, Vec<Vertex>> {
        self.vertices.keys()
    }

    fn get_edges(&self) -> Vec<Edge> {
        let mut edges: HashSet<Edge> = HashSet::new();
        for vert_iter in self.vertices.iter() {
            let from = *vert_iter.0;
            for to in vert_iter.1.iter() {
                edges.insert(Edge::new(Dart::new(from, *to), Dart::new(*to, from)).unwrap());
            }
        }
        Vec::from_iter(edges)
    }

    fn contains_vertex(&self, v: Vertex) -> bool {
        self.vertices.contains_key(&v)
    }

    fn contains_edge(&self, e: Edge) -> bool {
        let (from, to) = e.get_vertices();
        self.is_adjacent(from, to)
    }

    fn is_adjacent(&self, v: Vertex, u: Vertex) -> bool {
        (self.vertices.contains_key(&v) && self.vertices[&v].contains(&u))
        || (self.vertices.contains_key(&u) && self.vertices[&u].contains(&v))
    }

    fn add_vertex(&mut self, v: Vertex) -> bool {
        if self.vertices.contains_key(&v) {
            return false;
        }
        self.vertices.insert(v, Vec::new());
        true
    }

    fn rem_vertex(&mut self, v: Vertex) -> bool {
        if !self.vertices.contains_key(&v) {
            return false;
        }
        self.vertices.remove(&v);
        for adjacent_vertices in self.vertices.values_mut() {
            adjacent_vertices.retain(|x| *x != v); // swap_remove?
        }
        true
    }

    fn add_edge(&mut self, e: Edge) {
        todo!()
    }

    fn rem_edge(&mut self, e: Edge) {
        todo!()
    }

    fn get_neighbours(&self, v: Vertex) -> &[Vertex] {
        self.vertices.get(&v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::graph::Graph;
    use crate::data_structure::map_graph::MapGraph;

    #[test]
    fn create_graph() {
        let result = MapGraph::new();

        assert_eq!(result, MapGraph::default());
    }

    // TODO: Tests
}