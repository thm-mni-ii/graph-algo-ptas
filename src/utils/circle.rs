use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkGraph, LinkVertex},
};

pub fn generate_circle(n: usize) -> LinkGraph {
    let mut lg = LinkGraph::new();
    let nodes: Vec<LinkVertex> = (0..n).map(|_| lg.new_vertex()).collect();

    let mut prev = None;
    let mut prev_twin = None;
    let mut inner_face = None;
    let mut outter_face = None;

    for i in 0..n + 1 {
        let node = nodes.get(i % n).unwrap();
        let next = nodes.get((i + 1) % n).unwrap();
        let mut next_dart = None;
        let mut next_dart_twin = None;
        if i == n {
            let mut dart_iter = lg.get_darts();
            next_dart = dart_iter.next();
            next_dart_twin = dart_iter.next();
        }
        let ld = lg.new_dart(
            node.clone(),
            next.clone(),
            prev.clone(),
            next_dart,
            None,
            inner_face.clone(),
        );
        let lt = lg.new_dart(
            next.clone(),
            node.clone(),
            next_dart_twin,
            prev_twin,
            Some(ld.clone()),
            outter_face.clone(),
        );

        if inner_face.is_none() {
            inner_face = Some(lg.new_face(ld.clone()))
        }
        if outter_face.is_none() {
            outter_face = Some(lg.new_face(lt.clone()))
        }

        prev = Some(ld);
        prev_twin = Some(lt);
    }

    lg
}

#[cfg(test)]
mod tests {
    use crate::data_structure::{graph_dcel::GraphDCEL, link_graph::LinkDart};

    use super::generate_circle;

    #[test]
    fn test() {
        let cg = generate_circle(10);
        let sv = cg.get_vertexes().next().unwrap();
        let sd = cg.dart_vertex(&sv);
        let mut cd = sd.clone();
        println!("{:?}", cg.get_darts().collect::<Vec<LinkDart>>());
        for _i in 0..10 {
            cd = cg.next(&cd);
        }
        let tv = cg.target(&cd);
        assert_eq!(sv, tv);
        cd = sd.clone();
        for _i in 0..12 {
            println!("{:?}", cd);
            cd = cg.prev(&cd);
        }
        let tv = cg.target(&cd);
        assert_eq!(sv, tv);
        cd = cg.twin(&sd);
        for _i in 0..10 {
            cd = cg.next(&cd);
        }
        let tv = cg.target(&cd);
        assert_eq!(sv, tv);
        cd = cg.twin(&sd);
        for _i in 0..12 {
            cd = cg.prev(&cd);
        }
        let tv = cg.target(&cd);
        assert_eq!(sv, tv);
    }
}
