use item::BTreeNodeItem;

pub mod item;

pub struct BTreeNode<Key, Value> {
    items: Vec<BTreeNodeItem<Key, Value>>,
    internal: bool,
}

impl<Key, Value> BTreeNode<Key, Value> {
    pub fn empty(internal: bool) -> Self {
        Self {
            items: Vec::new(),
            internal,
        }
    }

    pub fn append(&mut self, item: BTreeNodeItem<Key, Value>) {
        self.items.push(item);
    }

    pub fn insert(&mut self, item: BTreeNodeItem<Key, Value>, idx: usize) {
        self.items.splice(idx..idx, [item]);
    }

    pub fn items(&self) -> &[BTreeNodeItem<Key, Value>] {
        &self.items
    }

    pub fn is_internal(&self) -> bool {
        self.internal
    }
}
