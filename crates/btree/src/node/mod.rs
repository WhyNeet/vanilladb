use item::BTreeNodeItem;

pub mod item;

pub struct BTreeNode<Value> {
    items: Vec<BTreeNodeItem<Value>>,
}

impl<Value> BTreeNode<Value> {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }
}
