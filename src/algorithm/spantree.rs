use std::collections::{HashSet, VecDeque};
use crate::data_structure::{
    graph_dcel::GraphDCEL,
    graph_types::{Face, Vertex},
};

fn span(g: &impl GraphDCEL, v: Vertex) -> HashSet<(Vertex, Vertex)> {
    let mut queue = VecDeque::new();
    let mut result = HashSet::new();
    let mut visited = HashSet::new();
    queue.push_back(v);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        /*if visited.contains(&v) {
            continue;
        }*/
        visited.insert(v);
        for n in neighbors(g, v) {
            if !visited.contains(&n) {
                queue.push_back(n);
                result.insert((v, n));
            }
        }
    }
    result
}

fn neighbors(g: &impl GraphDCEL, v: Vertex) -> Vec<Vertex> {
    let mut current_dart = g.dart_vertex(v);
    let first = current_dart;
    let mut current_neighbor = g.target(current_dart);
    let mut result = vec![];
    loop {
        result.push(current_neighbor);
        let twin_dart = g.twin(current_dart);
        current_dart = g.next(twin_dart);
        if current_dart == first {
            break;
        }
        current_neighbor = g.target(g.next(current_dart));
    }
    result
}