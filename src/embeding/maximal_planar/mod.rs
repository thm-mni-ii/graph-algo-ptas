//! Implements the maximal planar embeding algorithm from [A simple linear time algorithm for
//! embedding maximal planar graphs](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf)

pub mod index;
mod phase1;
mod phase2;
mod phase3;
mod stack_item;
