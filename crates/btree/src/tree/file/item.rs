use std::rc::Rc;
use std::{mem, ptr};

use llio::util::record_id::RecordId;
use trail::deserialize::Deserialize;
use trail::field::Field;
use trail::serialize::Serialize;

#[derive(Debug)]
pub enum FileBTreeNodeItem {
    Key(Rc<Field>),
    Pair(Rc<Field>, Vec<Rc<Field>>),
    Pointer(RecordId),
}

impl FileBTreeNodeItem {
    pub fn as_pair(&self) -> (&Field, &[Rc<Field>]) {
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

    pub fn as_key(&self) -> &Field {
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
            Self::Key(k) => Self::Key(Rc::clone(k)),
            Self::Pair(k, v) => {
                Self::Pair(Rc::clone(k), v.iter().map(|val| Rc::clone(val)).collect())
            }
            Self::Pointer(ptr) => Self::Pointer(ptr.clone()),
        }
    }

    pub fn push_value(&mut self, value: Rc<Field>) {
        match self {
            Self::Pair(_k, v) => {
                v.push(value);
            }
            _ => unreachable!(),
        }
    }
}

impl Serialize for FileBTreeNodeItem {
    fn size(&self) -> u32 {
        // type + size + item
        mem::size_of::<u8>() as u32
            + mem::size_of::<u32>() as u32
            + match self {
                Self::Key(key) => key.size() - mem::size_of::<u32>() as u32,
                // key field size + value
                Self::Pair(key, value) => key.size() + value.size(),
                Self::Pointer(rci) => rci.size() - mem::size_of::<u32>() as u32,
            }
    }

    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        let size = self.size();
        let mut buffer = vec![0u8; size as usize].into_boxed_slice();

        match self {
            Self::Key(key) => {
                buffer[0] = 0;

                unsafe {
                    ptr::copy_nonoverlapping(
                        key.serialize()?.as_ptr(),
                        buffer.as_mut_ptr().add(1),
                        size as usize - 1,
                    );
                };
            }
            Self::Pair(key, value) => {
                buffer[0] = 1;

                let key_size = key.size();
                let value_size = value.size();

                let total_length = mem::size_of::<u32>() as u32 + key_size + value_size;

                // write total pair size
                unsafe {
                    ptr::copy_nonoverlapping(
                        total_length.to_le_bytes().as_ptr(),
                        buffer.as_mut_ptr().add(1),
                        mem::size_of::<u32>(),
                    );
                }

                // write key
                unsafe {
                    ptr::copy_nonoverlapping(
                        key.serialize()?.as_ptr(),
                        buffer.as_mut_ptr().add(1).add(mem::size_of::<u32>()),
                        key_size as usize,
                    );
                }

                // write value
                unsafe {
                    ptr::copy_nonoverlapping(
                        value.serialize()?.as_ptr(),
                        buffer
                            .as_mut_ptr()
                            .add(1)
                            .add(mem::size_of::<u32>())
                            .add(key_size as usize),
                        value_size as usize,
                    );
                }
            }
            Self::Pointer(ptr) => {
                buffer[0] = 2;

                unsafe {
                    ptr::copy_nonoverlapping(
                        ptr.serialize()?.as_ptr(),
                        buffer.as_mut_ptr().add(1),
                        ptr.size() as usize,
                    );
                };
            }
        };

        Ok(buffer)
    }
}

impl Deserialize for FileBTreeNodeItem {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let item_type = from[0];

        Ok(match item_type {
            0 => FileBTreeNodeItem::Key(Rc::new(Field::deserialize(
                &from[(mem::size_of::<u8>())..],
            )?)),
            1 => {
                let pair_size = u32::deserialize(
                    &from[mem::size_of::<u8>()..(mem::size_of::<u8>() + mem::size_of::<u32>())],
                )? as usize;

                let key = Field::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>())
                        ..(mem::size_of::<u8>() + pair_size)],
                )?;

                let key_size = key.size() as usize;

                let value = Vec::<Field>::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>() + key_size)
                        ..(mem::size_of::<u8>() + pair_size)],
                )?;

                FileBTreeNodeItem::Pair(
                    Rc::new(key),
                    value.into_iter().map(|item| Rc::new(item)).collect(),
                )
            }
            2 => {
                FileBTreeNodeItem::Pointer(RecordId::deserialize(&from[(mem::size_of::<u8>())..])?)
            }
            _ => unreachable!(),
        })
    }
}
