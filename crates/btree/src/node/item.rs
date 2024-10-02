use std::{cell::RefCell, rc::Rc};

use super::BTreeNode;

#[derive(Debug)]
pub enum BTreeNodeItem<Key, Value> {
    Pointer(Rc<RefCell<BTreeNode<Key, Value>>>),
    Key(Key),
    Pair(Key, Value),
}

impl<Key, Value> BTreeNodeItem<Key, Value> {
    pub fn as_pair(&self) -> (&Key, &Value) {
        match self {
            BTreeNodeItem::Pair(k, v) => (k, v),
            _ => unreachable!(),
        }
    }

    pub fn as_key(&self) -> &Key {
        match self {
            BTreeNodeItem::Key(k) => k,
            _ => unreachable!(),
        }
    }

    pub fn as_pointer(&self) -> &Rc<RefCell<BTreeNode<Key, Value>>> {
        match self {
            BTreeNodeItem::Pointer(ptr) => ptr,
            _ => unreachable!(),
        }
    }
}
