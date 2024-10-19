use std::rc::Rc;

use llio::util::record_id::RecordId;

#[derive(Debug)]
pub enum FileBTreeNodeItem<Key: Clone, Value> {
    Key(Key),
    Pair(Key, Vec<Rc<Value>>),
    Pointer(RecordId),
}
