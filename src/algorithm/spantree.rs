use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashSet, VecDeque};

fn span(
    g: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    v: LinkVertex,
) -> Vec<(LinkVertex, LinkVertex)> {
    if g.get_vertexes().count() <= 1 {
        return vec![];
    }

    let mut queue = VecDeque::new();
    let mut result = vec![];
    let mut visited = HashSet::new();
    queue.push_back(v);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        /*if visited.contains(&v) {
            continue;
        }*/
        visited.insert(v.clone());
        for n in neighbors(g, &v) {
            if !visited.contains(&n) {
                queue.push_back(n.clone());
                result.push((v.clone(), n));
            }
        }
    }
    result
}

fn neighbors(
    g: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    v: &LinkVertex,
) -> Vec<LinkVertex> {
    let mut current_dart = g.dart_vertex(v);
    let first_dart = current_dart.clone();
    let mut current_neighbor = g.target(&current_dart);
    let mut result = vec![];
    loop {
        result.push(current_neighbor);
        let twin_dart = g.twin(&current_dart);
        current_dart = g.next(&twin_dart);
        if current_dart == first_dart {
            break;
        }
        current_neighbor = g.target(&current_dart);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::algorithm::spantree::span;
    use crate::data_structure::link_graph::LinkGraph;

    #[test]
    fn single_vertex() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();

        let edges = span(&lg, lv1);

        assert_eq!(edges, vec![]);
    }

    #[test]
    fn single_edge() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        lg.new_face(ld1.clone());
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(ld1.clone()),
            Some(ld1.clone()),
            Some(ld1.clone()),
            None,
        );

        let edges = span(&lg, lv1);
        assert_eq!(edges[0].0.get_id(), 0);
        assert_eq!(edges[0].1.get_id(), 1);
    }

    #[test]
    fn triangle() {}
}
