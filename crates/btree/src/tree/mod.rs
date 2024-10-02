use std::{cell::RefCell, rc::Rc};

use crate::node::{item::BTreeNodeItem, BTreeNode};

/// B+ Tree
pub struct BTree<Key: std::cmp::PartialOrd, Value> {
    max_degree: usize,
    root: Rc<RefCell<BTreeNode<Key, Value>>>,
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd,
{
    pub fn new(max_degree: usize) -> Self {
        Self {
            max_degree,
            root: Rc::new(RefCell::new(BTreeNode::empty(false))),
        }
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd,
{
    pub fn insert(&self, kv: (Key, Value)) {
        self._insert(Rc::clone(&self.root), kv);
    }

    fn _insert(&self, root: Rc<RefCell<BTreeNode<Key, Value>>>, kv: (Key, Value)) {
        if root.borrow().is_internal() {
        } else {
            // if the node is a leaf node
            // insert the new KV pair before

            let idx = root
                .borrow()
                .items()
                .into_iter()
                .map(|item| item.as_pair())
                .position(|(k, _v)| k.partial_cmp(&kv.0).unwrap().is_ge())
                .map(|pos| pos)
                .unwrap_or(root.borrow().items().len());
            root.borrow_mut()
                .insert(BTreeNodeItem::Pair(kv.0, kv.1), idx);
        }
    }
}
