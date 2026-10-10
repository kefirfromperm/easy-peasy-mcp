//! The syntax tree of a query.

/// A node of the tree.
#[derive(Debug, Clone)]
pub enum Node {
    And(Vec<Node>),
    Or(Vec<Node>),
    Not(Box<Node>),
    Condition(String),
}

impl Node {
    /// The first child of a node.
    pub fn first(&self) -> &Node {
        match self {
            Node::Not(inner) => inner,
            _ => self.children().first().unwrap(),
        }
    }

    /// The children of `and` and `or`.
    pub fn children(&self) -> &[Node] {
        match self {
            Node::And(nodes) | Node::Or(nodes) => nodes,
            _ => &[],
        }
    }
}
