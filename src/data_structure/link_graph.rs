//! Contains a linked implementation of the DCEL trait
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

macro_rules! impl_ord {
    ($struct:ident) => {
        impl PartialOrd for $struct {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.borrow().id.partial_cmp(&other.0.borrow().id)
            }
        }

        impl Ord for $struct {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.borrow().id.cmp(&other.0.borrow().id)
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

/// A vertex in the LinkGraph
#[derive(Clone, Default, Eq)]
pub struct LinkVertex(Rc<RefCell<LinkVertexStruct>>);

impl LinkVertex {
    fn new(id: usize) -> LinkVertex {
        LinkVertex(Rc::new(RefCell::new(LinkVertexStruct {
            id,
            ..Default::default()
        })))
    }

    /// Returns the id of this LinkVertex
    pub fn get_id(&self) -> usize {
        return self.0.clone().borrow().id;
    }
}

impl_inner_debug!(LinkVertex);
impl_hash_and_eq!(LinkVertex);
impl_ord!(LinkVertex);
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

/// A dart in the LinkGraph
#[derive(Clone, Default, Eq)]
pub struct LinkDart(Rc<RefCell<LinkDartStructure>>);

impl_inner_debug!(LinkDart);
impl_hash_and_eq!(LinkDart);
impl_ord!(LinkDart);
impl Dart for LinkDart {}

impl LinkDart {
    fn new(id: usize, target: LinkVertex) -> LinkDart {
        LinkDart(Rc::new(RefCell::new(LinkDartStructure {
            id,
            target,
            ..Default::default()
        })))
    }

    /// Returns the id of this LinkDart
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

/// A face in the LinkGraph
#[derive(Clone, Default, Eq)]
pub struct LinkFace(Rc<RefCell<LinkFaceStructure>>);

impl_inner_debug!(LinkFace);
impl_hash_and_eq!(LinkFace);
impl_ord!(LinkFace);
impl Face for LinkFace {}

impl LinkFace {
    fn new(id: usize, dart: LinkDart) -> LinkFace {
        LinkFace(Rc::new(RefCell::new(LinkFaceStructure { id, dart })))
    }

    /// Returns the id of this LinkFace
    pub fn get_id(&self) -> usize {
        return self.0.clone().borrow().id;
    }
}

/// A linked implementation of the DCEL trait
pub struct LinkGraph {
    id_counter: usize,
    vertexes: Vec<LinkVertex>,
    darts: Vec<LinkDart>,
    faces: Vec<LinkFace>,
}

/// A helper struct to iterate over the LinkGraph
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

    fn face_vertex_count(&self, face: &LinkFace) -> usize {
        let mut count = 1;
        let dart = self.dart_face(face);
        let mut current_dart = dart.clone();

        loop {
            current_dart = self.next(&current_dart);

            if current_dart == dart {
                break count;
            }

            count += 1;
        }
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
    /// Returns a new empty ListGraph
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
    /// Adds a new vertex to this LinkGraph
    pub fn new_vertex(&mut self) -> LinkVertex {
        let lv = LinkVertex::new(self.next_id());
        self.vertexes.push(lv.clone());
        lv
    }
    /// Adds a new dart to this LinkGraph
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

    /// Adds a new face to this LinkGraph
    pub fn new_face(&mut self, dart: LinkDart) -> LinkFace {
        let lv = LinkFace::new(self.next_id(), dart.clone());
        self.faces.push(lv.clone());
        dart.0.borrow_mut().face = Some(lv.clone());
        lv
    }

    pub fn new_edge(
        &mut self,
        from: LinkVertex,
        to: LinkVertex,
        prev: Option<LinkDart>,
        next: Option<LinkDart>,
        face: Option<LinkFace>,
    ) -> (LinkDart, LinkDart) {
        let dart = self.new_dart(
            from.clone(),
            to.clone(),
            prev.clone(),
            next.clone(),
            None,
            face.clone(),
        );
        let twin = self.new_dart(
            to,
            from,
            next.and_then(|n| n.0.borrow().twin.clone()),
            prev.and_then(|n| n.0.borrow().twin.clone()),
            Some(dart.clone()),
            face,
        );

        (dart, twin)
    }

    fn remove_dart(&mut self, from: &LinkVertex, dart: LinkDart) -> LinkDart {
        let mut dart_ref = dart.0.borrow_mut();
        dart_ref.twin.take();
        dart_ref.next.take();
        dart_ref.prev.take();
        dart_ref.face.take();
        drop(dart_ref);

        if let Some(dart_pos) = self.darts.iter().position(|d| &dart == d) {
            self.darts.remove(dart_pos);
        }
        from.0.borrow_mut().dart.take();
        dart
    }

    pub fn remove_edge(&mut self, from: &LinkVertex, dart: LinkDart) -> (LinkDart, LinkDart) {
        (self.remove_dart(from, dart.clone()), self.remove_dart(from, self.twin(&dart)))
    }
}

impl Default for LinkGraph {
    fn default() -> Self {
        LinkGraph::new()
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
    use std::{cmp::Ordering, collections::HashSet};

    use crate::data_structure::{graph_dcel::GraphDCEL, link_graph::LinkGraph};

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
            Some(lf),
        );
        let lt1 = lg.new_dart(lv2.clone(), lv1.clone(), None, None, Some(ld1), None);
        let lof = lg.new_face(lt1.clone());
        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2,
            None,
            Some(lt1.clone()),
            Some(ld2),
            Some(lof.clone()),
        );
        let _lt3 = lg.new_dart(lv1, lv3, Some(lt1), Some(lt2), Some(ld3), Some(lof));
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
        assert_eq!(twin_2_dart, twin_dart_3);
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

        assert_eq!(graph.get_vertexes().count(), 3);
        assert_eq!(graph.get_darts().count(), 6);
        assert_eq!(graph.get_faces().count(), 2);
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
    fn cmp_test() {
        let graph = example_graph();
        let mut vi = graph.get_vertexes();
        let v1 = vi.next().unwrap();
        let v2 = vi.next().unwrap();
        assert_eq!(v1.partial_cmp(&v2), Some(Ordering::Less));
        assert_eq!(v2.partial_cmp(&v1), Some(Ordering::Greater));
        assert_eq!(v1.cmp(&v2), Ordering::Less);
        assert_eq!(v2.cmp(&v1), Ordering::Greater);
        let mut di = graph.get_vertexes();
        let d1 = di.next().unwrap();
        let d2 = di.next().unwrap();
        assert_eq!(d1.partial_cmp(&d2), Some(Ordering::Less));
        assert_eq!(d2.partial_cmp(&d1), Some(Ordering::Greater));
        assert_eq!(d1.cmp(&d2), Ordering::Less);
        assert_eq!(d2.cmp(&d1), Ordering::Greater);
        let mut fi = graph.get_vertexes();
        let f1 = fi.next().unwrap();
        let f2 = fi.next().unwrap();
        assert_eq!(f1.partial_cmp(&f2), Some(Ordering::Less));
        assert_eq!(f2.partial_cmp(&f1), Some(Ordering::Greater));
        assert_eq!(f1.cmp(&f2), Ordering::Less);
        assert_eq!(f2.cmp(&f1), Ordering::Greater);
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

    #[test]
    fn dart_count() {
        let g = example_graph();

        assert_eq!(g.dart_count(), 6)
    }

    #[test]
    fn edge_count() {
        let g = example_graph();

        assert_eq!(g.edge_count(), 3);
    }

    #[test]
    fn face_count() {
        let g = example_graph();

        assert_eq!(g.face_count(), 2);
    }

    #[test]
    fn face_vertex_count() {
        let g = example_graph();
        let f = g.face(&g.get_darts().next().unwrap());

        assert_eq!(g.face_vertex_count(&f), 3);
    }

    #[test]
    fn neighbors_count() {
        let g = example_graph();

        assert_eq!(g.neighbors_count(&g.get_vertexes().next().unwrap()), 2);
    }
}
