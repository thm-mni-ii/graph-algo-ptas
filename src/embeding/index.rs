//! This module is the main entry point for embeding

use crate::{
    data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex},
    utils::convert::UndirectedGraph,
};

/// The Embeding trait is implemented by all embeding algorithms

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
    /// Receives a graph as an Argument an returns the embeding
    fn embed(graph: UndirectedGraph) -> T;
}
