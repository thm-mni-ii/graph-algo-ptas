//! Creates a Pedgraph instance from the Dot Format
//! ```
//! use std::fs::File;
//! use graph_algo_ptas::dot_reader::read_graph;
//!
//! let g = "graph g { 0 -- 1 }" // the input in dot format
//! let g = read_graph(input); // create Pedgraph instance
//! ```

use graphviz_parser::ast_nodes::Statement::{Edge, Node};
use graphviz_parser::ast_nodes::{EdgeLHS, EdgeRHS};
use graphviz_parser::DotGraph;
use petgraph::stable_graph::DefaultIx;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;
use std::{collections::HashMap, str::FromStr};

/// Generates an Pedgraph from a Dot Format string
pub fn read_graph(graph_text: String) -> Option<StableGraph<u32, (), Undirected, DefaultIx>> {
    let graph_ast = DotGraph::from_str(&graph_text);

    match graph_ast {
        Ok(DotGraph::Undirected(g)) => {
            let mut graph: StableGraph<u32, (), Undirected, DefaultIx> = Default::default();
            let mut node_mapper = HashMap::new();

            g.statements.iter().for_each(|s| {
                if let Node(n) = s {
                    let idx = graph.add_node(0);
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
    graph: &mut StableGraph<u32, (), Undirected, DefaultIx>,
    node_mapper: &mut HashMap<String, petgraph::stable_graph::NodeIndex>,
) -> petgraph::stable_graph::NodeIndex {
    *node_mapper.entry(node).or_insert_with(|| graph.add_node(0))
}
