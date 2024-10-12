use std::{cell::RefCell, rc::Weak};

use item::BTreeNodeItem;

pub mod item;

#[derive(Debug)]
pub struct BTreeNode<Key: Clone, Value> {
    items: Vec<BTreeNodeItem<Key, Value>>,
    internal: bool,
    parent: Option<Weak<RefCell<BTreeNode<Key, Value>>>>,
    non_ptr_items: usize,
}

impl<Key: Clone, Value> BTreeNode<Key, Value> {
    pub fn empty(internal: bool, parent: Option<Weak<RefCell<BTreeNode<Key, Value>>>>) -> Self {
        Self {
            items: Vec::new(),
            internal,
            parent,
            non_ptr_items: 0,
        }
    }

    pub fn from_items(
        items: &[BTreeNodeItem<Key, Value>],
        parent: Option<Weak<RefCell<BTreeNode<Key, Value>>>>,
    ) -> Self {
        let mut node = Self::empty(false, parent);
        for it in items {
            node.append(it.cloned());
            if !it.is_pair() {
                node.set_internal(true);
            }
        }

        node
    }

    fn set_internal(&mut self, internal: bool) {
        self.internal = internal;
    }

    pub fn append(&mut self, item: BTreeNodeItem<Key, Value>) {
        if !item.is_pointer() {
            self.non_ptr_items += 1
        }
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<BTreeNodeItem<Key, Value>> {
        self.items.pop()
    }

    pub fn insert(&mut self, item: BTreeNodeItem<Key, Value>, idx: usize) {
        if !item.is_pointer() {
            self.non_ptr_items += 1
        }
        self.items.splice(idx..idx, [item]);
    }

    pub fn replace(
        &mut self,
        item: BTreeNodeItem<Key, Value>,
        idx: usize,
    ) -> Option<BTreeNodeItem<Key, Value>> {
        if !item.is_pointer() {
            self.non_ptr_items += 1
        }
        if self.items.get(idx).is_none() {
            return None;
        }
        if self.items.get(idx).unwrap().is_pointer() {
            self.non_ptr_items -= 1
        }

        Some(std::mem::replace(&mut self.items[idx], item))
    }

    pub fn get(&self, idx: usize) -> Option<&BTreeNodeItem<Key, Value>> {
        self.items.get(idx)
    }

    pub fn last(&self) -> Option<&BTreeNodeItem<Key, Value>> {
        self.items.last()
    }

    pub fn items(&self) -> &[BTreeNodeItem<Key, Value>] {
        &self.items
    }

    pub fn is_internal(&self) -> bool {
        self.internal
    }

    pub fn parent(&self) -> Option<&Weak<RefCell<BTreeNode<Key, Value>>>> {
        self.parent.as_ref()
    }

    pub fn non_ptr_len(&self) -> usize {
        self.non_ptr_items
    }
}
