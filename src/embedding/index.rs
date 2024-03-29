//! This module is the main entry point for embedding

use crate::{
    data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex},
    utils::convert::UndirectedGraph,
};

/// The Embedding trait is implemented by all embedding algorithms
pub trait Embedding<
    V: Vertex,
    D: Dart,
    F: Face,
    VI: Iterator<Item = V>,
    DI: Iterator<Item = D>,
    FI: Iterator<Item = F>,
    T: GraphDCEL<V, D, F, VI, DI, FI>,
>
{
    /// Receives a graph as an Argument an returns the embedding
    fn embed(graph: UndirectedGraph) -> T;
}
