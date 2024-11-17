use std::{mem, ptr};

use llio::util::record_id::RecordId;
use trail::{deserialize::Deserialize, serialize::Serialize};

#[derive(Debug)]
pub struct FileBTreeNode {
    items: Vec<RecordId>,
    internal: bool,
    // non_ptr_items: usize,
}

impl FileBTreeNode {
    pub fn empty(internal: bool) -> Self {
        Self {
            items: Vec::new(),
            internal,
            // non_ptr_items: 0,
        }
    }

    pub fn from_items(items: &[RecordId]) -> Self {
        let mut node = Self::empty(false);
        for it in items {
            node.append(it.clone());
            // if !it.is_pair() {
            //     node.set_internal(true);
            // }
        }

        node
    }

    fn set_internal(&mut self, internal: bool) {
        self.internal = internal;
    }

    pub fn append(&mut self, item: RecordId) {
        // if !item.is_pointer() {
        //     self.non_ptr_items += 1
        // }
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<RecordId> {
        self.items.pop()
    }

    pub fn insert(&mut self, item: RecordId, idx: usize) {
        // if !item.is_pointer() {
        //     self.non_ptr_items += 1
        // }
        self.items.splice(idx..idx, [item]);
    }

    pub fn replace(&mut self, item: RecordId, idx: usize) -> Option<RecordId> {
        // if !item.is_pointer() {
        //     self.non_ptr_items += 1
        // }
        if self.items.get(idx).is_none() {
            return None;
        }
        // if self.items.get(idx).unwrap().is_pointer() {
        //     self.non_ptr_items -= 1
        // }

        Some(std::mem::replace(&mut self.items[idx], item))
    }

    pub fn get(&self, idx: usize) -> Option<&RecordId> {
        self.items.get(idx)
    }

    pub fn last(&self) -> Option<&RecordId> {
        self.items.last()
    }

    pub fn items(&self) -> &[RecordId] {
        &self.items
    }

    pub fn is_internal(&self) -> bool {
        self.internal
    }

    // pub fn non_ptr_len(&self) -> usize {
    //     self.non_ptr_items
    // }
}

impl Serialize for FileBTreeNode {
    fn size(&self) -> u32 {
        // size + is internal + vector of items
        mem::size_of::<u32>() as u32
            + mem::size_of::<bool>() as u32
            + self.items.iter().map(|item| item.size()).sum::<u32>()
    }

    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        let size = self.size();
        let mut buffer = vec![0u8; size as usize].into_boxed_slice();

        unsafe {
            ptr::copy_nonoverlapping(
                size.to_le_bytes().as_ptr(),
                buffer.as_mut_ptr(),
                mem::size_of::<u32>(),
            );
        }

        unsafe {
            ptr::copy_nonoverlapping(
                self.internal.serialize()?.as_ptr(),
                buffer.as_mut_ptr().add(mem::size_of::<u32>()),
                mem::size_of::<bool>(),
            );
        }

        let mut offset = mem::size_of::<u32>() + mem::size_of::<bool>();
        for item in self.items.iter().map(|item| item.serialize()) {
            let item = item?;
            unsafe {
                ptr::copy_nonoverlapping(
                    item.as_ptr(),
                    buffer.as_mut_ptr().add(offset),
                    item.len(),
                );
            }

            offset += item.len();
        }

        Ok(buffer)
    }
}

impl Deserialize for FileBTreeNode {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let size = u32::deserialize(&from[..mem::size_of::<u32>()])?;
        let mut node = Self::empty(false);
        node.set_internal(bool::deserialize(
            &from[mem::size_of::<u32>()..(mem::size_of::<u32>() + mem::size_of::<bool>())],
        )?);

        let mut offset = mem::size_of::<u32>() + mem::size_of::<bool>();

        while offset < size as usize {
            let item = RecordId::deserialize(&from[offset..])?;
            offset += item.size() as usize;

            node.append(item);
        }

        Ok(node)
    }
}
