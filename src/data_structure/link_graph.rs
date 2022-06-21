use std::{rc::Rc, cell::RefCell};

use super::graph_dcel::{GraphDCEL, Vertex, Dart, Face};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LinkVertexStruct {
    dart: Option<LinkDart>
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkVertex(Rc<RefCell<LinkVertexStruct>>);

impl LinkVertex {
    pub fn new() -> LinkVertex {
        return Default::default();
    }
}

impl Vertex for LinkVertex {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LinkDartStructure {
    target: LinkVertex,
    twin: Option<LinkDart>,
    next: Option<LinkDart>,
    prev: Option<LinkDart>,
    face: Option<LinkFace>
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkDart(Rc<RefCell<LinkDartStructure>>);

impl Dart for LinkDart {}

impl LinkDart {
    pub fn new(target: LinkVertex) -> LinkDart {
        return LinkDart(Rc::new(RefCell::new(LinkDartStructure {target, ..Default::default()})));
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LinkFaceStructure {
    dart: LinkDart,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkFace(Rc<RefCell<LinkFaceStructure>>);

impl Face for LinkFace {}

impl LinkFace {
    pub fn new(dart: LinkDart) -> LinkFace {
        return LinkFace(Rc::new(RefCell::new(LinkFaceStructure {dart})));
    }
}

pub struct LinkGraph {}

impl GraphDCEL<LinkVertex, LinkDart, LinkFace> for LinkGraph {
    fn dart_vertex(&self, vertex: LinkVertex) -> LinkDart {
        vertex.0.borrow().dart.clone().unwrap()
    }

    fn dart_face(&self, face: LinkFace) -> LinkDart {
        face.0.borrow().dart.clone()
    }

    fn twin(&self, dart: LinkDart) -> LinkDart {
        dart.0.borrow().twin.clone().unwrap()
    }

    fn target(&self, dart: LinkDart) -> LinkVertex {
        dart.0.borrow().target.clone()
    }

    fn face(&self, dart: LinkDart) -> LinkFace {
        dart.0.borrow().face.clone().unwrap()
    }

    fn next(&self, current: LinkDart) -> LinkDart {
        current.0.borrow().next.clone().unwrap()
    }

    fn prev(&self, current: LinkDart) -> LinkDart {
        current.0.borrow().prev.clone().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::{link_graph::LinkGraph, graph_dcel::GraphDCEL};

    use super::{LinkVertex, LinkDart, LinkFace};

    fn example_graph() -> LinkVertex {
        let lv1 = LinkVertex::new();
        let lv2 = LinkVertex::new();
        let lv3 = LinkVertex::new();
        let ld1 = LinkDart::new(lv1.clone());
        let ld2 = LinkDart::new(lv2.clone());
        let ld3 = LinkDart::new(lv3.clone());
        lv1.0.borrow_mut().dart = Some(ld2.clone());
        lv2.0.borrow_mut().dart = Some(ld3.clone());
        lv3.0.borrow_mut().dart = Some(ld1.clone());
        ld1.0.borrow_mut().next = Some(ld2.clone());
        ld2.0.borrow_mut().next = Some(ld3.clone());
        ld3.0.borrow_mut().next = Some(ld1.clone());
        ld1.0.borrow_mut().prev = Some(ld3.clone());
        ld2.0.borrow_mut().prev = Some(ld1.clone());
        ld3.0.borrow_mut().prev = Some(ld2.clone());
        let lt1 = LinkDart::new(lv2.clone());
        let lt2 = LinkDart::new(lv3.clone());
        let lt3 = LinkDart::new(lv1.clone());
        lt1.0.borrow_mut().next = Some(lt2.clone());
        lt2.0.borrow_mut().next = Some(lt3.clone());
        lt3.0.borrow_mut().next = Some(lt1.clone());
        lt1.0.borrow_mut().prev = Some(lt3.clone());
        lt2.0.borrow_mut().prev = Some(lt1.clone());
        lt3.0.borrow_mut().prev = Some(lt2.clone());
        ld1.0.borrow_mut().twin = Some(lt1.clone());
        ld2.0.borrow_mut().twin = Some(lt2.clone());
        ld3.0.borrow_mut().twin = Some(lt3.clone());
        lt1.0.borrow_mut().twin = Some(ld1.clone());
        lt2.0.borrow_mut().twin = Some(ld2.clone());
        lt3.0.borrow_mut().twin = Some(ld3.clone());
        let lf = LinkFace::new(ld1.clone());
        ld1.0.borrow_mut().face = Some(lf.clone());
        ld2.0.borrow_mut().face = Some(lf.clone());
        ld3.0.borrow_mut().face = Some(lf.clone());
        let lof = LinkFace::new(ld1.clone());
        lt1.0.borrow_mut().face = Some(lof.clone());
        lt2.0.borrow_mut().face = Some(lof.clone());
        lt3.0.borrow_mut().face = Some(lof.clone());
        return lv1
    }

    #[test]
    fn test() {
        let graph = LinkGraph{};
        let vertex = example_graph();
        let dart = graph.dart_vertex(vertex.clone());
        let face = graph.face(dart.clone());
        graph.dart_face(face.clone());
        graph.next(dart.clone());
        graph.prev(dart.clone());
        graph.twin(dart.clone());
        graph.target(dart.clone());
    }
}
