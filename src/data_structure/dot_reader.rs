//! Creates a Pedgraph instance from the Dot Format
//! ```
//! use std::fs::File;
//! use graph_algo_ptas::data_structure::dot_reader::read_graph;
//!
//! let input = "graph g { 0 -- 1 }"; // the input in dot format
//! let g = read_graph(input.to_string()); // create Pedgraph instance
//! ```

use graphviz_parser::ast_nodes::Statement::{Edge, Node};
use graphviz_parser::ast_nodes::{EdgeLHS, EdgeRHS};
use graphviz_parser::DotGraph;
use std::{collections::HashMap, str::FromStr};

use crate::utils::convert::UndirectedGraph;

/// Generates an Pedgraph from a Dot Format string
pub fn read_graph(graph_text: String) -> Option<UndirectedGraph> {
    let graph_ast = DotGraph::from_str(&graph_text);

    match graph_ast {
        Ok(DotGraph::Undirected(g)) => {
            let mut graph: UndirectedGraph = Default::default();
            let mut node_mapper = HashMap::new();

            g.statements.iter().for_each(|s| {
                if let Node(n) = s {
                    let idx = graph.add_node(());
                    node_mapper.insert(n.id.clone(), idx);
                }

                if let Edge(e) = s {
                    if let EdgeLHS::Node(n1) = &e.lhs {
                        if let EdgeRHS::Node(n2) = e.rhs.as_ref() {
                            let a = get_or_create_node(n1.id.clone(), &mut graph, &mut node_mapper);
                            let b = get_or_create_node(n2.id.clone(), &mut graph, &mut node_mapper);
                            graph.add_edge(a, b, ());
                        }
                    }
                }
            });

            Some(graph)
        }
        _ => {
            eprintln!("Invalid input graph");
            None
        }
    }
}

fn get_or_create_node(
    node: String,
    graph: &mut UndirectedGraph,
    node_mapper: &mut HashMap<String, petgraph::stable_graph::NodeIndex>,
) -> petgraph::stable_graph::NodeIndex {
    *node_mapper
        .entry(node)
        .or_insert_with(|| graph.add_node(()))
}

#[cfg(test)]
mod tests {
    use super::read_graph;

    #[test]
    fn small_graph() {
        let input = "graph g { 0[]\n 1[]\n 2[]\n 3[]\n 0 -- 1\n 2 --3\n 2 -- 1 }";
        let g = read_graph(input.to_string());

        assert!(g.is_some());
        let graph = g.unwrap();
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn small_graph_no_nodes() {
        let input = "graph g { 0 -- 1\n 2 --3\n 2 -- 1\n 10 -- 20\n 4 -- 1\n 5 -- 3 }";
        let g = read_graph(input.to_string());

        assert!(g.is_some());
        let graph = g.unwrap();
        assert_eq!(graph.node_count(), 8);
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn digraph() {
        let input = "digraph g { 0 -- 1 }";
        let g = read_graph(input.to_string());

        assert!(g.is_none());
    }

    #[test]
    #[should_panic]
    fn invalid_graph() {
        let input = "graph { 0 -- 1 }";
        read_graph(input.to_string());
    }
}
