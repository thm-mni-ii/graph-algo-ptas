use crate::data_structure::list_graph::{EdgeId, ListGraph, NodeId};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub fn write_as_files(
    graph: &ListGraph,
    node_color_map: &HashMap<NodeId, String>,
    edge_color_map: &HashMap<EdgeId, String>,
    counter: &mut usize,
) {
    let _ = fs::create_dir("graph");
    let mut dot_file = std::fs::File::create(format!("graph/{:04}.dot", counter)).unwrap();
    let mut tup_file = std::fs::File::create(format!("graph/{:04}.tup", counter)).unwrap();
    *counter += 1;
    list_graph_to_dot(graph, node_color_map, edge_color_map, &mut dot_file).unwrap();
    write!(tup_file, "{:?}", graph.all_edges()).unwrap();
}

fn list_graph_to_dot(
    graph: &ListGraph,
    node_color_map: &HashMap<NodeId, String>,
    edge_color_map: &HashMap<EdgeId, String>,
    out: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(out, "graph {{")?;
    for node in graph.node_indexes() {
        if let Some(color) = node_color_map.get(&node) {
            writeln!(
                out,
                "\t {} [label = \"{}\", color = \"{}\"]",
                node, node, color
            )?;
        } else {
            writeln!(out, "\t {} [label = \"{}\"]", node, node)?;
        }
    }
    for edge in graph.edge_indexes() {
        if let Some((from, to)) = graph.edge(edge) {
            if let Some(color) = edge_color_map.get(&edge) {
                writeln!(
                    out,
                    "\t {} -- {} [label = \"{}\", color = \"{}\"]",
                    from, to, edge, color
                )?;
            } else {
                writeln!(out, "\t {} -- {} [label = \"{}\"]", from, to, edge)?;
            }
        }
    }
    writeln!(out, "}}")?;
    Ok(())
}
