//! Contains the tree_decomposition function

//! Contains the tree_decomposition function
use arboretum_td::tree_decomposition::TreeDecomposition;
use fxhash::FxHashSet;

use std::collections::{HashMap, HashSet};

use crate::algorithm::spantree::Span;
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    link_graph::{LinkDart, LinkFace, LinkGraphIter, LinkVertex},
};

fn tree_decomposition(
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    dual_graph: HashMap<LinkFace, HashSet<LinkFace>>,
    spantree: &Span<LinkVertex>,
    root_vertex: LinkFace,
) -> TreeDecomposition {
    let mut tree: TreeDecomposition = Default::default();

    add_bags(root_vertex, 0, &mut tree, spantree, &dual_graph, graph);

    tree
}

fn add_bags(
    vertex: LinkFace,
    parent: usize,
    tree: &mut TreeDecomposition,
    spantree: &Span<LinkVertex>,
    dual_graph: &HashMap<LinkFace, HashSet<LinkFace>>,
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
) {
    let face_dart = graph.dart_face(&vertex);

    let face_vertices = get_face_vertices(graph, face_dart);

    let bag = create_bag(face_vertices, &spantree);

    if parent == 0 {
        tree.add_bag(bag);
    } else {
        tree.add_child_bags(parent, vec![bag]);
    }

    for c in dual_graph.get(&vertex).unwrap_or(&HashSet::new()) {
        add_bags(
            c.clone(),
            vertex.get_id(),
            tree,
            spantree,
            dual_graph,
            graph,
        );
    }
}

fn create_bag(
    face_vertices: HashSet<LinkVertex>,
    spantree: &&Span<LinkVertex>,
) -> FxHashSet<usize> {
    let mut vertices: FxHashSet<usize> = FxHashSet::default();

    for v in face_vertices {
        let mut node = v;

        vertices.insert(node.get_id());

        while spantree.upwards.get(&node).is_some() {
            node = spantree.upwards.get(&node).unwrap().clone();
            vertices.insert(node.get_id());
        }
    }
    vertices
}

fn get_face_vertices(
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
    mut dart: LinkDart,
) -> HashSet<LinkVertex> {
    let mut result: HashSet<LinkVertex> = HashSet::new();

    while result.insert(graph.dart_target(&dart)) {
        dart = graph.next(&dart);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::algorithm::dualgraph::dual_graph;
    use crate::algorithm::spantree::Span;
    use crate::algorithm::tree_decomposition::tree_decomposition;
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
    use crate::utils::convert::UndirectedGraph;
    use fxhash::FxHashSet;
    use petgraph::stable_graph::StableGraph;

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
            Some(lf.clone()),
        );
        let span = Span::compute(&lg, lv1.clone());
        let td = tree_decomposition(&lg, dual_graph(&lg, &span), &span, lf);

        println!("[RESULT]: {:?}", td);

        assert_eq!(td.bags.len(), 1);
        assert_eq!(td.max_bag_size, 2);
        assert_eq!(td.bags[0].vertex_set.len(), 2);
        let mut cb = FxHashSet::default();
        cb.insert(lv1.get_id());
        cb.insert(lv2.get_id());
        assert_eq!(td.bags[0].vertex_set, cb)
    }

    #[test]
    fn triangle() {
        let sg: UndirectedGraph = StableGraph::from_edges(&[(0, 1), (1, 2), (2, 0)]);

        let lg = MaximalPlanar::embed(sg);
        assert_eq!(lg.vertex_count(), 3);
        let lv0 = lg.vertex_by_id(0).unwrap();
        let lv1 = lg.vertex_by_id(1).unwrap();
        let lv2 = lg.vertex_by_id(2).unwrap();
        let lof = lg.face(&lg.get_dart(&lv1, &lv0).unwrap());

        let span = Span::compute(&lg, lv1.clone());
        let td = tree_decomposition(&lg, dual_graph(&lg, &span), &span, lof);

        println!("[RESULT]: {:?}", td);

        assert_eq!(td.bags.len(), 1);
        assert_eq!(td.max_bag_size, 3);
        assert_eq!(td.bags[0].vertex_set.len(), 3);
        let mut cb = FxHashSet::default();
        cb.insert(lv1.get_id());
        cb.insert(lv0.get_id());
        cb.insert(lv2.get_id());
        assert_eq!(td.bags[0].vertex_set, cb)
    }
}
