use std::collections::{BTreeSet, HashMap};

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;

use crate::data_structure::graph_dcel::GraphDCEL;
use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkGraphIter, LinkVertex};

use super::index::Embeding;

#[derive(Debug)]
enum StackItem {
    Node(NodeIndex),
    Edge(EdgeIndex),
    Degree(usize),
}

impl StackItem {
    fn unwrap_node(self) -> NodeIndex {
        match self {
            StackItem::Node(node) => node,
            _ => panic!("failed to unwrap node"),
        }
    }
    fn unwrap_edge(self) -> EdgeIndex {
        match self {
            StackItem::Edge(edge) => edge,
            _ => panic!("failed to unwrap edge"),
        }
    }
    fn unwrap_degree(self) -> usize {
        match self {
            StackItem::Degree(degree) => degree,
            _ => panic!("failed to unwrap degree"),
        }
    }
}

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

        Phase1::new(&mut graph, &mut stack).execute();
        //MaximalPlanar::phase_1(&mut graph, &mut stack);
        let dcel = MaximalPlanar::phase_2();
        MaximalPlanar::phase_3(graph, graph_copy, stack, dcel)
    }
}

pub struct Phase1<'a> {
    graph: &'a mut StableGraph<u32, (), Undirected>,
    stack: &'a mut Vec<StackItem>,
    reducible: BTreeSet<NodeIndex>,
}

impl Phase1<'_> {
    fn new<'a>(
        graph: &'a mut StableGraph<u32, (), Undirected>,
        stack: &'a mut Vec<StackItem>,
    ) -> Phase1<'a> {
        let mut phase1 = Phase1 {
            graph,
            stack,
            reducible: Default::default(),
        };

        phase1.reducible = phase1
            .graph
            // TODO: remove clone?
            .clone()
            .node_indices()
            .filter(|n| phase1.is_reducible(*n))
            .collect::<BTreeSet<_>>();

        phase1
    }

    fn execute(&mut self) {
        while self.graph.node_count() > 4 {
            let v = match self.reducible.iter().next() {
                Some(v) => *v,
                None => panic!(), // TODO: improve
            };
            self.reducible.remove(&v);
            let degree = self.graph.edges(v).count();
            let h = self.graph.neighbors(v).collect::<BTreeSet<_>>();

            self.graph.clone().edges(v).for_each(|e| {
                self.stack.push(StackItem::Edge(e.id()));
                self.graph.remove_edge(e.id());
            });

            self.graph.remove_node(v);
            self.stack.push(StackItem::Node(v));

            let new_h = h.clone();
            let w = if degree >= 4 {
                new_h.iter().find(|n| self.find_neighbors(&h, **n))
            } else {
                None
            };

            if degree == 4 {
                let mut x = h.clone();
                self.graph.neighbors(*w.unwrap()).for_each(|n| {
                    x.remove(&n);
                });
                x.remove(w.unwrap());

                self.stack.push(StackItem::Edge(self.graph.add_edge(
                    *w.unwrap(),
                    *x.iter().next().unwrap(),
                    (),
                )));
            }

            if degree == 5 {
                let mut x = h.clone();
                self.graph.neighbors(*w.unwrap()).for_each(|n| {
                    x.remove(&n);
                });
                x.remove(w.unwrap());

                let mut xi = x.iter();

                self.stack.push(StackItem::Edge(self.graph.add_edge(
                    *w.unwrap(),
                    *xi.next().unwrap(),
                    (),
                )));

                self.stack.push(StackItem::Edge(self.graph.add_edge(
                    *w.unwrap(),
                    *xi.next().unwrap(),
                    (),
                )));
            }

            self.update_local(&h);
            self.stack.push(StackItem::Degree(degree))
        }
    }

    fn is_reducible(&mut self, node_idx: NodeIndex) -> bool {
        let count = self.graph.edges(node_idx).count();
        let small_neighbore_count = self.get_small_meighbor_count(node_idx);

        count <= 3
            || count == 4 && small_neighbore_count >= 2
            || count == 5 && small_neighbore_count >= 4
    }

    fn get_small_meighbor_count(&mut self, node_idx: NodeIndex) -> usize {
        self.graph
            .neighbors(node_idx)
            .into_iter()
            .filter(|n| self.graph.edges(*n).count() < 18)
            .count()
    }

    fn find_neighbors(&mut self, h: &BTreeSet<NodeIndex>, node_idx: NodeIndex) -> bool {
        let neighbors = self.graph.neighbors(node_idx);
        let mut count = 0;

        neighbors.for_each(|n| {
            if h.contains(&n) {
                count += 1;
            }
        });

        count == 2
    }

    fn update_local(&mut self, h: &BTreeSet<NodeIndex>) {
        h.iter().for_each(|x| {
            if self.graph.edges(*x).count() < 18
            /* TODO: check if x was small before reduction */
            {
                self.update_reducible(*x);
            }

            // TODO: remove clone?
            self.graph.clone().neighbors(*x).for_each(|n| {
                if self.graph.edges(n).into_iter().count() <= 5 {
                    self.update_reducible(n);
                }
            })
        })
    }

    fn update_reducible(&mut self, node_idx: NodeIndex) {
        let is_reducible = self.is_reducible(node_idx);

        if is_reducible {
            self.reducible.insert(node_idx);
        } else {
            self.reducible.remove(&node_idx);
        }
    }
}

impl MaximalPlanar {
    fn phase_2() -> LinkGraph {
        let mut dcel = LinkGraph::new();

        let v0 = dcel.new_vertex();
        let v1 = dcel.new_vertex();
        let v2 = dcel.new_vertex();
        let v3 = dcel.new_vertex();

        MaximalPlanar::create_face(&mut dcel, v0.clone(), v1.clone(), v3.clone());
        MaximalPlanar::create_face(&mut dcel, v1, v2.clone(), v3.clone());
        MaximalPlanar::create_face(&mut dcel, v2, v0, v3);

        dcel
    }

    fn phase_3(
        graph: StableGraph<u32, (), Undirected>,
        graph_copy: StableGraph<u32, (), Undirected>,
        mut stack: Vec<StackItem>,
        mut dcel: LinkGraph,
    ) -> LinkGraph {
        let mut node_id_mapper = graph
            .node_indices()
            .zip(dcel.get_vertexes())
            .collect::<HashMap<NodeIndex, LinkVertex>>();

        let mut last_dart: Option<LinkDart> = None;
        while let Some(entry) = stack.pop() {
            let k = entry.unwrap_degree();
            let mut create_face = false;
            let (ec, hc) = match k {
                3 => (0, 3),
                4 => (1, 4),
                5 => (2, 5),
                degree => {
                    println!("unexpected degree {:?}", degree);
                    continue;
                }
            };
            let es = MaximalPlanar::pop_edges(&mut stack, ec);
            let v = stack.pop().unwrap().unwrap_node();
            let hs = MaximalPlanar::pop_edges(&mut stack, hc);

            for e in es {
                let (a_node, _) = graph_copy.edge_endpoints(e).unwrap();
                let a_vertex = node_id_mapper.get(&a_node).unwrap().clone();
                dcel.remove_edge(&a_vertex, dcel.dart_vertex(&a_vertex));
            }

            MaximalPlanar::get_or_create_vertex(&mut node_id_mapper, v, &mut dcel);

            for h in hs {
                let (a_node, b_node) = graph_copy.edge_endpoints(h).unwrap();
                let a_vertex = node_id_mapper.get(&a_node).unwrap().clone();
                let b_vertex =
                    MaximalPlanar::get_or_create_vertex(&mut node_id_mapper, b_node, &mut dcel);

                let (new_dart, _) =
                    dcel.new_edge(a_vertex.clone(), b_vertex.clone(), last_dart, None, None);

                if create_face {
                    dcel.new_face(new_dart.clone());
                } else {
                    create_face = true;
                }

                last_dart = Some(new_dart);
            }
        }

        dcel
    }

    fn create_face(
        dcel: &mut LinkGraph,
        vertex1: LinkVertex,
        vertex2: LinkVertex,
        vertex3: LinkVertex,
    ) {
        let d0 = dcel.new_edge(vertex1, vertex2.clone(), None, None, None).0;
        let f0 = dcel.new_face(d0.clone());
        let _d1 = dcel.new_edge(vertex2, vertex3, Some(d0), None, Some(f0)).0;
    }

    fn pop_edges(stack: &mut Vec<StackItem>, count: i32) -> Vec<EdgeIndex> {
        (0..count)
            .map(|_| stack.pop().unwrap().unwrap_edge())
            .collect::<Vec<_>>()
    }

    fn get_or_create_vertex(
        node_id_mapper: &mut HashMap<NodeIndex, LinkVertex>,
        key: NodeIndex,
        dcel: &mut LinkGraph,
    ) -> LinkVertex {
        node_id_mapper
            .entry(key)
            .or_insert_with(|| dcel.new_vertex())
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use petgraph::{
        dot::{Config, Dot},
        stable_graph::StableGraph,
        Undirected,
    };

    use crate::embeding::{index::Embeding, maximal_planar::Phase1};

    use super::MaximalPlanar;
    use crate::data_structure::graph_dcel::GraphDCEL;

    fn k4_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([(0, 1), (1, 2), (2, 0), (1, 3), (2, 3)])
    }

    fn other_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
        ])
    }

    fn large_graph() -> StableGraph<u32, (), Undirected> {
        StableGraph::from_edges([
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 5),
            (5, 1),
        ])
    }

    #[test]
    fn phase_1() {
        let mut graph = other_graph();
        let mut stack = Vec::new();

        Phase1::new(&mut graph, &mut stack).execute();

        print!("{:?}", stack);
        // TODO: test
    }

    #[test]
    fn phase_2() {
        let dcel = MaximalPlanar::phase_2();

        assert_eq!(dcel.get_vertexes().count(), 4);
        assert_eq!(dcel.get_faces().count(), 3);
        // TODO: Test structure
    }

    #[test]
    fn phase_3() {
        let graph = large_graph();
        let mut f = File::create("circle.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::embed(graph);

        dot::render(&dcel, &mut f).unwrap();
        println!("FACE COUNT: {:?}", dcel.get_faces().count());
    }
}
