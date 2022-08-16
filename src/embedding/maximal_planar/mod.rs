//! Implements the maximal planar embedding algorithm from [A simple linear time algorithm for
//! embedding maximal planar graphs](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf)
//!
//! ```
//! use graph_algo_ptas::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
//! use graph_algo_ptas::generation::planar::generate;
//!
//! let graph = generate(10, None).to_pet_graph(); // the graph to embedd
//! let dcel = MaximalPlanar::embed(graph.clone()); // embedd the graph
//! ```

pub mod index;
mod phase1;
mod phase2;
mod phase3;
mod stack_item;
