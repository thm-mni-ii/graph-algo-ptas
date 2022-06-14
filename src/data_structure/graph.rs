use crate::data_structure::graph_types::{Edge, Vertex};

#[allow(unused)]
pub trait Graph {
    fn new() -> Self;

    fn get_vertices(&self) -> &[Vertex];
    fn get_edges(&self) -> &[Edge];

    fn contains_vertex(&self, v: Vertex) -> bool;
    fn contains_edge(&self, e: Edge) -> bool;
    fn is_adjacent(&self, v: Vertex, u: Vertex) -> bool;
    fn add_vertex(&mut self, v: Vertex);
    fn rem_vertex(&mut self, v: Vertex);
    fn add_edge(&mut self, e: Edge);
    fn rem_edge(&mut self, e: Edge);
    fn get_neighbours(&self, v: Vertex) -> Vec<Vertex>;
}
