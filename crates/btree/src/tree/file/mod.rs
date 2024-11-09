pub mod item;
pub mod node;

use std::{error::Error, io::Read, mem};

use llio::{io::direct::DirectFileIo, page::PAGE_SIZE, pager::Pager, util::record_id::RecordId};
use node::FileBTreeNode;
use trail::{deserialize::Deserialize, field::FieldType};

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
    fn root_rci(&self) -> Result<RecordId, Box<dyn Error>> {
        let mut root_rci_len = vec![0u8; mem::size_of::<u32>()];
        let mut page = self.metadata.load_page(0).unwrap();

        page.read(&mut root_rci_len)?;
        let root_rci_len = u32::deserialize(&root_rci_len)?;

        let mut root_rci = vec![0u8; root_rci_len as usize];
        page.read_at(&mut root_rci, mem::size_of::<u32>() as u16)?;
        let root_rci = RecordId::deserialize(&root_rci)?;

        Ok(root_rci)
    }

    pub fn key_type(&self) -> Result<FieldType, Box<dyn Error>> {
        let mut key_type = vec![0u8; mem::size_of::<u32>()].into_boxed_slice();
        let mut page = self.metadata.load_page(1).unwrap();

        page.read(&mut key_type)?;
        let key_type = FieldType::deserialize(&key_type[..])?;

        Ok(key_type)
    }

    pub fn root(&self) -> Result<FileBTreeNode, Box<dyn Error>> {
        let root_rci = self.root_rci()?;
        let offset = root_rci.offset();
        let page_idx = offset / PAGE_SIZE as u64;
        let page_offset = offset % PAGE_SIZE as u64;

        let mut size = vec![0u8; mem::size_of::<u32>()].into_boxed_slice();
        self.pager
            .read_at(&mut size, (page_idx, page_offset as u16))?;
        let size = u32::deserialize(&size)?;
        let mut root = vec![0u8; size as usize].into_boxed_slice();
        self.pager
            .read_at(&mut root, (page_idx, page_offset as u16))?;
        let root = FileBTreeNode::deserialize(&root)?;

        Ok(root)
    }
}
