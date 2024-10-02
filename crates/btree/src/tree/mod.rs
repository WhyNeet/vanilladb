use crate::node::BTreeNode;

/// B+ Tree
pub struct BTree<Key, Value> {
    max_degree: usize,
    root: BTreeNode<Key, Value>,
}

impl<Key, Value> BTree<Key, Value> {
    pub fn new(max_degree: usize) -> Self {
        Self {
            max_degree,
            root: BTreeNode::empty(),
        }
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
}
