pub mod item;

use std::{error::Error, io::Read, mem};

use llio::{io::direct::DirectFileIo, pager::Pager, util::record_id::RecordId};
use trail::{deserialize::Deserialize, serialize::Serialize};

/// A file-based B+ tree
pub struct FileBTree {
    pager: Pager,
    unique: bool,
    max_degree: usize,
    metadata: DirectFileIo,
}

impl FileBTree {
    pub fn new(path: &str, max_degree: usize, unique: bool) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pager: Pager::new(DirectFileIo::new(path)?),
            unique,
            max_degree,
            metadata: DirectFileIo::new({
                let (path_start, path_end) = path.rsplit_once('/').unwrap_or(("", path));
                let (filename, _ext) = path_end.rsplit_once('.').unwrap_or((path_end, ""));
                &format!("{path_start}/{filename}.btm")
            })?,
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

    pub fn root(&self) -> Result<(), Box<dyn Error>> {
        let root_rci = self.root_rci()?;

        Ok(())
    }
}
