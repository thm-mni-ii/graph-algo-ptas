use super::{
    solve::{DpTable, DpTableEntry},
    utils::{bit_vec_powerset, immutable_bit_vec_update, init_bit_vec, to_bit_vec},
};
use arboretum_td::graph::{BaseGraph, HashMapGraph};
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::HashSet;

pub fn handle_leaf_node(graph: &HashMapGraph, id: usize, tables: &mut Vec<DpTable>, vertex: usize) {
    tables[id].insert(init_bit_vec(graph.order()), DpTableEntry::new_leaf(0, None));
    tables[id].insert(
        immutable_bit_vec_update(&init_bit_vec(graph.order()), vertex),
        DpTableEntry::new_leaf(1, Some(vertex)),
    );
}

pub fn handle_join_node(
    graph: &HashMapGraph,
    id: usize,
    left_child_id: usize,
    right_child_id: usize,
    tables: &mut Vec<DpTable>,
    vertex_set: &FxHashSet<usize>,
) {
    for subset_vec in vertex_set.iter().powerset() {
        let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
        let left_val = tables[left_child_id].get(&subset).unwrap().val;
        let right_val = tables[right_child_id].get(&subset).unwrap().val;

        let new_val = if left_val == i32::max_value() || right_val == i32::max_value() {
            i32::max_value()
        } else {
            left_val + right_val - subset_vec.len() as i32
        };

        tables[id].insert(
            subset.clone(),
            DpTableEntry::new_join(new_val, left_child_id, right_child_id, subset),
        );
    }
}

pub fn handle_forget_node(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut Vec<DpTable>,
    vertex_set: &FxHashSet<usize>,
    forgotten_vertex: usize,
) {
    for subset in bit_vec_powerset(vertex_set, graph.order()) {
        let val = tables[child_id].get(&subset).unwrap().val;
        let subset_with_v = immutable_bit_vec_update(&subset, forgotten_vertex);
        let val_with_v = tables[child_id].get(&subset_with_v).unwrap().val;
        let (min_val, subset_used) = if val < val_with_v {
            (val, subset.clone())
        } else {
            (val_with_v, subset_with_v)
        };
        tables[id].insert(
            subset,
            DpTableEntry::new_forget(min_val, child_id, subset_used),
        );
    }
}

pub fn handle_introduce_node(
    graph: &HashMapGraph,
    id: usize,
    child_id: usize,
    tables: &mut Vec<DpTable>,
    _: &FxHashSet<usize>,
    child_vertex_set: &FxHashSet<usize>,
    introduced_vertex: usize,
) {
    for subset_vec in child_vertex_set.iter().powerset() {
        let subset = to_bit_vec(subset_vec.iter().copied(), graph.order());
        let neighbors = graph
            .neighborhood_set(introduced_vertex)
            .iter()
            .filter(|w| child_vertex_set.contains(w));

        let mut is_covered = true;
        for w in neighbors {
            if !subset_vec.contains(&w) {
                is_covered = false;
                break;
            }
        }

        let val = if is_covered {
            tables[child_id].get(&subset).unwrap().val
        } else {
            i32::max_value()
        };
        let mut children = HashSet::new();
        children.insert((child_id, subset.clone()));
        tables[id].insert(
            subset.clone(),
            DpTableEntry {
                val,
                children: children.clone(),
                vertex_used: None,
            },
        );

        let child_val = tables[child_id].get(&subset).unwrap().val;
        let val = if child_val < i32::max_value() {
            child_val + 1
        } else {
            child_val
        };
        tables[id].insert(
            immutable_bit_vec_update(&subset, introduced_vertex),
            DpTableEntry {
                val,
                children,
                vertex_used: Some(introduced_vertex),
            },
        );
    }
}
