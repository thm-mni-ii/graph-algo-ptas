use std::{cell::RefCell, rc::Rc};

use super::graph_dcel::{Dart, Face, GraphDCEL, Vertex};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LinkVertexStruct {
    id: usize,
    dart: Option<LinkDart>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkVertex(Rc<RefCell<LinkVertexStruct>>);

impl LinkVertex {
    pub fn new(id: usize) -> LinkVertex {
        LinkVertex(Rc::new(RefCell::new(LinkVertexStruct { id, dart: None })))
    }
}

impl Vertex for LinkVertex {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LinkDartStructure {
    target: LinkVertex,
    twin: Option<LinkDart>,
    next: Option<LinkDart>,
    prev: Option<LinkDart>,
    face: Option<LinkFace>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkDart(Rc<RefCell<LinkDartStructure>>);

impl Dart for LinkDart {}

impl LinkDart {
    pub fn new(target: LinkVertex) -> LinkDart {
        LinkDart(Rc::new(RefCell::new(LinkDartStructure {
            target,
            ..Default::default()
        })))
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
        LinkFace(Rc::new(RefCell::new(LinkFaceStructure { dart })))
    }
}

pub struct LinkGraph {
    id_counter: usize,
    vertexes: Vec<LinkVertex>,
    darts: Vec<LinkDart>,
    faces: Vec<LinkFace>,
}

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

impl LinkGraph {
    pub fn new() -> LinkGraph {
        LinkGraph {
            id_counter: 0,
            vertexes: Vec::new(),
            darts: Vec::new(),
            faces: Vec::new(),
        }
    }
    pub fn new_vertex(&mut self) -> LinkVertex {
        let id = self.id_counter;
        self.id_counter += 1;
        let lv = LinkVertex::new(id);
        self.vertexes.push(lv.clone());
        lv
    }
    pub fn new_dart(
        &mut self,
        from: LinkVertex,
        to: LinkVertex,
        prev: Option<LinkDart>,
        next: Option<LinkDart>,
        twin: Option<LinkDart>,
        face: Option<LinkFace>,
    ) -> LinkDart {
        let ld = LinkDart::new(to);
        match prev {
            Some(prev_dart) => {
                prev_dart.0.borrow_mut().next = Some(ld.clone());
                ld.0.borrow_mut().prev = Some(prev_dart);
            }
            None => {}
        };
        match next {
            Some(next_dart) => {
                next_dart.0.borrow_mut().prev = Some(ld.clone());
                ld.0.borrow_mut().next = Some(next_dart);
            }
            None => {}
        };
        match twin {
            Some(twin_dart) => {
                ld.0.borrow_mut().twin = Some(twin_dart);
            }
            None => {}
        };
        match face {
            Some(link_face) => {
                ld.0.borrow_mut().face = Some(link_face);
            }
            None => {}
        }
        self.darts.push(ld.clone());
        from.0.borrow_mut().dart = Some(ld.clone());
        ld
    }
    pub fn new_face(&mut self, dart: LinkDart) -> LinkFace {
        let lv = LinkFace::new(dart.clone());
        dart.0.borrow_mut().face = Some(lv.clone());
        lv
    }
}

impl Drop for LinkGraph {
    fn drop(&mut self) {
        for vertex in &self.vertexes {
            vertex.0.borrow_mut().dart.take();
        }
        for dart in &self.darts {
            let mut dart_ref = dart.0.borrow_mut();
            dart_ref.twin.take();
            dart_ref.next.take();
            dart_ref.prev.take();
            dart_ref.face.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::{graph_dcel::GraphDCEL, link_graph::LinkGraph};

    fn example_graph() -> LinkGraph {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
        let ld1 = lg.new_dart(lv3.clone(), lv1.clone(), None, None, None, None);
        let lf = lg.new_face(ld1.clone());
        let ld2 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld1.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld3 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(ld2.clone()),
            Some(ld1.clone()),
            None,
            Some(lf.clone()),
        );
        let lt1 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            None,
            None,
            Some(ld1.clone()),
            None,
        );
        let lof = lg.new_face(ld1.clone());
        let lt2 = lg.new_dart(
            lv1.clone(),
            lv3.clone(),
            Some(lt1.clone()),
            None,
            Some(ld2.clone()),
            Some(lof.clone()),
        );
        let _lt3 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2.clone()),
            Some(lt1.clone()),
            Some(ld3.clone()),
            Some(lof.clone()),
        );
        lg
    }

    #[test]
    fn test() {
        let graph = example_graph();
        let vertex = graph.vertexes.first().unwrap().clone();
        let dart = graph.dart_vertex(vertex.clone());
        let face = graph.face(dart.clone());
        graph.dart_face(face.clone());
        graph.next(dart.clone());
        graph.prev(dart.clone());
        graph.twin(dart.clone());
        graph.target(dart.clone());
    }
}
