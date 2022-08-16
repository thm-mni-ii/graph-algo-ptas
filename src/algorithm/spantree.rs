use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn span(
    g: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    v: LinkVertex,
) -> HashMap<LinkVertex, LinkVertex> {
    if g.get_vertexes().count() <= 1 {
        return HashMap::new();
    }
    let mut queue = VecDeque::new();
    let mut result = HashMap::new();
    let mut visited = HashSet::new();
    queue.push_back(v);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        visited.insert(v.clone());
        for n in g.neighbors(&v) {
            if visited.insert(n.clone()) {
                queue.push_back(n.clone());
                result.insert(n, v.clone());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::algorithm::spantree::span;
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
    use crate::utils::convert::UndirectedGraph;
    use petgraph::stable_graph::StableGraph;
    use std::collections::HashMap;

    #[test]
    fn single_vertex() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();

        let edges = span(&lg, lv1);

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

        let edges = span(&lg, lv1.clone());

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

        let edges = span(&lg, lv1.clone());

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 2);
        assert_eq!(edges.get(&lv2), Some(&lv1));
        assert_eq!(edges.get(&lv0), Some(&lv1));
    }
}
