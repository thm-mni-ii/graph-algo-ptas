use arboretum_td::tree_decomposition::TreeDecomposition;

use crate::data_structure::graph_types::Vertex;

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
    dual_graph: HashMap<Vertex, HashSet<Vertex>>,
    spantree: HashMap<Vertex, Vertex>,
    root_vertex: Vertex,
) -> TreeDecomposition {
    let mut tree: TreeDecomposition;

    add_bags(root_vertex, 0, tree, spantree, dual_graph, graph);

    tree
}

fn add_bags(
    vertex: Vertex,
    parent: usize,
    tree: TreeDecomposition,
    spantree: HashMap<Vertex, Vertex>,
    dual_graph: HashMap<Vertex, HashSet<Vertex>>,
    graph: &impl GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    >,
) {
    let face_dart = graph.dart_face(vertex);

    let face_vertices = get_face_vertices(graph, face_dart);

    let bag = create_bag(face_vertices, spantree);

    if parent == 0 {
        tree.add_bag(bag);
    } else {
        tree.add_child_bags(parent, bag);
    }

    for c in dual_graph.get(&vertex) {
        add_bags(c, vertex, tree, spantree, dual_graph, graph);
    }
}

fn create_bag(face_vertices: HashSet<u32>, spantree: HashMap<Vertex, Vertex>) -> HashSet<usize> {
    let mut vertices: HashSet<usize> = HashSet::new();

    for v in face_vertices {
        let mut node = v;

        vertices.insert(node);

        while spantree.get(&node).is_some() {
            node = Some(spantree.get(&node));
            vertices.insert(node);
        }
    }
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
) -> HashSet<Vertex> {
    let mut result: HashSet<Vertex> = HashSet::new();

    while result.insert(graph.target(&dart)) {
        dart = graph.next(&dart);
    }
}
