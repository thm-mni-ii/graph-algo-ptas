use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub enum StackItem {
    Node(NodeIndex),
    Edge((NodeIndex, NodeIndex)),
    Degree(usize),
}

impl StackItem {
    pub fn unwrap_node(self) -> NodeIndex {
        match self {
            StackItem::Node(node) => node,
            _ => panic!("failed to unwrap node"),
        }
    }
    pub fn unwrap_edge(self) -> (NodeIndex, NodeIndex) {
        match self {
            StackItem::Edge(edge) => edge,
            _ => panic!("failed to unwrap edge"),
        }
    }
    pub fn unwrap_degree(self) -> usize {
        match self {
            StackItem::Degree(degree) => degree,
            _ => panic!("failed to unwrap degree"),
        }
    }
}
