use std::{cell::RefCell, rc::Rc};

use crate::node::{item::BTreeNodeItem, BTreeNode};

/// B+ Tree
#[derive(Debug)]
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
            let mut root = root.borrow_mut();
            let ptr = root
                .items()
                .iter()
                .map(|item| item.as_pointer())
                .find(|&(k, _v)| k.ge(&kv.0));

            if let Some((_k, ptr)) = ptr {
                // if there is a pointer to the right
                self._insert(Rc::clone(ptr), kv);
            } else {
                // create a new leaf node if no item is greater than key
                let leaf = Rc::new(RefCell::new(BTreeNode::<Key, Value>::empty(false)));
                let idx = root
                    .items()
                    .iter()
                    .map(|item| item.as_pointer())
                    .position(|(k, _ptr)| k.ge(&kv.0))
                    .unwrap_or(root.items().len());
                root.insert(BTreeNodeItem::Pointer(kv.0, leaf), idx);
            }
        } else {
            // if the node is a leaf node
            // insert the new KV pair before the first larger key

            let idx = root
                .borrow()
                .items()
                .iter()
                .map(|item| item.as_pair())
                .position(|(k, _v)| k.ge(&kv.0))
                .map(|pos| pos)
                .unwrap_or(root.borrow().items().len());
            root.borrow_mut()
                .insert(BTreeNodeItem::Pair(kv.0, kv.1), idx);
        }
    }
}
