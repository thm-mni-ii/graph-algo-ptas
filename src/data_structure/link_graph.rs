use std::{cell::RefCell, cmp::PartialEq, fmt::Debug, hash::Hash, rc::Rc};

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

macro_rules! impl_hash_and_eq {
    ($struct:ident) => {
        impl Hash for $struct {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.borrow().id.hash(state)
            }
        }
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.0.borrow().id == other.0.borrow().id
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

#[derive(Clone, Default, Eq)]
pub struct LinkVertex(Rc<RefCell<LinkVertexStruct>>);

impl LinkVertex {
    pub fn new(id: usize) -> LinkVertex {
        LinkVertex(Rc::new(RefCell::new(LinkVertexStruct {
            id,
            ..Default::default()
        })))
    }

    pub fn get_id(&self) -> usize {
        return self.0.clone().borrow().id;
    }
}

impl_inner_debug!(LinkVertex);
impl_hash_and_eq!(LinkVertex);
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

#[derive(Clone, Default, Eq)]
pub struct LinkDart(Rc<RefCell<LinkDartStructure>>);

impl_inner_debug!(LinkDart);
impl_hash_and_eq!(LinkDart);
impl Dart for LinkDart {}

impl LinkDart {
    pub fn new(id: usize, target: LinkVertex) -> LinkDart {
        LinkDart(Rc::new(RefCell::new(LinkDartStructure {
            id,
            target,
            ..Default::default()
        })))
    }

    pub fn get_id(&self) -> usize {
        return self.0.clone().borrow().id;
    }
}

#[derive(Default)]
struct LinkFaceStructure {
    id: usize,
    dart: LinkDart,
}

impl_non_recursive_eq!(LinkFaceStructure);
impl_non_recursive_debug!(LinkFaceStructure, "LinkFace");

#[derive(Clone, Default, Eq)]
pub struct LinkFace(Rc<RefCell<LinkFaceStructure>>);

impl_inner_debug!(LinkFace);
impl_hash_and_eq!(LinkFace);
impl Face for LinkFace {}

impl LinkFace {
    pub fn new(id: usize, dart: LinkDart) -> LinkFace {
        LinkFace(Rc::new(RefCell::new(LinkFaceStructure { id, dart })))
    }

    pub fn get_id(&self) -> usize {
        return self.0.clone().borrow().id;
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

    fn vertex_count(&self) -> usize {
        self.vertexes.len()
    }

    fn dart_count(&self) -> usize {
        self.darts.len()
    }

    fn edge_count(&self) -> usize {
        self.dart_count() / 2
    }

    fn face_count(&self) -> usize {
        self.faces.len()
    }

    fn neighbors_count(&self, vertex: &LinkVertex) -> usize {
        self.neighbors(vertex).len()
    }

    fn neighbors(&self, vertex: &LinkVertex) -> Vec<LinkVertex> {
        let mut current_dart = self.dart_vertex(vertex);
        let first_dart = current_dart.clone();
        let mut current_neighbor = self.dart_target(&current_dart);
        let mut result = vec![];
        loop {
            result.push(current_neighbor);
            let twin_dart = self.twin(&current_dart);
            current_dart = self.next(&twin_dart);
            if current_dart == first_dart {
                break;
            }
            current_neighbor = self.dart_target(&current_dart);
        }
        result
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

    fn dart_target(&self, dart: &LinkDart) -> LinkVertex {
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
    use std::collections::HashSet;

    use crate::data_structure::{
        graph_dcel::GraphDCEL,
        link_graph::{LinkDart, LinkGraph},
    };

    use super::{LinkFace, LinkVertex};

    fn example_graph() -> LinkGraph {
        let mut lg = LinkGraph::new();
        let lv1 = lg.new_vertex();
        assert_eq!(lv1.get_id(), 0);
        let lv2 = lg.new_vertex();
        assert_eq!(lv2.get_id(), 1);
        let lv3 = lg.new_vertex();
        assert_eq!(lv3.get_id(), 2);
        let ld1 = lg.new_dart(lv1.clone(), lv2.clone(), None, None, None, None);
        assert_eq!(ld1.get_id(), 3);
        let lf = lg.new_face(ld1.clone());
        assert_eq!(lf.get_id(), 4);
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
        let target_vertex = graph.dart_target(&twin_dart);
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

    #[test]
    fn hash_test() {
        let graph = example_graph();
        let mut vs = HashSet::new();
        let mut vi = graph.get_vertexes();
        vs.insert(vi.next().unwrap());
        vs.insert(vi.next().unwrap());
        assert_eq!(vs.len(), 2);
        let mut ds = HashSet::new();
        let mut di = graph.get_vertexes();
        ds.insert(di.next().unwrap());
        ds.insert(di.next().unwrap());
        assert_eq!(ds.len(), 2);
        let mut fs = HashSet::new();
        let mut fi = graph.get_vertexes();
        fs.insert(fi.next().unwrap());
        fs.insert(fi.next().unwrap());
        assert_eq!(fs.len(), 2);
    }

    #[test]
    fn eq_test() {
        let graph = example_graph();
        let mut vi = graph.get_vertexes();
        let v1 = vi.next().unwrap();
        let v2 = vi.next().unwrap();
        assert_eq!(v1, v1);
        assert_ne!(v1, v2);
        let mut di = graph.get_vertexes();
        let d1 = di.next().unwrap();
        let d2 = di.next().unwrap();
        assert_eq!(d1, d1);
        assert_ne!(d1, d2);
        let mut fi = graph.get_vertexes();
        let f1 = fi.next().unwrap();
        let f2 = fi.next().unwrap();
        assert_eq!(f1, f1);
        assert_ne!(f1, f2);
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
            Some(ld1),
            Some(lf),
        );

        assert_eq!(lg.neighbors(&lv1), vec![lv2.clone()]);
        assert_eq!(lg.neighbors(&lv2), vec![lv1]);
    }

    #[test]
    fn neighbors_triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(), None, None, None, None);
        let lf = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv0.clone(),
            Some(ld1.clone()),
            Some(ld0.clone()),
            None,
            Some(lf),
        );

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(), None, None, Some(ld0), None);
        let lof = lg.new_face(lt0.clone());
        let lt2 = lg.new_dart(
            lv0.clone(),
            lv2.clone(),
            Some(lt0.clone()),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            Some(lt0),
            Some(ld1),
            Some(lof),
        );

        assert_eq!(lg.neighbors(&lv0), vec![lv2.clone(), lv1.clone()]);
        assert_eq!(lg.neighbors(&lv1), vec![lv0.clone(), lv2.clone()]);
        assert_eq!(lg.neighbors(&lv2), vec![lv1, lv0]);
    }
}
