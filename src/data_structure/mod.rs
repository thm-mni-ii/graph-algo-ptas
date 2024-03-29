//! Contains the datastructures used by the crate

#[cfg(feature = "cli")]
pub mod dot_reader;
pub mod dot_renderer;
pub mod graph_dcel;
#[allow(dead_code)]
pub mod link_graph;
#[allow(dead_code)]
pub mod list_graph;
#[allow(dead_code)]
pub mod ring_segment;
