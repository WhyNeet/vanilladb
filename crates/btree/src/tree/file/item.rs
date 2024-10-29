use std::rc::Rc;
use std::{mem, ptr};

use llio::util::record_id::RecordId;
use trail::deserialize::Deserialize;
use trail::field::Field;
use trail::serialize::Serialize;

#[derive(Debug)]
pub enum FileBTreeNodeItem<Key: Clone + Serialize> {
    Key(Key),
    Pair(Key, Vec<Rc<Field>>),
    Pointer(RecordId),
}

impl<Key: Clone + Serialize> FileBTreeNodeItem<Key> {
    pub fn as_pair(&self) -> (&Key, &[Rc<Field>]) {
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

    pub fn push_value(&mut self, value: Rc<Field>) {
        match self {
            Self::Pair(_k, v) => {
                v.push(value);
            }
            _ => unreachable!(),
        }
    }
}

impl<Key> Serialize for FileBTreeNodeItem<Key>
where
    Key: Clone + Serialize,
{
    fn size(&self) -> u32 {
        // type + size + item
        mem::size_of::<u8>() as u32
            + mem::size_of::<u32>() as u32
            + match self {
                Self::Key(key) => key.size(),
                // key length + \0 + value size
                Self::Pair(key, value) => key.size() + 1 + value.size(),
                Self::Pointer(rci) => rci.size(),
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
                        key.size().to_le_bytes().as_ptr(),
                        buffer.as_mut_ptr().add(1),
                        mem::size_of::<u32>(),
                    );
                }

                unsafe {
                    ptr::copy_nonoverlapping(
                        key.serialize()?.as_ptr(),
                        buffer.as_mut_ptr().add(1).add(mem::size_of::<u32>()),
                        size as usize - 1,
                    );
                };
            }
            Self::Pair(key, value) => {
                buffer[0] = 1;

                let key_size = key.size();
                let value_size = value.size();

                let total_length = key_size + 1 + value_size;

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
                            .add(key_size as usize)
                            .add(1),
                        value_size as usize,
                    );
                }
            }
            Self::Pointer(ptr) => {
                buffer[0] = 2;

                unsafe {
                    ptr::copy_nonoverlapping(
                        ptr.size().to_le_bytes().as_ptr(),
                        buffer.as_mut_ptr().add(1),
                        mem::size_of::<u32>(),
                    );
                }

                unsafe {
                    ptr::copy_nonoverlapping(
                        ptr.serialize()?.as_ptr(),
                        buffer.as_mut_ptr().add(1).add(mem::size_of::<u32>()),
                        ptr.size() as usize,
                    );
                };
            }
        };

        Ok(buffer)
    }
}

impl<Key> Deserialize for FileBTreeNodeItem<Key>
where
    Key: Clone + Deserialize + Serialize,
{
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let item_type = from[0];

        Ok(match item_type {
            0 => {
                let key_size = u32::deserialize(
                    (&from[mem::size_of::<u8>()..(mem::size_of::<u8>() + mem::size_of::<u32>())])
                        .try_into()?,
                )?;

                FileBTreeNodeItem::Key(Key::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>())
                        ..(mem::size_of::<u8>() + mem::size_of::<u32>() + key_size as usize)],
                )?)
            }
            1 => {
                let pair_size = u32::deserialize(
                    (&from[mem::size_of::<u8>()..(mem::size_of::<u8>() + mem::size_of::<u32>())])
                        .try_into()?,
                )?;

                // seek until null terminator
                let key_size = from
                    .iter()
                    .skip(mem::size_of::<u8>() + mem::size_of::<u32>())
                    .take_while(|&byte| *byte != 0)
                    .count();

                println!("key size: {key_size}");

                let key = Key::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>())
                        ..(mem::size_of::<u8>() + mem::size_of::<u32>() + key_size as usize)],
                )?;

                let value = Vec::<Field>::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>() + key_size as usize + 1)
                        ..(pair_size as usize + mem::size_of::<u8>() + mem::size_of::<u32>())],
                )?;

                FileBTreeNodeItem::Pair(key, value.into_iter().map(|item| Rc::new(item)).collect())
            }
            2 => {
                let id_size = u32::deserialize(
                    (&from[mem::size_of::<u8>()..(mem::size_of::<u8>() + mem::size_of::<u32>())])
                        .try_into()?,
                )?;

                FileBTreeNodeItem::Pointer(RecordId::deserialize(
                    &from[(mem::size_of::<u8>() + mem::size_of::<u32>())
                        ..(mem::size_of::<u8>() + mem::size_of::<u32>() + id_size as usize)],
                )?)
            }
            _ => unreachable!(),
        })
    }
}
