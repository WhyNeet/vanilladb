use crate::node::BTreeNode;

pub struct BTree<Value> {
    max_degree: usize,
    root: BTreeNode<Value>,
}

impl<Value> BTree<Value> {
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
