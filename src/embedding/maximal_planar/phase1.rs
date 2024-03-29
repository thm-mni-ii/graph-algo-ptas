//! Implements the first phase of the algorithm.
//! Three different reductions are applied to the graph until it has only four nodes and therefore corresponds to the K4 graph.
//! The reductions are stored on a stack.

use super::stack_item::StackItem;
use crate::utils::convert::UndirectedGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::BTreeSet;

pub struct Phase1<'a> {
    graph: &'a mut UndirectedGraph,
    stack: &'a mut Vec<StackItem>,
    reducible: BTreeSet<NodeIndex>,
}

impl Phase1<'_> {
    pub fn new<'a>(graph: &'a mut UndirectedGraph, stack: &'a mut Vec<StackItem>) -> Phase1<'a> {
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

    pub fn execute(&mut self) {
        while self.graph.node_count() > 4 {
            let v = match self.reducible.iter().next() {
                Some(v) => *v,
                None => panic!(), // TODO: improve
            };
            self.reducible.remove(&v);
            let degree = self.graph.edges(v).count();
            let h = self.graph.neighbors(v).collect::<BTreeSet<_>>();

            self.graph.clone().edges(v).for_each(|e| {
                self.stack
                    .push(StackItem::Edge(self.graph.edge_endpoints(e.id()).unwrap()));
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

            if degree >= 4 {
                let mut x = h.clone();
                self.graph.neighbors(*w.unwrap()).for_each(|n| {
                    x.remove(&n);
                });
                x.remove(w.unwrap());

                let mut xi = x.iter();

                self.add_edge(*w.unwrap(), *xi.next().unwrap());

                if degree == 5 {
                    self.add_edge(*w.unwrap(), *xi.next().unwrap());
                }
            }

            self.update_local(&h);
            self.stack.push(StackItem::Degree(degree))
        }
    }

    fn is_reducible(&mut self, node_idx: NodeIndex) -> bool {
        let count = self.graph.edges(node_idx).count();
        let small_neighbor_count = self.get_small_neighbor_count(node_idx);

        count <= 3
            || count == 4 && small_neighbor_count >= 2
            || count == 5 && small_neighbor_count >= 4
    }

    fn get_small_neighbor_count(&mut self, node_idx: NodeIndex) -> usize {
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

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) {
        self.graph.add_edge(a, b, ());
        self.stack.push(StackItem::Edge((a, b)));
    }
}

#[cfg(test)]
mod tests {
    use petgraph::stable_graph::StableGraph;

    use crate::{embedding::maximal_planar::phase1::Phase1, utils::convert::UndirectedGraph};

    fn other_graph() -> UndirectedGraph {
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

        Phase1::new(&mut graph, &mut stack).execute();

        // TODO: test
    }
}
