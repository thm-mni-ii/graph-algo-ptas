use arboretum_td::tree_decomposition::TreeDecomposition;
use fxhash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NiceTdNodeType {
    Join,
    Introduce(usize),
    Forget(usize),
    Leaf,
}

pub struct NiceTreeDecomposition {
    pub td: TreeDecomposition,
    pub mapping: Vec<NiceTdNodeType>,
}

impl NiceTreeDecomposition {
    pub fn new(mut td: TreeDecomposition) -> Self {
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
                    &get_child_bag_ids(td, *child_id, id),
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
            &get_child_bag_ids(td, left_child_id, id),
            td,
        );

        Self::nicify_multi_child_nodes(
            right_child_id,
            &get_child_bag_ids(td, right_child_id, id),
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
                    &get_child_bag_ids(td, *child_id, id),
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
            &get_child_bag_ids(td, left_child_id, new_left_child_id),
            td,
        );

        Self::nicify_double_child_nodes(
            right_child_id,
            &get_child_bag_ids(td, right_child_id, new_right_child_id),
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
                    &get_child_bag_ids(td, *child_id, id),
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

            for grandchild_id in get_child_bag_ids(td, child_id, id) {
                td.add_edge(id, grandchild_id);
                Self::remove_edge(td, child_id, grandchild_id);
                Self::nicify_single_child_nodes(
                    grandchild_id,
                    &get_child_bag_ids(td, grandchild_id, id),
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

        Self::nicify_single_child_nodes(child_id, &get_child_bag_ids(td, child_id, parent_id), td);
    }

    fn nicify_leaf_nodes(id: usize, children: &FxHashSet<usize>, td: &mut TreeDecomposition) {
        if !children.is_empty() {
            for child_id in children {
                Self::nicify_leaf_nodes(*child_id, &get_child_bag_ids(td, *child_id, id), td);
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
                        &get_child_bag_ids(td, *child_id, id),
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
}

pub fn get_child_bag_ids(td: &TreeDecomposition, id: usize, parent_id: usize) -> FxHashSet<usize> {
    td.bags()[id]
        .neighbors
        .iter()
        .filter(|id| **id != parent_id)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::NiceTreeDecomposition;
    use crate::{
        algorithm::nice_tree_decomposition::get_child_bag_ids,
        utils::random_graph::random_hashmap_graph,
    };
    use arboretum_td::{solver::Solver, tree_decomposition::TreeDecomposition};
    use fxhash::FxHashSet;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn single_bag_with_1_vertex() {
        let mut td = TreeDecomposition::default();
        let mut vertex_set = FxHashSet::default();
        vertex_set.insert(1);
        td.add_bag(vertex_set);

        let nice_td = NiceTreeDecomposition::new(td.clone());
        assert!(nice_td.td.bags().len() == td.bags().len());
        assert!(nice_td.td.bags()[0].vertex_set == td.bags()[0].vertex_set);
    }

    #[test]
    fn single_bag_with_multiple_vertices() {
        let mut td = TreeDecomposition::default();
        let mut vertex_set = FxHashSet::default();
        vertex_set.insert(1);
        vertex_set.insert(2);
        vertex_set.insert(3);
        let id = td.add_bag(vertex_set);

        let nice_td = NiceTreeDecomposition::new(td.clone());
        let bag = &nice_td.td.bags()[id];
        let child_id = *bag.neighbors.iter().next().unwrap();
        let grandchild_id = get_child_bag_id(&nice_td.td, child_id, id).unwrap();

        assert!(nice_td.td.bags().len() == 3);
        assert!(bag.vertex_set.len() == 3);
        assert!(
            NiceTreeDecomposition::is_nice_td_intro(&nice_td.td, id, &bag.neighbors.clone())
                .is_some()
        );
        assert!(NiceTreeDecomposition::is_nice_td_intro(
            &nice_td.td,
            child_id,
            &get_child_bag_ids(&nice_td.td, child_id, id)
        )
        .is_some());
        assert!(NiceTreeDecomposition::is_nice_td_leaf(
            &nice_td.td,
            grandchild_id,
            &get_child_bag_ids(&nice_td.td, grandchild_id, child_id)
        ));
    }

    #[test]
    fn two_bags_with_multiple_vertices() {
        let mut td = TreeDecomposition::default();
        let mut vertex_set_1 = FxHashSet::default();
        vertex_set_1.insert(1);
        vertex_set_1.insert(2);
        let mut vertex_set_2 = FxHashSet::default();
        vertex_set_2.insert(2);
        vertex_set_2.insert(3);

        let id_1 = td.add_bag(vertex_set_1);
        let id_2 = td.add_bag(vertex_set_2);
        td.add_edge(id_1, id_2);

        let nice_td = NiceTreeDecomposition::new(td.clone());
        let bag = &nice_td.td.bags()[id_1];
        let child_id = *bag.neighbors.iter().next().unwrap();
        let grandchild_id = get_child_bag_id(&nice_td.td, child_id, id_1).unwrap();
        let grandgrandchild_id = get_child_bag_id(&nice_td.td, grandchild_id, child_id).unwrap();

        assert!(nice_td.td.bags().len() == 4);
        assert_eq!(
            NiceTreeDecomposition::is_nice_td_intro(&nice_td.td, id_1, &bag.neighbors.clone()),
            Some(1)
        );
        assert_eq!(
            NiceTreeDecomposition::is_nice_td_forget(
                &nice_td.td,
                child_id,
                &get_child_bag_ids(&nice_td.td, child_id, id_1)
            ),
            Some(3)
        );
        assert!(NiceTreeDecomposition::is_nice_td_intro(
            &nice_td.td,
            grandchild_id,
            &get_child_bag_ids(&nice_td.td, grandchild_id, child_id)
        )
        .is_some());
        assert!(NiceTreeDecomposition::is_nice_td_leaf(
            &nice_td.td,
            grandgrandchild_id,
            &get_child_bag_ids(&nice_td.td, grandgrandchild_id, grandchild_id)
        ));
    }

    #[test]
    fn random() {
        let seed = [1; 32];
        let mut rng = StdRng::from_seed(seed);

        for _ in 0..100 {
            let graph =
                random_hashmap_graph(rng.gen_range(1..30), rng.gen_range(0.05..0.1), &mut rng);
            let td = Solver::default().solve(&graph);
            let nice_td = NiceTreeDecomposition::new(td);
            assert!(nice_td.td.verify(&graph).is_ok(), "seed: {:?}", seed);
        }
    }

    fn get_child_bag_id(td: &TreeDecomposition, id: usize, parent_id: usize) -> Option<usize> {
        get_child_bag_ids(td, id, parent_id).iter().copied().next()
    }
}
