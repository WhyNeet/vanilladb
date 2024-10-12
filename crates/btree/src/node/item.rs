use std::{cell::RefCell, rc::Rc};

use super::BTreeNode;

#[derive(Debug)]
pub enum BTreeNodeItem<Key: Clone, Value> {
    Key(Key),
    Pointer(Rc<RefCell<BTreeNode<Key, Value>>>),
    Pair(Key, Vec<Rc<Value>>),
}

impl<Key: Clone, Value> BTreeNodeItem<Key, Value> {
    pub fn as_pair(&self) -> (&Key, &[Rc<Value>]) {
        match self {
            BTreeNodeItem::Pair(k, v) => (k, v),
            _ => unreachable!(),
        }
    }

    pub fn as_pointer(&self) -> &Rc<RefCell<BTreeNode<Key, Value>>> {
        match self {
            BTreeNodeItem::Pointer(ptr) => ptr,
            _ => unreachable!(),
        }
    }

    pub fn as_key(&self) -> &Key {
        match self {
            BTreeNodeItem::Key(k) => k,
            _ => unreachable!(),
        }
    }

    pub fn is_key(&self) -> bool {
        match self {
            BTreeNodeItem::Key(_) => true,
            _ => false,
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            BTreeNodeItem::Pair(_, _) => true,
            _ => false,
        }
    }

    pub fn is_pointer(&self) -> bool {
        match self {
            BTreeNodeItem::Pointer(_) => true,
            _ => false,
        }
    }

    pub fn cloned(&self) -> Self {
        match self {
            Self::Key(k) => Self::Key(k.clone()),
            Self::Pair(k, v) => Self::Pair(k.clone(), v.iter().map(|val| Rc::clone(val)).collect()),
            Self::Pointer(ptr) => Self::Pointer(Rc::clone(ptr)),
        }
    }

    pub fn push_value(&mut self, value: Rc<Value>) {
        match self {
            Self::Pair(_k, v) => {
                v.push(value);
            }
            _ => unreachable!(),
        }
    }
}
