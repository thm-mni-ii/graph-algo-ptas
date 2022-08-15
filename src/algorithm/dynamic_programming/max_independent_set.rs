use super::{
    solve::{DynamicProgrammingTable, DynamicProgrammingTableEntry},
    utils::{bit_vec_powerset, immutable_bit_vec_update, init_bit_vec, to_bit_vec},
};
use arboretum_td::graph::{BaseGraph, HashMapGraph};
use fxhash::FxHashSet;
use itertools::Itertools;

pub fn handle_leaf_node(
    graph: &HashMapGraph,
    id: usize,
    tables: &mut Vec<DynamicProgrammingTable>,
    vertex: usize,
) {
    tables[id].insert(
        init_bit_vec(graph.order()),
        DynamicProgrammingTableEntry::new_leaf(0, None),
    );
    tables[id].insert(
        immutable_bit_vec_update(&init_bit_vec(graph.order()), vertex),
        DynamicProgrammingTableEntry::new_leaf(1, Some(vertex)),
    );
}

pub fn handle_join_node(
    graph: &HashMapGraph,
    id: usize,
    left_child_id: usize,
    right_child_id: usize,
    tables: &mut Vec<DynamicProgrammingTable>,
    vertex_set: &FxHashSet<usize>,
) {
    for subset_vec in vertex_set.iter().powerset() {
        let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
        let left_val = tables[left_child_id].get(&subset).unwrap().val;
        let right_val = tables[right_child_id].get(&subset).unwrap().val;
        let new_val = if left_val == i32::min_value() || right_val == i32::min_value() {
            i32::min_value()
        } else {
            left_val + right_val - subset_vec.len() as i32
        };

        tables[id].insert(
            subset.clone(),
            DynamicProgrammingTableEntry::new_join(new_val, left_child_id, right_child_id, subset),
        );
    }
}

pub fn handle_forget_node(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut Vec<DynamicProgrammingTable>,
    vertex_set: &FxHashSet<usize>,
    forgotten_vertex: usize,
) {
    for subset in bit_vec_powerset(vertex_set, graph.order()) {
        let val = tables[child_id].get(&subset).unwrap().val;
        let subset_with_v = immutable_bit_vec_update(&subset, forgotten_vertex);
        let val_with_v = tables[child_id].get(&subset_with_v).unwrap().val;
        let (new_val, subset_used) = if val > val_with_v {
            (val, subset.clone())
        } else {
            (val_with_v, subset_with_v)
        };
        tables[id].insert(
            subset,
            DynamicProgrammingTableEntry::new_forget(new_val, child_id, subset_used),
        );
    }
}

pub fn handle_introduce_node(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut Vec<DynamicProgrammingTable>,
    _: &FxHashSet<usize>,
    child_vertex_set: &FxHashSet<usize>,
    introduced_vertex: usize,
) {
    for subset_vec in child_vertex_set.iter().powerset() {
        let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
        let val = tables[child_id].get(&subset).unwrap().val;
        tables[id].insert(
            subset.clone(),
            DynamicProgrammingTableEntry::new_intro(val, child_id, subset.clone(), None),
        );

        let has_edge = subset_vec
            .iter()
            .any(|w| graph.has_edge(introduced_vertex, **w));

        let (new_val, node_used) = if has_edge {
            (i32::min_value(), None)
        } else {
            (val + 1, Some(introduced_vertex))
        };

        let subset_with_v = immutable_bit_vec_update(&subset, introduced_vertex);
        tables[id].insert(
            subset_with_v,
            DynamicProgrammingTableEntry::new_intro(new_val, child_id, subset, node_used),
        );
    }
}
