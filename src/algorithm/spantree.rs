//! Contains implementation of a span tree
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashMap, HashSet, VecDeque};

/// The structure containing the span tree (downwards from root to leaves and upwards from leaf to root)
pub struct Span<T> {
    pub root: T,
    pub downwards: HashMap<T, HashSet<T>>,
    pub upwards: HashMap<T, T>,
}

impl Span<LinkVertex> {
    /// Returns a span tree beginning with root
    pub fn compute(
        g: &impl GraphDCEL<
            LinkVertex,
            LinkDart,
            LinkFace,
            LinkGraphIter<LinkVertex>,
            LinkGraphIter<LinkDart>,
            LinkGraphIter<LinkFace>,
        >,
        root: LinkVertex,
    ) -> Self {
        assert!(g.get_vertexes().count() > 1);
        let mut queue = VecDeque::new();
        let mut upwards = HashMap::new();
        let mut downwards = HashMap::new();
        let mut visited = HashSet::new();
        downwards.insert(root.clone(), HashSet::new());
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            visited.insert(u.clone());
            for n in g.neighbors(&u) {
                if visited.insert(n.clone()) {
                    queue.push_back(n.clone());
                    upwards.insert(n.clone(), u.clone());
                    if downwards.get(&u).is_none() {
                        downwards.insert(u.clone(), HashSet::new());
                    }
                    downwards.get_mut(&u).unwrap().insert(n);
                }
            }
        }
        Span {
            root,
            downwards,
            upwards,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::spantree::Span;
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
    use crate::utils::convert::UndirectedGraph;
    use petgraph::stable_graph::StableGraph;
    use std::collections::HashMap;

    #[test]
    #[should_panic]
    fn single_vertex() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();

        let edges = Span::compute(&lg, lv1).upwards;

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges, HashMap::new());
    }

    #[test]
    fn single_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        let lf = lg.new_face(ld1.clone());
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(ld1.clone()),
            Some(ld1.clone()),
            Some(ld1),
            Some(lf),
        );

        let edges = Span::compute(&lg, lv1.clone()).upwards;

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges.get(&lv2), Some(&lv1))
    }

    #[test]
    fn triangle() {
        let sg: UndirectedGraph = StableGraph::from_edges(&[(0, 1), (1, 2), (2, 0)]);

        let lg = MaximalPlanar::embed(sg);
        assert_eq!(lg.vertex_count(), 3);
        let lv0 = lg.vertex_by_id(0).unwrap();
        let lv1 = lg.vertex_by_id(1).unwrap();
        let lv2 = lg.vertex_by_id(2).unwrap();

        let edges = Span::compute(&lg, lv1.clone()).upwards;

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 2);
        assert_eq!(edges.get(&lv2), Some(&lv1));
        assert_eq!(edges.get(&lv0), Some(&lv1));
    }

    #[test]
    fn quad() {
        let sg: UndirectedGraph =
            StableGraph::from_edges(&[(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)]);

        let lg = MaximalPlanar::embed(sg);
        assert_eq!(lg.vertex_count(), 4);
        let lv0 = lg.vertex_by_id(0).unwrap();
        let lv1 = lg.vertex_by_id(1).unwrap();
        let lv2 = lg.vertex_by_id(2).unwrap();
        let lv3 = lg.vertex_by_id(3).unwrap();

        let edges = Span::compute(&lg, lv0.clone()).upwards;

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 3);
        assert_eq!(edges.get(&lv2), Some(&lv0));
        assert_eq!(edges.get(&lv1), Some(&lv0));
        assert_eq!(edges.get(&lv3), Some(&lv0));
    }
}
