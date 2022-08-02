use petgraph::{stable_graph::StableGraph, Graph, Undirected};

use crate::data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex};
pub trait Embeding<
    V: Vertex,
    D: Dart,
    F: Face,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
    T: GraphDCEL<V, D, F, VI, DI, FI>,
>
{
    fn embed(graph: StableGraph<u32, (), Undirected>) -> T;
}
