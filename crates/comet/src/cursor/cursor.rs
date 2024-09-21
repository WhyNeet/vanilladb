use std::{cell::RefCell, error::Error, io, rc::Rc};

use crate::{document::Document, page::PAGE_SIZE, pager::Pager, util};

pub struct Cursor {
    page: u64,
    offset: u16,
    pager: Rc<RefCell<Pager>>,
}

impl Cursor {
    pub fn new(pager: Rc<RefCell<Pager>>) -> Self {
        Self {
            page: 0,
            offset: 2,
            pager,
        }
    }

    pub fn next_document(&mut self) -> Result<(), Box<dyn Error>> {
        let current_size = self.current_document_size()? as usize;
        let new_offset = self.offset as usize + current_size + 4;
        let advance_pages = new_offset / PAGE_SIZE;
        let new_offset = new_offset % PAGE_SIZE;

        self.page += advance_pages as u64;
        // add 2 to the offset to include the new page's first two bytes
        self.offset = new_offset as u16 + if advance_pages > 0 { 2 } else { 0 };

        Ok(())
    }

    pub fn read_current_document(&self) -> Result<Document, Box<dyn Error>> {
        let current_size = self.current_document_size()? as usize;
        // take 4 bytes with document size into account
        let mut buffer = vec![0u8; current_size + 4].into_boxed_slice();
        self.pager
            .borrow()
            .read_at(&mut buffer, (self.page, self.offset))?;

        let document = Document::deserialize(&buffer)?;

        Ok(document)
    }

    pub fn remove_current_document(&self) -> Result<(), Box<dyn Error>> {
        let current_size = self.current_document_size()? as usize;

        self.pager
            .borrow_mut()
            .erase_at(current_size, (self.page, self.offset + 4))?;

        Ok(())
    }

    pub fn is_current_document_removed(&self) -> Result<bool, Box<dyn Error>> {
        let current_size = self.current_document_size()? as usize;
        let mut bytes = vec![0u8; current_size].into_boxed_slice();
        self.pager
            .borrow()
            .read_at(&mut bytes[..], (self.page, self.offset + 4))?;

        Ok(util::buf::is_zero(&bytes))
    }

    pub fn insert_document(&self, document: &Document) -> Result<(), Box<dyn Error>> {
        if !self.is_current_document_removed()? {
            return Err(Box::new(io::Error::other("current document is not empty")));
        }

        let current_document_size = self.current_document_size()?;

        if current_document_size < document.size() {
            return Err(Box::new(io::Error::other(
                "the document provided is larger than current gap",
            )));
        }

        let buffer = document.serialize()?;
        println!("insert {} into gap {current_document_size}", buffer.len());

        self.pager
            .borrow_mut()
            .replace_at(&buffer, (self.page, self.offset))?;

        Ok(())
    }

    pub fn current_document_size(&self) -> Result<u32, Box<dyn Error>> {
        let mut size = [0u8; 4];
        self.pager
            .borrow()
            .read_at(&mut size, (self.page, self.offset))?;
        Ok(u32::from_le_bytes(size))
    }
}
