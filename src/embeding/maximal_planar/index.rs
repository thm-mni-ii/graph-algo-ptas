use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};
use crate::embeding::index::Embeding;

use super::phase1::Phase1;
use super::phase2::Phase2;
use super::phase3::Phase3;

pub struct MaximalPlanar {}

impl
    Embeding<
        LinkVertex,
        LinkDart,
        LinkFace,
        LinkGraphIter<LinkVertex>,
        LinkGraphIter<LinkDart>,
        LinkGraphIter<LinkFace>,
        LinkGraph,
    > for MaximalPlanar
{
    fn embed(mut graph: StableGraph<u32, (), Undirected>) -> LinkGraph {
        let graph_copy = graph.clone();
        let mut stack = Vec::new();
        let mut dcel = LinkGraph::new();

        Phase1::new(&mut graph, &mut stack).execute();
        Phase2::new(&mut dcel).execute();
        Phase3::new(graph, graph_copy, &mut stack, &mut dcel).execute();

        dcel
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use petgraph::dot::{Config, Dot};

    use crate::{
        embeding::{index::Embeding, maximal_planar::index::MaximalPlanar},
        generation::planar::generate,
    };

    use crate::data_structure::graph_dcel::GraphDCEL;

    #[test]
    fn embend() {
        let graph = generate(200).to_pet_graph();
        let mut f = File::create("circle.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::embed(graph);

        dot::render(&dcel, &mut f).unwrap();
        println!("FACE COUNT: {:?}", dcel.get_faces().count());
    }
}
