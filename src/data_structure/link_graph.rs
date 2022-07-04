use std::borrow::Borrow;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::graph_dcel::{Dart, Face, GraphDCEL, Vertex};

macro_rules! impl_non_recursive_eq {
    ($struct:ident) => {
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for $struct {}
    };
}

macro_rules! impl_non_recursive_debug {
    ($struct:ident, $name:expr) => {
        impl Debug for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct($name).field("id", &self.id).finish()
            }
        }
    };
}

macro_rules! impl_inner_debug {
    ($struct:ident) => {
        impl Debug for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.borrow().fmt(f)
            }
        }
    };
}

#[derive(Default)]
struct LinkVertexStruct {
    id: usize,
    dart: Option<LinkDart>,
}

impl_non_recursive_eq!(LinkVertexStruct);
impl_non_recursive_debug!(LinkVertexStruct, "LinkVertex");

#[derive(Clone, Default, Eq, PartialEq)]
pub struct LinkVertex(Rc<RefCell<LinkVertexStruct>>);

impl LinkVertex {
    pub fn new(id: usize) -> LinkVertex {
        LinkVertex(Rc::new(RefCell::new(LinkVertexStruct {
            id,
            ..Default::default()
        })))
    }
}

impl_inner_debug!(LinkVertex);
impl Vertex for LinkVertex {}

#[derive(Default)]
struct LinkDartStructure {
    id: usize,
    target: LinkVertex,
    twin: Option<LinkDart>,
    next: Option<LinkDart>,
    prev: Option<LinkDart>,
    face: Option<LinkFace>,
}

impl_non_recursive_eq!(LinkDartStructure);
impl Debug for LinkDartStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkDart")
            .field("id", &self.id)
            .field(
                "next_id",
                &self.next.as_ref().map(|next| next.0.borrow().id),
            )
            .field(
                "prev_id",
                &self.prev.as_ref().map(|prev| prev.0.borrow().id),
            )
            .field(
                "twin_id",
                &self.twin.as_ref().map(|twin| twin.0.borrow().id),
            )
            .finish()
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct LinkDart(Rc<RefCell<LinkDartStructure>>);

impl_inner_debug!(LinkDart);
impl Dart for LinkDart {
    fn get_source(&self) -> Dart impl {
        todo!()
    }

    fn get_target(&self) -> LinkVertex {
        todo!()
    }
}

impl LinkDart {
    pub fn new(id: usize, target: LinkVertex) -> LinkDart {
        LinkDart(Rc::new(RefCell::new(LinkDartStructure {
            id,
            target,
            ..Default::default()
        })))
    }
}

#[derive(Default)]
struct LinkFaceStructure {
    id: usize,
    dart: LinkDart,
}

impl_non_recursive_eq!(LinkFaceStructure);
impl_non_recursive_debug!(LinkFaceStructure, "LinkFace");

#[derive(Clone, Default, Eq, PartialEq)]
pub struct LinkFace(Rc<RefCell<LinkFaceStructure>>);

impl_inner_debug!(LinkFace);
impl Face for LinkFace {}

impl LinkFace {
    pub fn new(id: usize, dart: LinkDart) -> LinkFace {
        LinkFace(Rc::new(RefCell::new(LinkFaceStructure { id, dart })))
    }
}

pub struct LinkGraph {
    id_counter: usize,
    vertexes: Vec<LinkVertex>,
    darts: Vec<LinkDart>,
    faces: Vec<LinkFace>,
}

pub struct LinkGraphIter<T: Clone> {
    counter: usize,
    inner: Vec<T>,
}

impl<T: Clone> LinkGraphIter<T> {
    fn new(inner: Vec<T>) -> LinkGraphIter<T> {
        LinkGraphIter { counter: 0, inner }
    }
}

impl<T: Clone> Iterator for LinkGraphIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.counter;
        self.counter += 1;
        self.inner.get(i).cloned()
    }
}

impl
    GraphDCEL<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
    > for LinkGraph
{
    fn get_vertexes(&self) -> LinkGraphIter<LinkVertex> {
        LinkGraphIter::new(self.vertexes.clone())
    }
    fn get_darts(&self) -> LinkGraphIter<LinkDart> {
        LinkGraphIter::new(self.darts.clone())
    }
    fn get_faces(&self) -> LinkGraphIter<LinkFace> {
        LinkGraphIter::new(self.faces.clone())
    }

    fn dart_vertex(&self, vertex: &LinkVertex) -> LinkDart {
        vertex.0.borrow().dart.clone().unwrap()
    }

    fn dart_face(&self, face: &LinkFace) -> LinkDart {
        face.0.borrow().dart.clone()
    }

    fn twin(&self, dart: &LinkDart) -> LinkDart {
        dart.0.borrow().twin.clone().unwrap()
    }

    fn target(&self, dart: &LinkDart) -> LinkVertex {
        dart.0.borrow().target.clone()
    }

    fn face(&self, dart: &LinkDart) -> LinkFace {
        dart.0.borrow().face.clone().unwrap()
    }

    fn next(&self, current: &LinkDart) -> LinkDart {
        current.0.borrow().next.clone().unwrap()
    }

    fn prev(&self, current: &LinkDart) -> LinkDart {
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
    fn next_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }
    pub fn new_vertex(&mut self) -> LinkVertex {
        let lv = LinkVertex::new(self.next_id());
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
        let ld = LinkDart::new(self.next_id(), to);
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
                twin_dart.0.borrow_mut().twin = Some(ld.clone());
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
        let lv = LinkFace::new(self.next_id(), dart.clone());
        self.faces.push(lv.clone());
        dart.0.borrow_mut().face = Some(lv.clone());
        lv
    }

    pub fn get_vertices(&self) -> &[LinkVertex] {
        return self.vertexes.borrow();
    }

    pub fn get_darts(&self) -> &[LinkDart] {
        return self.darts.borrow();
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
    use crate::data_structure::{
        graph_dcel::GraphDCEL,
        link_graph::{LinkDart, LinkGraph},
    };

    use super::{LinkFace, LinkVertex};

    fn example_graph() -> LinkGraph {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        let lf = lg.new_face(ld1.clone());
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(ld1.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld3 = lg.new_dart(
            lv3.clone(),
            lv1.clone(),
            Some(ld2.clone()),
            Some(ld1.clone()),
            None,
            Some(lf.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            None,
            None,
            Some(ld1.clone()),
            None,
        );
        let lof = lg.new_face(lt1.clone());
        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(lt1.clone()),
            None,
            Some(ld2.clone()),
            Some(lof.clone()),
        );
        let _lt3 = lg.new_dart(
            lv1.clone(),
            lv3.clone(),
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
        let dart = graph.dart_vertex(&vertex);
        let dart_2 = graph.next(&dart);
        let dart_3 = graph.next(&dart_2);
        let dart_4 = graph.next(&dart_3);
        assert_eq!(dart, dart_4);
        let twin_dart = graph.twin(&dart);
        let twin_back_dart = graph.twin(&twin_dart);
        assert_eq!(twin_back_dart, dart);
        let twin_2_dart = graph.next(&twin_dart);
        let twin_dart_3 = graph.twin(&dart_3);
        assert_ne!(twin_2_dart, twin_dart_3);
        let face = graph.face(&dart);
        graph.dart_face(&face);
        let prev_dart = graph.prev(&dart);
        assert_eq!(prev_dart, dart_3);
        let target_vertex = graph.target(&twin_dart);
        assert_eq!(target_vertex, vertex);
        format!("{:?}", vertex);
        format!("{:?}", dart);
        format!("{:?}", face);
    }

    #[test]
    fn iter_test() {
        let graph = example_graph();
        let vertexes: Vec<LinkVertex> = graph.get_vertexes().collect();
        let darts: Vec<LinkDart> = graph.get_darts().collect();
        let faces: Vec<LinkFace> = graph.get_faces().collect();
        assert_eq!(vertexes.len(), 3);
        assert_eq!(darts.len(), 6);
        assert_eq!(faces.len(), 2);
    }
}
