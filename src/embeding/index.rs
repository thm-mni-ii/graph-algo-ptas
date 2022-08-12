//! This module is the main entry point for embeding

use petgraph::{stable_graph::StableGraph, Undirected};

use crate::data_structure::graph_dcel::{Dart, Face, GraphDCEL, Vertex};

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
    fn embed(graph: StableGraph<u32, (), Undirected>) -> T;
}
