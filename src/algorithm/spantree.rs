use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};
use std::collections::{HashSet, VecDeque};

fn span(
    g: &impl GraphDCEL<LinkVertex, LinkDart, LinkFace,
        LinkGraphIter<LinkVertex>, LinkGraphIter<LinkDart>, LinkGraphIter<LinkFace>>,
    v: LinkVertex,
) -> HashSet<(LinkVertex, LinkVertex)> {
    if g.get_vertexes().count() <= 1 {
        return HashSet::new();
    }
    let mut queue = VecDeque::new();
    let mut result = HashSet::new();
    let mut visited = HashSet::new();
    queue.push_back(v);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        /*if visited.contains(&v) {
            continue;
        }*/
        visited.insert(v.clone());
        for n in neighbors(g, &v) {
            if visited.insert(n.clone()) {
                queue.push_back(n.clone());
                result.insert((v.clone(), n));
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
    let mut current_neighbor = g.dart_target(&current_dart);
    let mut result = vec![];
    loop {
        result.push(current_neighbor);
        let twin_dart = g.twin(&current_dart);
        current_dart = g.next(&twin_dart);
        if current_dart == first_dart {
            break;
        }
        current_neighbor = g.dart_target(&current_dart);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::algorithm::spantree::{neighbors, span};
    use crate::data_structure::link_graph::LinkGraph;

    #[test]
    fn single_vertex() {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();

        let edges = span(&lg, lv1);

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges, HashSet::new());
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
            Some(ld1.clone()),
            Some(lf),
        );

        let edges = span(&lg, lv1.clone());

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 1);
        assert!(edges.contains(&(lv1, lv2)));
    }

    #[test]
    fn triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(),
                              None, None, None, None);
        let lf = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(),
                              Some(ld0.clone()), None,
                              None, Some(lf.clone()));
        let ld2 = lg.new_dart(lv2.clone(), lv0.clone(),
                              Some(ld1.clone()), Some(ld0.clone()),
                              None, Some(lf));

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(),
                              None, None,
                              Some(ld0), None);
        let lof = lg.new_face(lt0.clone());
        let lt2 = lg.new_dart(lv0.clone(), lv2.clone(),
                              Some(lt0.clone()), None,
                              Some(ld2), Some(lof.clone()));
        lg.new_dart(lv2.clone(), lv1.clone(),
                    Some(lt2), Some(lt0),
                    Some(ld1), Some(lof));

        let edges = span(&lg, lv1.clone());

        println!("[RESULT]: {:?}", edges);
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(lv1.clone(), lv2)));
        assert!(edges.contains(&(lv1, lv0)));
    }

    #[test]
    fn neighbors_single_edge() {
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
            Some(ld1.clone()),
            Some(lf),
        );

        assert_eq!(neighbors(&lg, &lv1), vec![lv2.clone()]);
        assert_eq!(neighbors(&lg, &lv2), vec![lv1]);
    }

    #[test]
    fn neighbors_triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(),
                              None, None, None, None);
        let lf = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(),
                              Some(ld0.clone()), None,
                              None, Some(lf.clone()));
        let ld2 = lg.new_dart(lv2.clone(), lv0.clone(),
                              Some(ld1.clone()), Some(ld0.clone()),
                              None, Some(lf));

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(),
                              None, None,
                              Some(ld0), None);
        let lof = lg.new_face(lt0.clone());
        let lt2 = lg.new_dart(lv0.clone(), lv2.clone(),
                              Some(lt0.clone()), None,
                              Some(ld2), Some(lof.clone()));
        lg.new_dart(lv2.clone(), lv1.clone(),
                    Some(lt2), Some(lt0),
                    Some(ld1), Some(lof));

        assert_eq!(neighbors(&lg, &lv0), vec![lv2.clone(), lv1.clone()]);
        assert_eq!(neighbors(&lg, &lv1), vec![lv0.clone(), lv2.clone()]);
        assert_eq!(neighbors(&lg, &lv2), vec![lv1, lv0]);
    }
}
