use arboretum_td::tree_decomposition::TreeDecomposition;
use fxhash::FxHashSet;

use std::collections::{HashMap, HashSet};

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
    spantree: HashMap<LinkVertex, LinkVertex>,
    root_vertex: LinkFace,
) -> TreeDecomposition {
    let mut tree: TreeDecomposition = Default::default();

    add_bags(root_vertex, 0, &mut tree, &spantree, &dual_graph, graph);

    tree
}

fn add_bags(
    vertex: LinkFace,
    parent: usize,
    mut tree: &mut arboretum_td::tree_decomposition::TreeDecomposition,
    spantree: &HashMap<LinkVertex, LinkVertex>,
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

    for c in dual_graph.get(&vertex).unwrap() {
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
    spantree: &&HashMap<LinkVertex, LinkVertex>,
) -> FxHashSet<usize> {
    let mut vertices: FxHashSet<usize> = FxHashSet::default();

    for v in face_vertices {
        let mut node = v;

        vertices.insert(node.get_id());

        while spantree.get(&node).is_some() {
            node = spantree.get(&node).unwrap().clone();
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
