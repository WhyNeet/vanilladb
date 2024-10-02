use std::rc::Rc;

use super::BTreeNode;

pub enum BTreeNodeItem<Key, Value> {
    Pointer(Rc<BTreeNode<Value>>),
    Key(Key, Value),
}
