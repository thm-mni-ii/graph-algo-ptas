use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Span<T> {
    pub root: T,
    pub downwards: HashMap<T, HashSet<T>>,
    pub upwards: HashMap<T, T>,
}

impl Span<LinkVertex> {
    pub fn compute(
        g: &impl GraphDCEL<
            LinkVertex,
            LinkDart,
            LinkFace,
            LinkGraphIter<LinkVertex>,
            LinkGraphIter<LinkDart>,
            LinkGraphIter<LinkFace>,
        >,
        v: LinkVertex,
    ) -> Self {
        assert!(g.get_vertexes().count() > 1);
        let mut queue = VecDeque::new();
        let mut upwards = HashMap::new();
        let mut downwards = HashMap::new();
        let mut visited = HashSet::new();
        downwards.insert(v.clone(), HashSet::new());
        queue.push_back(v.clone());

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
            root: v,
            downwards,
            upwards,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::spantree::Span;
    use crate::data_structure::link_graph::LinkGraph;
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
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(), None, None, None, None);
        let lf = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv0.clone(),
            Some(ld1.clone()),
            Some(ld0.clone()),
            None,
            Some(lf),
        );

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(), None, None, Some(ld0), None);
        let lof = lg.new_face(lt0.clone());
        let lt2 = lg.new_dart(
            lv0.clone(),
            lv2.clone(),
            Some(lt0.clone()),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            Some(lt0),
            Some(ld1),
            Some(lof),
        );

        let edges = Span::compute(&lg, lv1.clone()).upwards;

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 2);
        assert_eq!(edges.get(&lv2), Some(&lv1));
        assert_eq!(edges.get(&lv0), Some(&lv1));
    }
}
