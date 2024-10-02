use std::{cell::RefCell, rc::Rc};

use super::BTreeNode;

#[derive(Debug)]
pub enum BTreeNodeItem<Key, Value> {
    Pointer(Key, Rc<RefCell<BTreeNode<Key, Value>>>),
    Pair(Key, Value),
}

impl<Key, Value> BTreeNodeItem<Key, Value> {
    pub fn as_pair(&self) -> (&Key, &Value) {
        match self {
            BTreeNodeItem::Pair(k, v) => (k, v),
            _ => unreachable!(),
        }
    }

    pub fn as_pointer(&self) -> (&Key, &Rc<RefCell<BTreeNode<Key, Value>>>) {
        match self {
            BTreeNodeItem::Pointer(key, ptr) => (key, ptr),
            _ => unreachable!(),
        }
    }
}
