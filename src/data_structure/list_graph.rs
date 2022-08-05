pub type EdgeId = usize;
pub type NodeId = usize;

pub struct ListGraph {
    nodes: Vec<Vec<(NodeId, EdgeId, bool)>>,
    edges: Vec<(NodeId, NodeId, bool)>,
}

impl ListGraph {
    pub fn new() -> ListGraph {
        ListGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn from_edges<'a>(edges: impl Iterator<Item = &'a (NodeId, NodeId)>) -> ListGraph {
        let mut graph = ListGraph::new();
        for (from, to) in edges {
            graph.add_edge(*from, *to, None, None);
        }
        graph
    }

    pub fn k4() -> ListGraph {
        ListGraph::from_edges_node_list(
            [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)].iter(),
            [
                [0, 2, 1].as_slice(),
                [3, 4, 0].as_slice(),
                [1, 5, 3].as_slice(),
                [2, 4, 5].as_slice(),
            ]
            .iter(),
        )
    }

    pub fn from_edges_node_list<'a, 'b>(
        edges: impl Iterator<Item = &'a (NodeId, NodeId)>,
        nodes: impl Iterator<Item = &'b &'b [EdgeId]>,
    ) -> ListGraph {
        let mut graph = ListGraph::from_edges(edges);
        for (i, node) in nodes.enumerate() {
            graph.nodes[i] = node
                .iter()
                .map(|edge_id| {
                    *graph.nodes[i]
                        .iter()
                        .find(|(_, inner_edge_id, _)| edge_id == inner_edge_id)
                        .unwrap_or_else(|| panic!("edge {} not adjacent to node {}", edge_id, i))
                })
                .collect()
        }
        graph
    }

    fn add_dart(&mut self, from: NodeId, to: NodeId, edge_id: EdgeId, after: Option<EdgeId>) {
        if self.nodes.len() < from + 1 {
            self.nodes.resize_with(from + 1, Vec::new);
        }

        let current_pos = self.nodes[from]
            .iter()
            .position(|(_, curr, _)| edge_id == *curr);

        match after {
            Some(after) => {
                if after == edge_id {
                    return;
                }
                if let Some(current_pos) = current_pos {
                    self.nodes[from].remove(current_pos);
                }
                let new_pos = self.nodes[from]
                    .iter()
                    .position(|(_, curr, active)| *active && after == *curr)
                    .expect("after must be in the edge list");
                self.nodes[from].insert(new_pos + 1, (to, edge_id, true));
            }
            None => {
                if let Some(current_pos) = current_pos {
                    self.nodes[from][current_pos].2 = true;
                } else {
                    self.nodes[from].push((to, edge_id, true))
                }
            }
        }
    }

    fn remove_dart(&mut self, from: NodeId, edge_id: EdgeId) {
        if let Some((_, _, active)) = self.nodes[from]
            .iter_mut()
            .find(|(_, item_id, active)| *active && &edge_id == item_id)
        {
            *active = false;
        }
    }

    fn cyclic_incident_rel(&self, edge: EdgeId, node: NodeId, relative: isize) -> Option<EdgeId> {
        self.nodes.get(node).and_then(|other_nodes| {
            other_nodes
                .iter()
                .position(|(_, item_edge, active)| *active && &edge == item_edge)
                .and_then(|index| {
                    let mut new_index = index;
                    let edge;
                    loop {
                        new_index = (other_nodes.len() as isize + (new_index as isize + relative))
                            as usize
                            % other_nodes.len();
                        edge = match other_nodes.get(new_index) {
                            Some((_, _, false)) => continue,
                            Some((_, eid, true)) => Some(*eid),
                            _ => None,
                        };
                        break;
                    }
                    edge
                })
        })
    }

    fn get_edge_id(&self, from: NodeId, to: NodeId, allow_inactive: bool) -> Option<EdgeId> {
        self.nodes.get(from).and_then(|other_nodes| {
            other_nodes
                .iter()
                .find(|(other_node, _, active)| (allow_inactive || *active) && &to == other_node)
                .map(|(_, id, _)| *id)
        })
    }

    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        after_from: Option<EdgeId>,
        after_to: Option<EdgeId>,
    ) -> EdgeId {
        if from == to {
            panic!("self edges are not supported")
        }
        let edge_id = match self.get_edge_id(from, to, true) {
            Some(edge_id) => edge_id,
            _ => {
                self.edges.push((from, to, true));
                self.edges.len() - 1
            }
        };
        self.add_dart(from, to, edge_id, after_from);
        self.add_dart(to, from, edge_id, after_to);
        edge_id
    }

    pub fn remove_edge(&mut self, edge_id: EdgeId) {
        if let Some((from_ref, to_ref, active)) = self.edges.get_mut(edge_id) {
            if *active {
                let from = *from_ref;
                let to = *to_ref;
                *active = false;
                self.remove_dart(from, edge_id);
                self.remove_dart(to, edge_id);
            }
        }
    }

    pub fn node_indexes(&self) -> NodeIndexIter {
        IndexIter {
            i: 0,
            entries: &self.nodes,
            validation_fn: |_| true,
        }
    }

    pub fn edge_indexes(&self) -> EdgeIndexIter {
        IndexIter {
            i: 0,
            entries: &self.edges,
            validation_fn: |(_, _, active)| *active,
        }
    }

    pub fn edge(&self, edge_id: EdgeId) -> Option<(NodeId, NodeId)> {
        self.edges
            .get(edge_id)
            .and_then(|(from, to, active)| if *active { Some((*from, *to)) } else { None })
    }

    pub fn edges(&self, node_id: NodeId) -> Option<Vec<EdgeId>> {
        self.nodes.get(node_id).map(|edges| {
            edges
                .iter()
                .filter(|(_, _, active)| *active)
                .map(|(_, edge_id, _)| *edge_id)
                .collect()
        })
    }

    pub fn neighbors(&self, node_id: NodeId) -> Option<Vec<NodeId>> {
        self.nodes.get(node_id).map(|edges| {
            edges
                .iter()
                .filter(|(_, _, active)| *active)
                .map(|(node_id, _, _)| *node_id)
                .collect()
        })
    }

    pub fn cyclic_incident_succ(&self, edge: EdgeId, node: NodeId) -> Option<EdgeId> {
        self.cyclic_incident_rel(edge, node, 1)
    }

    pub fn cyclic_incident_prev(&self, edge: EdgeId, node: NodeId) -> Option<EdgeId> {
        self.cyclic_incident_rel(edge, node, -1)
    }

    pub fn opposite(&self, node: NodeId, edge: EdgeId) -> Option<EdgeId> {
        self.edges.get(edge).and_then(|(a_node, b_node, active)| {
            if *active {
                if node == *a_node {
                    Some(*b_node)
                } else if node == *b_node {
                    Some(*a_node)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn all_edges(&self) -> Vec<(NodeId, NodeId)> {
        return self
            .edges
            .iter()
            .filter(|(_, _, active)| *active)
            .map(|(from, to, _)| (*from, *to))
            .collect();
    }

    pub fn new_vertex(&mut self) -> NodeId {
        let new_len = self.nodes.len() + 1;
        self.nodes.resize_with(new_len, Vec::new);
        new_len - 1
    }
}

type NodeIndexIter<'a> =
    IndexIter<'a, Vec<(NodeId, EdgeId, bool)>, fn(&Vec<(NodeId, EdgeId, bool)>) -> bool>;
type EdgeIndexIter<'a> = IndexIter<'a, (NodeId, NodeId, bool), fn(&(NodeId, NodeId, bool)) -> bool>;

pub struct IndexIter<'a, T, VF: Fn(&T) -> bool> {
    i: usize,
    entries: &'a [T],
    validation_fn: VF,
}

impl<'a, T, VF: Fn(&T) -> bool> Iterator for IndexIter<'a, T, VF> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.i < self.entries.len() {
                let i = self.i;
                self.i += 1;
                if (self.validation_fn)(&self.entries[i]) {
                    return Some(i);
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListGraph;

    const K4_EDGE_LIST: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

    fn k4_graph() -> ListGraph {
        ListGraph::from_edges(K4_EDGE_LIST.iter())
    }

    #[test]
    fn test_list_graph_counts() {
        let graph = k4_graph();

        assert_eq!(graph.node_indexes().count(), 4);
        assert_eq!(graph.edge_indexes().count(), 6);
    }

    #[test]
    fn test_list_graph_edges() {
        let graph = k4_graph();

        assert_eq!(graph.edges(0), Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_list_graph_all_edges() {
        let graph = k4_graph();

        assert_eq!(graph.all_edges(), K4_EDGE_LIST);
    }

    #[test]
    fn test_list_graph_new_vertex() {
        let mut graph = k4_graph();

        assert_eq!(graph.new_vertex(), 4);
    }

    #[test]
    fn test_list_graph_opposite() {
        let graph = k4_graph();
        assert_eq!(graph.opposite(0, 0), Some(1));
        assert_eq!(graph.opposite(1, 0), Some(0));
        assert_eq!(graph.opposite(0, 1), Some(2));
        assert_eq!(graph.opposite(2, 1), Some(0));
    }

    #[test]
    fn test_list_graph_cyclic_incident_succ() {
        let graph = k4_graph();
        let a = graph.cyclic_incident_succ(0, 0).unwrap();
        let b = graph.cyclic_incident_succ(a, 0).unwrap();
        let c = graph.cyclic_incident_succ(b, 0).unwrap();
        let d = graph.cyclic_incident_succ(c, 0).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 0);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_list_graph_cyclic_incident_prev() {
        let graph = k4_graph();
        let a = graph.cyclic_incident_prev(0, 0).unwrap();
        let b = graph.cyclic_incident_prev(a, 0).unwrap();
        let c = graph.cyclic_incident_prev(b, 0).unwrap();
        let d = graph.cyclic_incident_prev(c, 0).unwrap();
        assert_eq!(a, 2);
        assert_eq!(b, 1);
        assert_eq!(c, 0);
        assert_eq!(d, 2);
    }

    fn k4_p1() -> ListGraph {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        graph.add_edge(1, new, None, None);
        graph.add_edge(2, new, None, None);
        graph.add_edge(3, new, None, None);
        graph
    }

    #[test]
    fn test_list_graph_add_connected_node() {
        let graph = k4_p1();
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3]));
        assert_eq!(graph.neighbors(1), Some(vec![0, 2, 3, 4]));
        assert_eq!(graph.neighbors(2), Some(vec![0, 1, 3, 4]));
        assert_eq!(graph.neighbors(3), Some(vec![0, 1, 2, 4]));
    }

    #[test]
    fn test_list_graph_remove_connected_node() {
        let mut graph = k4_p1();
        graph.remove_edge(graph.get_edge_id(1, 4, false).unwrap());
        graph.remove_edge(graph.get_edge_id(2, 4, false).unwrap());
        graph.remove_edge(graph.get_edge_id(3, 4, false).unwrap());
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3]));
        assert_eq!(graph.neighbors(1), Some(vec![0, 2, 3]));
        assert_eq!(graph.neighbors(2), Some(vec![0, 1, 3]));
        assert_eq!(graph.neighbors(3), Some(vec![0, 1, 2]));
        assert_eq!(graph.neighbors(4), Some(vec![]));
    }

    #[test]
    fn test_list_graph_multi_connected_node() {
        let mut graph = k4_graph();
        let a_edge = graph.add_edge(0, 4, None, None);
        let b_edge = graph.add_edge(4, 0, None, None);
        assert_eq!(a_edge, b_edge);
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3, 4]));
        assert_eq!(graph.neighbors(4), Some(vec![0]));
        assert_eq!(graph.edges(4), Some(vec![a_edge]));
        graph.remove_edge(b_edge);
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3]));
        assert_eq!(graph.neighbors(4), Some(vec![]));
        assert_eq!(graph.edges(4), Some(vec![]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_none() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(0, new, None, None);
        assert_eq!(graph.edges(0), Some(vec![0, 1, 2, new_edge]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_start() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(0, new, Some(0), None);
        assert_eq!(graph.edges(0), Some(vec![0, new_edge, 1, 2]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_middle() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(0, new, Some(1), None);
        assert_eq!(graph.edges(0), Some(vec![0, 1, new_edge, 2]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_end() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(0, new, Some(2), None);
        assert_eq!(graph.edges(0), Some(vec![0, 1, 2, new_edge]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_start_reverse() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(0));
        assert_eq!(graph.edges(0), Some(vec![0, new_edge, 1, 2]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_middle_reverse() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(1));
        assert_eq!(graph.edges(0), Some(vec![0, 1, new_edge, 2]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_add_ordered_edge_end_reverse() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(2));
        assert_eq!(graph.edges(0), Some(vec![0, 1, 2, new_edge]));
        assert_eq!(graph.edges(new), Some(vec![new_edge]));
    }

    #[test]
    fn test_list_graph_ordered_re_add() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(0));
        assert_eq!(graph.neighbors(0), Some(vec![1, new, 2, 3]));
        assert_eq!(graph.edges(0), Some(vec![0, new_edge, 1, 2]));
        graph.remove_edge(new_edge);
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3]));
        assert_eq!(graph.edges(0), Some(vec![0, 1, 2]));
        graph.add_edge(new, 0, None, None);
        assert_eq!(graph.neighbors(0), Some(vec![1, new, 2, 3]));
        assert_eq!(graph.edges(0), Some(vec![0, new_edge, 1, 2]));
    }

    #[test]
    fn test_list_graph_ordered_re_add_different_position() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(0));
        assert_eq!(graph.edges(0), Some(vec![0, new_edge, 1, 2]));
        assert_eq!(graph.neighbors(0), Some(vec![1, new, 2, 3]));
        graph.remove_edge(new_edge);
        assert_eq!(graph.edges(0), Some(vec![0, 1, 2]));
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, 3]));
        graph.add_edge(new, 0, None, Some(1));
        assert_eq!(graph.edges(0), Some(vec![0, 1, new_edge, 2]));
        assert_eq!(graph.neighbors(0), Some(vec![1, 2, new, 3]));
    }

    #[test]
    fn test_list_graph_ordered_re_add_cyclic() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(0));
        let a = graph.cyclic_incident_succ(0, 0).unwrap();
        let b = graph.cyclic_incident_succ(a, 0).unwrap();
        let c = graph.cyclic_incident_succ(b, 0).unwrap();
        let d = graph.cyclic_incident_succ(c, 0).unwrap();
        assert_eq!(a, new_edge);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
        assert_eq!(d, 0);
        graph.remove_edge(new_edge);
        let a = graph.cyclic_incident_succ(0, 0).unwrap();
        let b = graph.cyclic_incident_succ(a, 0).unwrap();
        let c = graph.cyclic_incident_succ(b, 0).unwrap();
        let d = graph.cyclic_incident_succ(c, 0).unwrap();
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 0);
        assert_eq!(d, 1);
        graph.add_edge(new, 0, None, None);
        let a = graph.cyclic_incident_succ(0, 0).unwrap();
        let b = graph.cyclic_incident_succ(a, 0).unwrap();
        let c = graph.cyclic_incident_succ(b, 0).unwrap();
        let d = graph.cyclic_incident_succ(c, 0).unwrap();
        assert_eq!(a, new_edge);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
        assert_eq!(d, 0);
    }

    #[test]
    fn test_list_graph_ordered_double_add_wit_after() {
        let mut graph = k4_graph();
        let new = graph.new_vertex();
        let new_edge = graph.add_edge(new, 0, None, Some(0));
        graph.add_edge(new, 0, None, Some(new_edge));
        assert_eq!(graph.edges(0).unwrap(), vec![0, new_edge, 1, 2]);
        assert_eq!(graph.edges(new).unwrap(), vec![new_edge]);
    }

    #[test]
    fn from_edge_node_list_test() {
        let graph = ListGraph::k4();
        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.edges.len(), 6);
        assert_eq!(graph.edges(0), Some(vec![0, 2, 1]));
        assert_eq!(graph.edges(1), Some(vec![3, 4, 0]));
        assert_eq!(graph.edges(2), Some(vec![1, 5, 3]));
        assert_eq!(graph.edges(3), Some(vec![2, 4, 5]));
        assert_eq!(graph.neighbors(0), Some(vec![1, 3, 2]));
        assert_eq!(graph.neighbors(1), Some(vec![2, 3, 0]));
        assert_eq!(graph.neighbors(2), Some(vec![0, 3, 1]));
        assert_eq!(graph.neighbors(3), Some(vec![0, 1, 2]));
    }
}
