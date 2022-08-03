use std::borrow::BorrowMut;
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

        MaximalPlanar::phase_1(&mut graph, &mut stack);
        let dcel = MaximalPlanar::phase_2();
        MaximalPlanar::phase_3(graph, graph_copy, stack, dcel)
    }
}

impl MaximalPlanar {
    fn phase_1(graph: &mut StableGraph<u32, (), Undirected>, stack: &mut Vec<StackItem>) {
        let mut reducible = graph
            .node_indices()
            .filter(|n| MaximalPlanar::is_reducible(graph, *n))
            .collect::<BTreeSet<_>>();

        while graph.node_count() > 4 {
            let v = match reducible.iter().next() {
                Some(v) => *v,
                None => panic!(), // TODO: improve
            };
            let degree = graph.edges(v).count();
            let h = graph.neighbors(v).collect::<BTreeSet<_>>();

            graph.clone().edges(v).for_each(|e| {
                stack.push(StackItem::Edge(e.id()));
                graph.remove_edge(e.id());
            });

            graph.remove_node(v);
            stack.push(StackItem::Node(v));

            let new_h = h.clone();
            let w = if degree >= 4 {
                new_h
                    .iter()
                    .find(|n| MaximalPlanar::find_neighbors(graph, &h, **n))
            } else {
                None
            };

            if degree == 4 {
                let mut x = h.clone();
                graph.neighbors(*w.unwrap()).for_each(|n| {
                    x.remove(&n);
                });
                x.remove(w.unwrap());

                stack.push(StackItem::Edge(graph.add_edge(
                    *w.unwrap(),
                    *x.iter().next().unwrap(),
                    (),
                )));
            }

            if degree == 5 {
                let mut x = h.clone();
                graph.neighbors(*w.unwrap()).for_each(|n| {
                    x.remove(&n);
                });
                x.remove(w.unwrap());

                let mut xi = x.iter();

                stack.push(StackItem::Edge(graph.add_edge(
                    *w.unwrap(),
                    *xi.next().unwrap(),
                    (),
                )));

                stack.push(StackItem::Edge(graph.add_edge(
                    *w.unwrap(),
                    *xi.next().unwrap(),
                    (),
                )));
            }

            MaximalPlanar::update_local(graph, &h, &mut reducible);
            stack.push(StackItem::Degree(degree))
        }
    }

    fn phase_2() -> LinkGraph {
        let mut dcel = LinkGraph::new();

        let v0 = dcel.new_vertex();
        let v1 = dcel.new_vertex();
        let v2 = dcel.new_vertex();
        let v3 = dcel.new_vertex();

        // Face 1
        let d0 = dcel.new_edge(v0.clone(), v1.clone(), None, None, None).0;
        let f0 = dcel.new_face(d0.clone());
        let _d1 = dcel
            .new_edge(
                v1.clone(),
                v3.clone(),
                Some(d0.clone()),
                None,
                Some(f0.clone()),
            )
            .0;

        // Face 2
        let d3 = dcel.new_edge(v1.clone(), v2.clone(), None, None, None).0;
        let f1 = dcel.new_face(d3.clone());
        let _d4 = dcel
            .new_edge(
                v2.clone(),
                v3.clone(),
                Some(d3.clone()),
                None,
                Some(f1.clone()),
            )
            .0;

        // Face 3
        let d6 = dcel.new_edge(v2.clone(), v0.clone(), None, None, None).0;
        let f3 = dcel.new_face(d6.clone());
        let _d7 = dcel
            .new_edge(
                v0.clone(),
                v3.clone(),
                Some(d6.clone()),
                None,
                Some(f3.clone()),
            )
            .0;

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

            let (ec, hc) = match k {
                3 => (0, 3),
                4 => (1, 4),
                5 => (2, 5),
                degree => {
                    println!("unexpected degree {:?}", degree);
                    continue;
                }
            };
            let es = (0..ec)
                .map(|_| stack.pop().unwrap().unwrap_edge())
                .collect::<Vec<_>>();
            let v = stack.pop().unwrap().unwrap_node();
            let hs = (0..hc)
                .map(|_| stack.pop().unwrap().unwrap_edge())
                .collect::<Vec<_>>();

            for e in es {
                let (a_node, b_node) = graph_copy.edge_endpoints(e).unwrap();
                let a_vertex = node_id_mapper.get(&a_node).unwrap().clone();
                let b_vertex = node_id_mapper.get(&b_node).unwrap().clone();
                let (new_dart, _) = dcel.new_edge(a_vertex, b_vertex, None, None, None);
                last_dart = Some(new_dart);
            }
            node_id_mapper.entry(v).or_insert_with(|| dcel.new_vertex());
            let mut insert = false;
            for h in hs {
                let (a_node, b_node) = graph_copy.edge_endpoints(h).unwrap();
                let a_vertex = node_id_mapper.get(&a_node).unwrap().clone();
                let b_vertex = node_id_mapper
                    .entry(b_node)
                    .or_insert_with(|| dcel.new_vertex())
                    .clone();

                let (new_dart, _) =
                    dcel.new_edge(a_vertex.clone(), b_vertex.clone(), last_dart, None, None);
                
                if !insert {
                    insert = true;
                } else {
                    dcel.new_face(new_dart.clone());
                }
    
                last_dart = Some(new_dart);
            }
        }

        dcel
    }

    fn is_reducible(graph: &StableGraph<u32, (), Undirected>, node_idx: NodeIndex) -> bool {
        let count = graph.edges(node_idx).count();
        let small_neighbore_count = MaximalPlanar::get_small_meighbor_count(graph, node_idx);

        count <= 3
            || count == 4 && small_neighbore_count >= 2
            || count == 5 && small_neighbore_count >= 4
    }

    fn get_small_meighbor_count(
        graph: &StableGraph<u32, (), Undirected>,
        node_idx: NodeIndex,
    ) -> usize {
        graph
            .neighbors(node_idx)
            .into_iter()
            .filter(|n| graph.edges(*n).count() < 18)
            .count()
    }

    fn find_neighbors(
        graph: &StableGraph<u32, (), Undirected>,
        h: &BTreeSet<NodeIndex>,
        node_idx: NodeIndex,
    ) -> bool {
        let neighbors = graph.neighbors(node_idx);
        let mut count = 0;

        neighbors.for_each(|n| {
            if h.contains(&n) {
                count += 1;
            }
        });

        count == 2
    }

    fn update_local(
        graph: &StableGraph<u32, (), Undirected>,
        h: &BTreeSet<NodeIndex>,
        reduciable: &mut BTreeSet<NodeIndex>,
    ) {
        h.iter().for_each(|x| {
            if graph.edges(*x).count() < 18
            /* TODO: check if x was small before reduction */
            {
                MaximalPlanar::update_reducible(graph, reduciable, *x);
            }

            graph.neighbors(*x).for_each(|n| {
                if graph.edges(n).into_iter().count() <= 5 {
                    MaximalPlanar::update_reducible(graph, reduciable, n);
                }
            })
        })
    }

    fn update_reducible(
        graph: &StableGraph<u32, (), Undirected>,
        reduciable: &mut BTreeSet<NodeIndex>,
        node_idx: NodeIndex,
    ) {
        let is_reducible = MaximalPlanar::is_reducible(graph, node_idx);

        if is_reducible {
            reduciable.insert(node_idx);
        } else {
            reduciable.remove(&node_idx);
        }
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

    use crate::embeding::index::Embeding;

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

    #[test]
    fn phase_1() {
        let mut graph = other_graph();
        let mut stack = Vec::new();

        MaximalPlanar::phase_1(&mut graph, &mut stack);

        print!("{:?}", stack);
        // TODO: test
    }

    #[test]
    fn phase_2() {
        let mut graph = other_graph();
        let mut f = File::create("phase_2.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::phase_2();

        dot::render(&dcel, &mut f).unwrap()
        // TODO: test
    }

    #[test]
    fn phase_3() {
        let graph = other_graph();
        let mut f = File::create("circle.dot").unwrap();

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let dcel = MaximalPlanar::embed(graph);

        dot::render(&dcel, &mut f).unwrap();
        println!("FACE COUNT: {:?}", dcel.get_faces().count());
    }
}
