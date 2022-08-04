use std::{collections::HashSet, hash::BuildHasherDefault};

use arboretum_td::tree_decomposition::TreeDecomposition;
use fxhash::FxHasher;

#[derive(Debug, Clone, Copy)]
pub enum NiceTdNodeType {
    Join,
    Introduce(usize),
    Forget(usize),
    Leaf,
}

pub struct NiceTreeDecomposition {
    td: TreeDecomposition,
    mapping: Vec<NiceTdNodeType>,
}

type FxHashSet<T> = HashSet<T, BuildHasherDefault<FxHasher>>;

impl NiceTreeDecomposition {
    fn from(mut td: TreeDecomposition) -> Self {
        let root = td.root.unwrap_or(0);
        td.root = Some(root);

        Self::nicify_multi_child_nodes(root, &td.bags()[root].neighbors.clone(), &mut td);
        Self::nicify_double_child_nodes(root, &td.bags()[root].neighbors.clone(), &mut td);
        Self::nicify_single_child_nodes(root, &td.bags()[root].neighbors.clone(), &mut td);
        Self::nicify_leaf_nodes(root, &td.bags()[root].neighbors.clone(), &mut td);

        let mut mapping: Vec<Option<NiceTdNodeType>> = vec![None; td.bags().len()];

        assert!(Self::is_nice_td(
            &td,
            root,
            &td.bags()[root].neighbors.clone(),
            &mut mapping
        ));
        assert!(mapping.iter().all(|node_type| { node_type.is_some() }));

        Self {
            td: td.to_owned(),
            mapping: mapping.iter().map(|node_type| node_type.unwrap()).collect(),
        }
    }

    fn nicify_multi_child_nodes(
        id: usize,
        children: &FxHashSet<usize>,
        td: &mut TreeDecomposition,
    ) {
        if children.len() <= 2 {
            for child_id in children {
                Self::nicify_multi_child_nodes(
                    *child_id,
                    &Self::get_child_bag_ids(td, *child_id, id),
                    td,
                );
            }

            return;
        }

        let vertex_set: FxHashSet<usize> = td.bags()[id].vertex_set.clone();
        let left_child_id = td.add_bag(vertex_set);
        td.add_edge(id, left_child_id);
        let mut it = children.iter();
        let right_child_id = *it.next().unwrap();

        for child_id in it {
            Self::remove_edge(td, id, *child_id);
            td.add_edge(left_child_id, *child_id);
        }

        Self::nicify_multi_child_nodes(
            left_child_id,
            &Self::get_child_bag_ids(td, left_child_id, id),
            td,
        );

        Self::nicify_multi_child_nodes(
            right_child_id,
            &Self::get_child_bag_ids(td, right_child_id, id),
            td,
        );
    }

    fn nicify_double_child_nodes(
        id: usize,
        children: &FxHashSet<usize>,
        td: &mut TreeDecomposition,
    ) {
        if children.len() != 2 {
            for child_id in children {
                Self::nicify_double_child_nodes(
                    *child_id,
                    &Self::get_child_bag_ids(td, *child_id, id),
                    td,
                );
            }

            return;
        }

        let mut it = children.iter();
        let left_child_id = *it.next().unwrap();
        let right_child_id = *it.next().unwrap();
        let vertex_set: FxHashSet<usize> = td.bags()[id].vertex_set.clone();
        let new_left_child_id = td.add_bag(vertex_set.clone());
        let new_right_child_id = td.add_bag(vertex_set);

        Self::remove_edge(td, id, left_child_id);
        Self::remove_edge(td, id, right_child_id);
        td.add_edge(id, new_left_child_id);
        td.add_edge(id, new_right_child_id);
        td.add_edge(new_left_child_id, left_child_id);
        td.add_edge(new_right_child_id, right_child_id);

        Self::nicify_double_child_nodes(
            left_child_id,
            &Self::get_child_bag_ids(td, left_child_id, new_left_child_id),
            td,
        );

        Self::nicify_double_child_nodes(
            right_child_id,
            &Self::get_child_bag_ids(td, right_child_id, new_right_child_id),
            td,
        );
    }

    fn nicify_single_child_nodes(
        id: usize,
        children: &FxHashSet<usize>,
        td: &mut TreeDecomposition,
    ) {
        if children.len() != 1 {
            for child_id in children {
                Self::nicify_single_child_nodes(
                    *child_id,
                    &Self::get_child_bag_ids(td, *child_id, id),
                    td,
                );
            }

            return;
        }

        let mut vertex_set: FxHashSet<usize> = td.bags()[id].vertex_set.clone();
        let child_id = *children.iter().next().unwrap();
        let child_vertex_set: FxHashSet<usize> = td.bags()[child_id].vertex_set.clone();

        if vertex_set.eq(&child_vertex_set.clone()) {
            Self::remove_edge(td, id, child_id);

            for grandchild_id in Self::get_child_bag_ids(td, child_id, id) {
                td.add_edge(id, grandchild_id);
                Self::remove_edge(td, child_id, grandchild_id);
                Self::nicify_single_child_nodes(
                    grandchild_id,
                    &Self::get_child_bag_ids(td, grandchild_id, id),
                    td,
                );
            }

            Self::remove_bag(td, child_id);

            return;
        }

        let mut parent_id = id;

        for v in vertex_set.clone().difference(&child_vertex_set) {
            vertex_set.remove(v);

            if vertex_set.eq(&child_vertex_set) {
                break;
            }

            let new_child_id = td.add_bag(vertex_set.clone());
            Self::remove_edge(td, parent_id, child_id);
            td.add_edge(parent_id, new_child_id);
            td.add_edge(new_child_id, child_id);
            parent_id = new_child_id;
        }

        for v in child_vertex_set.difference(&vertex_set.clone()) {
            vertex_set.insert(*v);

            if vertex_set.eq(&child_vertex_set) {
                break;
            }

            let new_child_id = td.add_bag(vertex_set.clone());
            Self::remove_edge(td, parent_id, child_id);
            td.add_edge(parent_id, new_child_id);
            td.add_edge(new_child_id, child_id);
            parent_id = new_child_id;
        }

        Self::nicify_single_child_nodes(
            child_id,
            &Self::get_child_bag_ids(td, child_id, parent_id),
            td,
        );
    }

    fn nicify_leaf_nodes(id: usize, children: &FxHashSet<usize>, td: &mut TreeDecomposition) {
        if !children.is_empty() {
            for child_id in children {
                Self::nicify_leaf_nodes(*child_id, &Self::get_child_bag_ids(td, *child_id, id), td);
            }

            return;
        }

        let bag = &td.bags()[id];
        let mut vertex_set = bag.vertex_set.clone();
        let mut parent_id = id;

        while vertex_set.len() > 1 {
            let v = *vertex_set.iter().next().unwrap();
            vertex_set.remove(&v);
            let new_child_id = td.add_bag(vertex_set.clone());
            td.add_edge(parent_id, new_child_id);
            parent_id = new_child_id;
        }
    }

    fn is_nice_td(
        td: &TreeDecomposition,
        id: usize,
        children: &FxHashSet<usize>,
        mapping: &mut Vec<Option<NiceTdNodeType>>,
    ) -> bool {
        match Self::get_nice_td_node_type(td, id, children) {
            Some(node_type) => {
                mapping[id] = Some(node_type);
                children.iter().all(|child_id| {
                    Self::is_nice_td(
                        td,
                        *child_id,
                        &Self::get_child_bag_ids(td, *child_id, id),
                        mapping,
                    )
                })
            }
            None => false,
        }
    }

    fn get_nice_td_node_type(
        td: &TreeDecomposition,
        id: usize,
        children: &FxHashSet<usize>,
    ) -> Option<NiceTdNodeType> {
        if Self::is_nice_td_join(td, id, children) {
            return Some(NiceTdNodeType::Join);
        } else if let Some(v) = Self::is_nice_td_intro(td, id, children) {
            return Some(NiceTdNodeType::Introduce(v));
        } else if let Some(v) = Self::is_nice_td_forget(td, id, children) {
            return Some(NiceTdNodeType::Forget(v));
        } else if Self::is_nice_td_leaf(td, id, children) {
            return Some(NiceTdNodeType::Leaf);
        }

        None
    }

    fn is_nice_td_join(td: &TreeDecomposition, id: usize, children: &FxHashSet<usize>) -> bool {
        let mut it = children.iter();
        let left = it.next();
        let right = it.next();
        let none = it.next();

        match (left, right, none) {
            (Some(left), Some(right), None) => {
                let vertex_set: &FxHashSet<usize> = &td.bags()[id].vertex_set;
                let left_vertex_set: &FxHashSet<usize> = &td.bags()[*left].vertex_set;
                let right_vertex_set: &FxHashSet<usize> = &td.bags()[*right].vertex_set;
                vertex_set == left_vertex_set && vertex_set == right_vertex_set
            }
            _ => false,
        }
    }

    fn is_nice_td_intro(
        td: &TreeDecomposition,
        id: usize,
        children: &FxHashSet<usize>,
    ) -> Option<usize> {
        let mut it = children.iter();
        let child = it.next();
        let none = it.next();

        match (child, none) {
            (Some(child), None) => {
                let vertex_set: &FxHashSet<usize> = &td.bags()[id].vertex_set;
                let child_vertex_set: &FxHashSet<usize> = &td.bags()[*child].vertex_set;

                if !vertex_set.is_superset(child_vertex_set)
                    || vertex_set.len() != child_vertex_set.len() + 1
                {
                    return None;
                }

                Some(*vertex_set.difference(child_vertex_set).next().unwrap())
            }
            _ => None,
        }
    }

    fn is_nice_td_forget(
        td: &TreeDecomposition,
        id: usize,
        children: &FxHashSet<usize>,
    ) -> Option<usize> {
        let mut it = children.iter();
        let child = it.next();

        if it.next().is_some() {
            return None;
        }

        match child {
            Some(child) => {
                let vertex_set: &FxHashSet<usize> = &td.bags()[id].vertex_set;
                let child_vertex_set: &FxHashSet<usize> = &td.bags()[*child].vertex_set;

                if !vertex_set.is_subset(child_vertex_set)
                    || vertex_set.len() + 1 != child_vertex_set.len()
                {
                    return None;
                }

                Some(*child_vertex_set.difference(vertex_set).next().unwrap())
            }
            None => None,
        }
    }

    fn is_nice_td_leaf(td: &TreeDecomposition, id: usize, children: &FxHashSet<usize>) -> bool {
        let vertex_set = &td.bags()[id].vertex_set;

        children.is_empty() && vertex_set.len() == 1
    }

    fn remove_edge(td: &mut TreeDecomposition, b1: usize, b2: usize) {
        assert!(b1 < td.bags.len());
        assert!(b2 < td.bags.len());
        assert_ne!(b1, b2);
        td.bags[b1].neighbors.remove(&b2);
        td.bags[b2].neighbors.remove(&b1);
    }

    // taken from https://github.com/jmeintrup/arboretum/blob/master/src/tree_decomposition.rs
    fn remove_bag(td: &mut TreeDecomposition, id: usize) {
        assert!(td.bags[id].neighbors.is_empty());
        if id == td.bags.len() - 1 {
            td.bags.pop();
        } else {
            let old_last = td.bags.swap_remove(id);
            assert!(old_last.neighbors.is_empty());
            td.bags[id].id = id;
            let old_last = td.bags.len();

            for neighbor in td.bags[id].neighbors.clone() {
                assert!(td.bags[neighbor].neighbors.remove(&old_last));
                assert!(td.bags[neighbor].neighbors.insert(id));
            }
        }
    }

    fn get_child_bag_ids(
        td: &TreeDecomposition,
        vertex_id: usize,
        parent_id: usize,
    ) -> FxHashSet<usize> {
        td.bags()[vertex_id]
            .neighbors
            .iter()
            .filter(|id| **id != parent_id)
            .copied()
            .collect()
    }
}
