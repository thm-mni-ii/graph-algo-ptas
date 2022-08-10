use crate::data_structure::link_graph::{LinkGraph, LinkVertex};

pub struct Phase2<'a> {
    dcel: &'a mut LinkGraph,
}

impl Phase2<'_> {
    pub fn new(dcel: &mut LinkGraph) -> Phase2 {
        Phase2 { dcel }
    }

    pub fn execute(&mut self) {
        let v0 = self.dcel.new_vertex();
        let v1 = self.dcel.new_vertex();
        let v2 = self.dcel.new_vertex();
        let v3 = self.dcel.new_vertex();

        self.create_face(v0.clone(), v1.clone(), v3.clone());
        self.create_face(v1, v2.clone(), v3.clone());
        self.create_face(v2, v0, v3);
    }

    fn create_face(&mut self, vertex1: LinkVertex, vertex2: LinkVertex, vertex3: LinkVertex) {
        let (d0, t0) = self
            .dcel
            .new_edge(vertex1.clone(), vertex2.clone(), None, None, None, None);
        let f0 = self.dcel.new_face(d0.clone());
        let tf0 = self.dcel.new_face(t0.clone()); // ToDo macht das semantisch sinn?
        let (d1, _t1) = self.dcel.new_edge(
            vertex2,
            vertex3.clone(),
            Some(d0.clone()),
            None,
            Some(f0.clone()),
            Some(tf0.clone()),
        );
        let (_d2, _t2) =
            self.dcel
                .new_edge(vertex3, vertex1, Some(d1), Some(d0), Some(f0), Some(tf0));
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::graph_dcel::GraphDCEL;
    use crate::data_structure::link_graph::LinkGraph;
    use crate::embeding::maximal_planar::phase2::Phase2;

    #[test]
    fn phase_2() {
        let mut dcel = LinkGraph::new();

        Phase2::new(&mut dcel).execute();

        assert_eq!(dcel.get_vertexes().count(), 4);
        assert_eq!(dcel.get_faces().count(), 3);
        // TODO: Test structure
    }
}
