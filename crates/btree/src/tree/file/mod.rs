pub mod item;
pub mod node;

use std::{
    error::Error,
    io::{Read, Write},
    mem,
};

use llio::{io::direct::DirectFileIo, page::PAGE_SIZE, pager::Pager, util::record_id::RecordId};
use node::FileBTreeNode;
use trail::{
    deserialize::Deserialize,
    field::{Field, FieldType},
    serialize::Serialize,
};

/// A file-based B+ tree
pub struct FileBTree {
    pager: Pager,
    unique: bool,
    max_degree: usize,
    metadata: DirectFileIo,
}

impl FileBTree {
    pub fn new(
        path: &str,
        metadata_path: &str,
        max_degree: usize,
        unique: bool,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pager: Pager::new(DirectFileIo::new(path)?),
            unique,
            max_degree,
            metadata: DirectFileIo::new(metadata_path)?,
        })
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
}

impl FileBTree {
    fn root_rci(&self) -> Result<Option<RecordId>, Box<dyn Error>> {
        let mut root_rci_len = vec![0u8; mem::size_of::<u32>()];
        let mut page = self.metadata.load_page(0)?;

        page.read(&mut root_rci_len)?;
        let root_rci_len = u32::deserialize(&root_rci_len)?;

        if root_rci_len == 0 {
            return Ok(None);
        }

        let mut root_rci = vec![0u8; root_rci_len as usize];
        page.read(&mut root_rci)?;
        let root_rci = RecordId::deserialize(&root_rci)?;

        Ok(Some(root_rci))
    }

    fn create_root(&mut self) -> Result<(FileBTreeNode, RecordId), Box<dyn Error>> {
        let root = FileBTreeNode::empty(false);

        self.pager.write(&root.serialize()?)?;

        let root_record_id = RecordId::new("".to_string(), 0);

        let mut metadata_page = self.metadata.load_page(0)?;
        metadata_page.write(&root_record_id.serialize()?)?;
        self.metadata.flush_page(0, metadata_page)?;

        Ok((root, root_record_id))
    }

    pub fn key_type(&self) -> Result<FieldType, Box<dyn Error>> {
        let mut key_type = vec![0u8; mem::size_of::<u32>()].into_boxed_slice();
        let mut page = self.metadata.load_page(1).unwrap();

        page.read(&mut key_type)?;
        let key_type = FieldType::deserialize(&key_type[..])?;

        Ok(key_type)
    }

    pub fn root(&mut self) -> Result<FileBTreeNode, Box<dyn Error>> {
        if let Some(root_rci) = self.root_rci()? {
            let offset = root_rci.offset();
            let page_idx = offset / PAGE_SIZE as u64;
            let page_offset = offset % PAGE_SIZE as u64;

            let mut size = vec![0u8; mem::size_of::<u32>()].into_boxed_slice();
            self.pager
                .read_at(&mut size, (page_idx, 2 + page_offset as u16))?;
            let size = u32::deserialize(&size)?;
            let mut root = vec![0u8; size as usize].into_boxed_slice();
            self.pager
                .read_at(&mut root, (page_idx, 2 + page_offset as u16))?;
            let root = FileBTreeNode::deserialize(&root)?;

            Ok(root)
        } else {
            Ok(self.create_root()?.0)
        }
    }

    pub fn insert(&mut self, kv: (Field, Field)) -> Result<bool, Box<dyn Error>> {
        let root = self.root()?;
        self._insert(root, kv)
    }

    fn _insert(&mut self, root: FileBTreeNode, kv: (Field, Field)) -> Result<bool, Box<dyn Error>> {
        if root.is_internal() {
            Ok(false)
        } else {
            // let idx = root.items().iter().map(|rci| self.read_node(rci).unwrap()).position(|pair| pair.); // SERIALIZE ITEMS WITHIN NODES

            Ok(true)
        }
    }

    fn read_node(&self, record_id: &RecordId) -> Result<FileBTreeNode, Box<dyn Error>> {
        let mut node_size = vec![0u8; mem::size_of::<u32>()].into_boxed_slice();
        self.pager.read_at(
            &mut node_size,
            (
                record_id.offset() / PAGE_SIZE as u64,
                (record_id.offset() % PAGE_SIZE as u64) as u16,
            ),
        )?;
        let node_size = u32::deserialize(&node_size)?;

        let mut node = vec![0u8; node_size as usize].into_boxed_slice();
        self.pager.read_at(
            &mut node,
            (
                record_id.offset() / PAGE_SIZE as u64,
                (record_id.offset() % PAGE_SIZE as u64) as u16,
            ),
        )?;

        let node = FileBTreeNode::deserialize(&node)?;

        Ok(node)
    }

    fn save_node(&mut self, node: &FileBTreeNode) -> Result<RecordId, Box<dyn Error>> {
        let (page_idx, page_occ) = self.pager.occupied()?;
        self.pager.write(&node.serialize()?)?;

        Ok(RecordId::new(
            "".to_string(),
            page_idx * 4096 + page_occ as u64,
        ))
    }

    fn remove_node(&mut self, record_id: &RecordId) -> Result<FileBTreeNode, Box<dyn Error>> {
        let node = self.read_node(record_id)?;

        self.pager.erase_at(
            node.size() as usize,
            (
                record_id.offset() / PAGE_SIZE as u64,
                (record_id.offset() % PAGE_SIZE as u64) as u16,
            ),
        )?;

        Ok(node)
    }
}
