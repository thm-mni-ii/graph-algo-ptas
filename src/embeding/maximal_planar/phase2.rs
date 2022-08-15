use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};

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

        // Face 0
        let (d0, d1, d2) = self.create_face(v0.clone(), v1.clone(), v3.clone(), None, None, None);

        // Face 1
        let (_, d3, d4) =
            self.create_face(v1.clone(), v0.clone(), v2.clone(), Some(d0), None, None);

        // Face 2
        let (d5, _, _) = self.create_face(v3.clone(), v2.clone(), v0, None, Some(d3), Some(d2));

        // Face 3
        self.create_face(v2, v3, v1, Some(d5), Some(d1), Some(d4));
    }

    fn create_face(
        &mut self,
        v0: LinkVertex,
        v1: LinkVertex,
        v2: LinkVertex,
        t1: Option<LinkDart>,
        t2: Option<LinkDart>,
        t3: Option<LinkDart>,
    ) -> (LinkDart, LinkDart, LinkDart) {
        let d0 = self
            .dcel
            .new_dart(v0.clone(), v1.clone(), None, None, t1, None);
        let f0 = self.dcel.new_face(d0.clone());
        let d1 = self
            .dcel
            .new_dart(v1, v2.clone(), Some(d0.clone()), None, t2, Some(f0.clone()));
        let d2 = self
            .dcel
            .new_dart(v2, v0, Some(d1.clone()), Some(d0.clone()), t3, Some(f0));

        (d0, d1, d2)
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

        dcel.validate();
        assert_eq!(dcel.get_vertexes().count(), 4);
        assert_eq!(dcel.edge_count(), 6);
        assert_eq!(dcel.get_faces().count(), 4);
    }
}
