use std::collections::HashSet;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};

use crate::data_structure::graph_types::Vertex as VertexType;
use crate::data_structure::link_graph::{LinkDart, LinkFace, LinkGraph, LinkVertex};

use super::index::Embeding;

enum StackItem {
    Node(NodeIndex),
    Edge(EdgeIndex),
    Degree(usize),
}

pub struct MaximalPlanar {}

impl Embeding<LinkVertex, LinkDart, LinkFace, LinkGraph> for MaximalPlanar {
    fn embed(mut graph: Graph<VertexType, (), Undirected>) -> LinkGraph {
        let mut stack = Vec::new();
        MaximalPlanar::phase_1(&mut graph, &mut stack);
        todo!()
    }
}

impl MaximalPlanar {
    fn phase_1(graph: &mut Graph<VertexType, (), Undirected>, stack: &mut Vec<StackItem>) {
        let mut reducible = graph
            .node_indices()
            .filter(|n| MaximalPlanar::is_reducible(graph, *n))
            .collect::<HashSet<_>>();

        while graph.node_count() > 4 {
            let v = match reducible.iter().next() {
                Some(v) => *v,
                None => panic!(), // TODO: improve
            };
            let degree = graph.edges(v).count();
            let h = graph.neighbors(v).collect::<HashSet<_>>();

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

    fn pahase_2(graph: &mut Graph<VertexType, (), Undirected>) -> LinkGraph {
        let mut dcel = LinkGraph::new();

        let v0 = dcel.new_vertex();
        let v1 = dcel.new_vertex();
        let v2 = dcel.new_vertex();
        let v3 = dcel.new_vertex();

        // Face 1
        let d0 = dcel.new_edge(v0.clone(), v1.clone(), None, None, None).0;
        let f0 = dcel.new_face(d0.clone());
        let d1 = dcel
            .new_edge(
                v1.clone(),
                v3.clone(),
                Some(d0.clone()),
                None,
                Some(f0.clone()),
            )
            .0;
        let d2 = dcel
            .new_edge(
                v3.clone(),
                v0.clone(),
                Some(d1),
                Some(d0.clone()),
                Some(f0.clone()),
            )
            .0;

        // Face 2
        let d3 = dcel.new_edge(v1.clone(), v2.clone(), None, None, None).0;
        let f1 = dcel.new_face(d3.clone());
        let d4 = dcel
            .new_edge(
                v2.clone(),
                v3.clone(),
                Some(d3.clone()),
                None,
                Some(f1.clone()),
            )
            .0;
        let d5 = dcel
            .new_edge(
                v3.clone(),
                v1.clone(),
                Some(d4.clone()),
                Some(d3.clone()),
                Some(f1.clone()),
            )
            .0;

        // Face 3
        let d6 = dcel.new_edge(v2.clone(), v0.clone(), None, None, None).0;
        let f3 = dcel.new_face(d6.clone());
        let d7 = dcel
            .new_edge(
                v0.clone(),
                v3.clone(),
                Some(d6.clone()),
                None,
                Some(f3.clone()),
            )
            .0;
        let d8 = dcel
            .new_edge(
                v3.clone(),
                v2.clone(),
                Some(d7.clone()),
                Some(d6.clone()),
                Some(f3.clone()),
            )
            .0;

        dcel
    }

    fn is_reducible(graph: &Graph<VertexType, (), Undirected>, node_idx: NodeIndex) -> bool {
        let count = graph.edges(node_idx).count();
        let small_neighbore_count = MaximalPlanar::get_small_meighbor_count(graph, node_idx);

        count <= 3
            || count == 4 && small_neighbore_count >= 2
            || count == 5 && small_neighbore_count >= 4
    }

    fn get_small_meighbor_count(
        graph: &Graph<VertexType, (), Undirected>,
        node_idx: NodeIndex,
    ) -> usize {
        graph
            .neighbors(node_idx)
            .into_iter()
            .filter(|n| graph.edges(*n).count() < 18)
            .count()
    }

    fn find_neighbors(
        graph: &Graph<VertexType, (), Undirected>,
        h: &HashSet<NodeIndex>,
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
        graph: &Graph<VertexType, (), Undirected>,
        h: &HashSet<NodeIndex>,
        reduciable: &mut HashSet<NodeIndex>,
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
        graph: &Graph<VertexType, (), Undirected>,
        reduciable: &mut HashSet<NodeIndex>,
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
