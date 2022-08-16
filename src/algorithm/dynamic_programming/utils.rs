use arboretum_td::graph::{BaseGraph, HashMapGraph, MutableGraph};
use bitvec::vec::BitVec;
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::HashMap;

pub fn bit_vec_powerset(set: &FxHashSet<usize>, subset_size: usize) -> Vec<BitVec> {
    set.iter()
        .powerset()
        .map(|subset| to_bit_vec(subset.iter().copied(), subset_size))
        .collect()
}

pub fn init_bit_vec(size: usize) -> BitVec {
    let n = (size as f64 / 64.0).ceil() as usize;

    BitVec::from_vec(vec![0; n])
}

pub fn to_bit_vec<'a>(it: impl Iterator<Item = &'a usize>, size: usize) -> BitVec {
    let mut bit_vec: BitVec = init_bit_vec(size);

    for x in it {
        bit_vec.set(*x, true);
    }

    bit_vec
}

pub fn immutable_bit_vec_update(subset: &BitVec, v: usize) -> BitVec {
    let mut subset = subset.clone();
    subset.set(v, true);
    subset
}

// the result is a graph isomorphic to the input graph but is guaranteed to have vertex IDs 0..n-1.
pub fn remap_vertices(graph: &HashMapGraph) -> (HashMapGraph, HashMap<usize, usize>) {
    let mut remapped_graph = HashMapGraph::new();
    let mut forward_mapping = HashMap::new();
    let mut backward_mapping = HashMap::new();

    for (i, v) in graph.vertices().enumerate() {
        remapped_graph.add_vertex(i);
        forward_mapping.insert(v, i);
        backward_mapping.insert(i, v);
    }

    for u in graph.vertices() {
        for v in graph.neighborhood(u) {
            let remapped_u = forward_mapping.get(&u).unwrap();
            let remapped_v = forward_mapping.get(&v).unwrap();
            remapped_graph.add_edge(*remapped_u, *remapped_v);
        }
    }

    (remapped_graph, backward_mapping)
}
