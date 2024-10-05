use std::{cell::RefCell, rc::Rc};

use crate::node::{item::BTreeNodeItem, BTreeNode};

/// B+ Tree
#[derive(Debug)]
pub struct BTree<Key: std::cmp::PartialOrd + Clone, Value> {
    max_degree: usize,
    root: Rc<RefCell<BTreeNode<Key, Value>>>,
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd + Clone,
{
    pub fn new(max_degree: usize) -> Self {
        Self {
            max_degree,
            root: Rc::new(RefCell::new(BTreeNode::empty(false, None))),
        }
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd + Clone,
{
    pub fn insert(&mut self, kv: (Key, Value)) {
        self._insert(Rc::clone(&self.root), (kv.0, Rc::new(kv.1)));
    }

    fn _insert(&mut self, root: Rc<RefCell<BTreeNode<Key, Value>>>, kv: (Key, Rc<Value>)) {
        if root.borrow().is_internal() {
            let root_mut = root.borrow_mut();
            let idx = root_mut
                .items()
                .iter()
                .enumerate()
                .filter(|(_, k)| k.is_key())
                .rev()
                .map(|(idx, k)| (idx, k.as_key()))
                .find(|(_idx, key)| (*key).ge(&kv.0))
                .map(|(idx, _)| idx + 1)
                .unwrap_or(0);

            let ptr = Rc::clone(root.borrow().items()[idx + 1].as_pointer());

            self._insert(ptr, kv);
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

            self.balance(root);
        }
    }

    fn balance(&mut self, node: Rc<RefCell<BTreeNode<Key, Value>>>) {
        let node = node.borrow();

        if node.is_internal() {
            return;
        }

        if node.items().len() < self.max_degree {
            return;
        }

        let mid = node.items().len() >> 1;
        let (left, right) = node.items().split_at(mid);

        if node.parent().is_none() {
            // Edge case: the node is root
            let new_root = Rc::new(RefCell::new(BTreeNode::empty(true, None)));
            let left = Rc::new(RefCell::new(BTreeNode::from_items(
                left,
                Some(Rc::downgrade(&new_root)),
            )));
            new_root.borrow_mut().append(BTreeNodeItem::Pointer(left));
            let middle = BTreeNodeItem::Key(
                if right[0].is_pointer() {
                    right[1].as_key()
                } else if right[0].is_pair() {
                    right[0].as_pair().0
                } else {
                    right[0].as_key()
                }
                .clone(),
            );
            new_root.borrow_mut().append(middle);
            let right = Rc::new(RefCell::new(BTreeNode::from_items(
                right,
                Some(Rc::downgrade(&new_root)),
            )));
            new_root.borrow_mut().append(BTreeNodeItem::Pointer(right));

            self.root = new_root;
        }
    }
}
