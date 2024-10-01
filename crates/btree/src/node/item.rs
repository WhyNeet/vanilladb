use std::rc::Rc;

use super::BTreeNode;

pub enum BTreeNodeItem<Value> {
    Pointer(Rc<BTreeNode<Value>>),
    Key(Value),
}
