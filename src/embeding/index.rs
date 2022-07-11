use petgraph::{Graph, Undirected};

use crate::data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex};
use crate::data_structure::graph_types::Vertex as VertexType;

pub trait Embeding<V: Vertex, D: Dart, F: Face, T: GraphDCEL<V, D, F>> {
    fn embed(graph: Graph<VertexType, (), Undirected>) -> T;
}
