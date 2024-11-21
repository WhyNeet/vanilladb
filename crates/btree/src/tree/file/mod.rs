pub mod item;
pub mod node;

use std::{
    error::Error,
    io::{Read, Write},
    mem,
    rc::Rc,
};

use item::FileBTreeNodeItem;
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
        let root = FileBTreeNode::empty(false, Some(RecordId::new("".to_string(), 2)));

        let root_record_id = self.save_node(&root)?;

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
            let root = self.read_node(&root_rci)?;

            Ok(root)
        } else {
            Ok(self.create_root()?.0)
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

        let mut node = FileBTreeNode::deserialize(&node)?;

        node.set_record_id(Some(record_id.clone()));

        Ok(node)
    }

    fn save_node(&mut self, node: &FileBTreeNode) -> Result<RecordId, Box<dyn Error>> {
        let pos = if let Some(record_id) = node.record_id() {
            (
                record_id.offset() / PAGE_SIZE as u64,
                (record_id.offset() % PAGE_SIZE as u64) as u16,
            )
        } else {
            self.pager.occupied()?
        };
        self.pager.replace_at(&node.serialize()?, pos)?;

        Ok(RecordId::new(
            "".to_string(),
            pos.0 * PAGE_SIZE as u64 + pos.1 as u64,
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

impl FileBTree {
    pub fn insert(&mut self, kv: (Field, Rc<Field>)) -> Result<bool, Box<dyn Error>> {
        let root = self.root()?;
        self._insert(root, kv)
    }

    fn _insert(
        &mut self,
        mut root: FileBTreeNode,
        kv: (Field, Rc<Field>),
    ) -> Result<bool, Box<dyn Error>> {
        if root.is_internal() {
            let idx = root
                .items()
                .iter()
                .enumerate()
                .filter(|(_, k)| k.is_key())
                .rev()
                .map(|(idx, k)| (idx, k.as_key()))
                .find(|(_idx, key)| kv.0.ge(key))
                .map(|(idx, _)| idx + 1)
                .unwrap_or(0);

            let ptr = root.items()[idx].as_pointer();

            self._insert(self.read_node(ptr)?, kv)
        } else {
            let idx = root
                .items()
                .iter()
                .map(|item| item.as_pair())
                .position(|(k, _v)| k.ge(&kv.0))
                .unwrap_or(root.items().len());

            let item = root.get(idx).map(|item| item.cloned());

            if item.is_some() && self.unique && item.as_ref().unwrap().as_pair().1[0].eq(&kv.1) {
                return Ok(false);
            }

            if item.is_some() && item.as_ref().unwrap().as_pair().0.eq(&kv.0) {
                let mut item = item.unwrap();
                item.push_value(kv.1);
                root.replace(item, idx);
            } else {
                root.insert(FileBTreeNodeItem::Pair(Rc::new(kv.0), vec![kv.1]), idx);
            }

            self.save_node(&root)?;

            Ok(true)
        }
    }

    pub fn get(&mut self, key: &Field) -> Result<Option<Box<[Rc<Field>]>>, Box<dyn Error>> {
        let root = self.root()?;
        self._get(key, root)
    }

    fn _get(
        &self,
        key: &Field,
        root: FileBTreeNode,
    ) -> Result<Option<Box<[Rc<Field>]>>, Box<dyn Error>> {
        if !root.is_internal() {
            return Ok(root
                .items()
                .iter()
                .map(|item| item.as_pair())
                .find(|item| item.0.eq(key))
                .map(|(_, v)| v.into_iter().map(|ptr| Rc::clone(ptr)).collect()));
        }

        let idx = root
            .items()
            .iter()
            .enumerate()
            .filter(|(_idx, item)| item.is_key())
            .map(|(idx, item)| (idx, item.as_key()))
            .find(|(_idx, k)| (*k).gt(key))
            .map(|(idx, _k)| idx)
            .unwrap_or(root.items().len())
            - 1;

        let ptr = root.items().get(idx).map(|item| item.as_pointer()).unwrap();
        let node = self.read_node(ptr)?;

        self._get(key, node)
    }
}
