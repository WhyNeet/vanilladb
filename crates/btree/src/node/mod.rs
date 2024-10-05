use std::{cell::RefCell, rc::Weak};

use item::BTreeNodeItem;

pub mod item;

#[derive(Debug)]
pub struct BTreeNode<Key, Value> {
    items: Vec<BTreeNodeItem<Key, Value>>,
    internal: bool,
    parent: Option<Weak<RefCell<BTreeNode<Key, Value>>>>,
}

impl<Key, Value> BTreeNode<Key, Value> {
    pub fn empty(internal: bool, parent: Option<Weak<RefCell<BTreeNode<Key, Value>>>>) -> Self {
        Self {
            items: Vec::new(),
            internal,
            parent,
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

    pub fn parent(&self) -> Option<Weak<RefCell<BTreeNode<Key, Value>>>> {
        self.parent.as_ref().map(|node| Weak::clone(node))
    }
}
