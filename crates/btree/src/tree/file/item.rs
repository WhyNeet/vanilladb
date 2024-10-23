use std::rc::Rc;

use llio::util::record_id::RecordId;
use trail::serialize::Serialize;

#[derive(Debug)]
pub enum FileBTreeNodeItem<Key: Clone, Value: Serialize> {
    Key(Key),
    Pair(Key, Vec<Rc<Value>>),
    Pointer(RecordId),
}

impl<Key: Clone, Value: Serialize> FileBTreeNodeItem<Key, Value> {
    pub fn as_pair(&self) -> (&Key, &[Rc<Value>]) {
        match self {
            FileBTreeNodeItem::Pair(k, v) => (k, v),
            _ => unreachable!(),
        }
    }

    pub fn as_pointer(&self) -> &RecordId {
        match self {
            FileBTreeNodeItem::Pointer(ptr) => ptr,
            _ => unreachable!(),
        }
    }

    pub fn as_key(&self) -> &Key {
        match self {
            FileBTreeNodeItem::Key(k) => k,
            _ => unreachable!(),
        }
    }

    pub fn is_key(&self) -> bool {
        match self {
            FileBTreeNodeItem::Key(_) => true,
            _ => false,
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            FileBTreeNodeItem::Pair(_, _) => true,
            _ => false,
        }
    }

    pub fn is_pointer(&self) -> bool {
        match self {
            FileBTreeNodeItem::Pointer(_) => true,
            _ => false,
        }
    }

    pub fn cloned(&self) -> Self {
        match self {
            Self::Key(k) => Self::Key(k.clone()),
            Self::Pair(k, v) => Self::Pair(k.clone(), v.iter().map(|val| Rc::clone(val)).collect()),
            Self::Pointer(ptr) => Self::Pointer(ptr.clone()),
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
