use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::node::{item::BTreeNodeItem, BTreeNode};

/// B+ Tree
#[derive(Debug)]
pub struct BTree<
    Key: std::cmp::PartialOrd + Clone + std::fmt::Debug,
    Value: std::cmp::PartialEq + std::fmt::Debug,
> {
    max_degree: usize,
    root: Rc<RefCell<BTreeNode<Key, Value>>>,
    unique: bool,
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd + Clone + std::fmt::Debug,
    Value: std::cmp::PartialEq + std::fmt::Debug,
{
    pub fn new(max_degree: usize, unique: bool) -> Self {
        Self {
            max_degree,
            root: Rc::new(RefCell::new(BTreeNode::empty(false, None))),
            unique,
        }
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
}

impl<Key, Value> BTree<Key, Value>
where
    Key: std::cmp::PartialOrd + Clone + std::fmt::Debug,
    Value: std::cmp::PartialEq + std::fmt::Debug,
{
    pub fn insert(&mut self, kv: (Key, Value)) -> bool {
        self._insert(Rc::clone(&self.root), (kv.0, Rc::new(kv.1)))
    }

    fn _insert(&mut self, root: Rc<RefCell<BTreeNode<Key, Value>>>, kv: (Key, Rc<Value>)) -> bool {
        if root.borrow().is_internal() {
            let root_mut = root.borrow_mut();
            let idx = root_mut
                .items()
                .iter()
                .enumerate()
                .filter(|(_, k)| k.is_key())
                .rev()
                .map(|(idx, k)| (idx, k.as_key()))
                .find(|(_idx, key)| kv.0.ge(key))
                .map(|(idx, _)| idx + 1)
                .unwrap_or(0);

            let ptr = Rc::clone(root_mut.items()[idx].as_pointer());

            drop(root_mut);

            self._insert(ptr, kv)
        } else {
            // if the node is a leaf node
            // insert the new KV pair before the first larger key

            let idx = root
                .borrow()
                .items()
                .iter()
                .map(|item| item.as_pair())
                .position(|(k, _v)| k.ge(&kv.0))
                .unwrap_or(root.borrow().items().len());
            let rt = root.borrow();
            let item = rt.get(idx).map(|item| item.cloned());
            drop(rt);

            if item.is_some() && self.unique && item.as_ref().unwrap().as_pair().1[0].eq(&kv.1) {
                return false;
            }

            if item.is_some() && item.as_ref().unwrap().as_pair().0.eq(&kv.0) {
                let mut item = item.unwrap();
                item.push_value(kv.1);
                root.borrow_mut().replace(item, idx);
            } else {
                root.borrow_mut()
                    .insert(BTreeNodeItem::Pair(kv.0, vec![kv.1]), idx);
            }

            self.balance(root);

            true
        }
    }

    fn balance(&mut self, node: Rc<RefCell<BTreeNode<Key, Value>>>) {
        let node = node.borrow();

        if node.is_internal() && node.non_ptr_len() < self.max_degree
            || node.items().len() < self.max_degree
        {
            return;
        }

        let mid = node.items().len() >> 1;
        let (left, right) = node
            .items()
            .split_at(if node.is_internal() { mid + 1 } else { mid });

        let middle_key = if right[0].is_pointer() {
            right[1].as_key()
        } else if right[0].is_pair() {
            right[0].as_pair().0
        } else {
            right[0].as_key()
        }
        .clone();
        let middle = BTreeNodeItem::Key(middle_key);

        if let Some(parent) = node.parent() {
            let parent = Weak::upgrade(&parent).unwrap();

            let left = Rc::new(RefCell::new(BTreeNode::from_items(
                left,
                Some(Rc::downgrade(&parent)),
            )));
            let right = Rc::new(RefCell::new(BTreeNode::from_items(
                right,
                Some(Rc::downgrade(&parent)),
            )));

            let mut parent_mut = parent.borrow_mut();
            parent_mut.pop();
            parent_mut.append(BTreeNodeItem::Pointer(left));
            parent_mut.append(middle);
            parent_mut.append(BTreeNodeItem::Pointer(right));

            drop(parent_mut);

            self.balance(parent);
        } else {
            // Edge case: the node is root
            let new_root = Rc::new(RefCell::new(BTreeNode::empty(true, None)));
            let left = Rc::new(RefCell::new(BTreeNode::from_items(
                left,
                Some(Rc::downgrade(&new_root)),
            )));
            new_root.borrow_mut().append(BTreeNodeItem::Pointer(left));
            new_root.borrow_mut().append(middle);
            let right = Rc::new(RefCell::new(BTreeNode::from_items(
                right,
                Some(Rc::downgrade(&new_root)),
            )));
            new_root.borrow_mut().append(BTreeNodeItem::Pointer(right));

            self.root = new_root;
        }
    }

    pub fn get(&self, key: &Key) -> Option<Box<[Rc<Value>]>> {
        self._get(key, Rc::clone(&self.root))
    }

    fn _get(
        &self,
        key: &Key,
        root: Rc<RefCell<BTreeNode<Key, Value>>>,
    ) -> Option<Box<[Rc<Value>]>> {
        if !root.borrow().is_internal() {
            return root
                .borrow()
                .items()
                .iter()
                .map(|item| item.as_pair())
                .find(|item| item.0.eq(key))
                .map(|(_, v)| v.into_iter().map(|ptr| Rc::clone(ptr)).collect());
        }

        let idx = root
            .borrow()
            .items()
            .iter()
            .enumerate()
            .filter(|(_idx, item)| item.is_key())
            .map(|(idx, item)| (idx, item.as_key()))
            .find(|(_idx, k)| (*k).gt(key))
            .map(|(idx, _k)| idx)
            .unwrap_or(root.borrow().items().len())
            - 1;

        let ptr = root
            .borrow()
            .items()
            .get(idx)
            .map(|item| item.as_pointer())
            .map(Rc::clone)
            .unwrap();

        self._get(key, ptr)
    }
}
