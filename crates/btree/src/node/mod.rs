use item::BTreeNodeItem;

pub mod item;

pub struct BTreeNode<Key, Value> {
    items: Vec<BTreeNodeItem<Key, Value>>,
}

impl<Key, Value> BTreeNode<Key, Value> {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }
}
