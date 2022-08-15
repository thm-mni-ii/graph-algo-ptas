//! Contains example LinkGraphs
use crate::data_structure::link_graph::{LinkDart, LinkGraph, LinkVertex};

/// Returns an example LinkGraph which contains three interconnected circles.
/// The first circle contains one vertex.
/// The second circle contains four vertex.
/// The third circle contains eight vertex.
pub fn three_ring_graph() -> (LinkGraph, Vec<LinkVertex>, Vec<LinkDart>) {
    let mut g = LinkGraph::new();
    let vertexes = (0..13).map(|_| g.new_vertex()).collect::<Vec<_>>();

    let edge_definition = [
        (1, 2, None, None),
        (2, 3, Some(1), None),
        (3, 1, Some(2), None),
        (1, 3, None, Some(3)),
        (3, 4, Some(4), None),
        (4, 1, Some(5), None),
        (1, 4, None, Some(6)),
        (4, 5, Some(7), None),
        (5, 1, Some(8), None),
        (1, 5, None, Some(9)),
        (5, 2, Some(10), None),
        (2, 1, Some(11), Some(1)),
        (2, 6, None, None),
        (6, 7, Some(13), None),
        (7, 2, Some(14), None),
        (2, 7, None, Some(15)),
        (7, 3, Some(16), None),
        (3, 2, Some(17), Some(2)),
        (3, 7, None, Some(17)),
        (7, 8, Some(19), None),
        (8, 3, Some(20), None),
        (3, 8, None, Some(21)),
        (8, 9, Some(22), None),
        (9, 3, Some(23), None),
        (3, 9, None, Some(24)),
        (9, 4, Some(25), None),
        (4, 3, Some(26), Some(5)),
        (4, 9, None, Some(26)),
        (9, 10, Some(28), None),
        (10, 4, Some(29), None),
        (4, 10, None, Some(30)),
        (10, 11, Some(31), None),
        (11, 4, Some(32), None),
        (4, 11, None, Some(33)),
        (11, 5, Some(34), None),
        (5, 4, Some(35), Some(8)),
        (5, 11, None, Some(35)),
        (11, 12, Some(37), None),
        (12, 5, Some(38), None),
        (5, 12, None, Some(39)),
        (12, 13, Some(40), None),
        (13, 5, Some(41), None),
        (5, 13, None, Some(42)),
        (13, 2, Some(43), None),
        (2, 5, Some(44), Some(11)),
        (2, 13, None, Some(44)),
        (13, 6, Some(46), None),
        (6, 2, Some(47), Some(13)),
        (7, 6, None, Some(14)),
        (8, 7, Some(49), Some(20)),
        (9, 8, Some(50), Some(23)),
        (10, 9, Some(51), Some(29)),
        (11, 10, Some(52), Some(32)),
        (12, 11, Some(53), Some(38)),
        (13, 12, Some(54), Some(41)),
        (6, 13, Some(55), Some(47)),
    ];
    let mut edges: Vec<LinkDart> = Vec::with_capacity(edge_definition.len());
    for (from, to, prev, twin) in edge_definition {
        edges.push(g.new_dart(
            vertexes[from - 1].clone(),
            vertexes[to - 1].clone(),
            prev.map(|p| edges[p - 1].clone()),
            None,
            twin.map(|t| edges[t - 1].clone()),
            None,
        ));
    }
    g.auto_face();
    (g, vertexes, edges)
}

#[cfg(test)]
mod test {
    use crate::data_structure::link_graph::example::three_ring_graph;

    #[test]
    pub fn validate_three_ring_graph() {
        let (g, _, _) = three_ring_graph();
        g.validate();
    }
}
