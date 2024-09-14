use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{document::Document, page::PAGE_SIZE, pager::Pager};

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
        let advance_pages = current_size / PAGE_SIZE;
        let new_offset = current_size % PAGE_SIZE;
        self.page += advance_pages as u64;
        self.offset = new_offset as u16;

        Ok(())
    }

    pub fn read_current_document(&self) -> Result<Document, Box<dyn Error>> {
        let current_size = self.current_document_size()? as usize;
        let mut buffer = vec![0u8; current_size].into_boxed_slice();
        self.pager
            .borrow()
            .read_at(&mut buffer, (self.page, self.offset))?;
        let document = Document::deserialize(&buffer)?;

        Ok(document)
    }

    pub fn current_document_size(&self) -> Result<u32, Box<dyn Error>> {
        let mut size = [0u8; 4];
        self.pager
            .borrow()
            .read_at(&mut size, (self.page, self.offset))?;
        Ok(u32::from_le_bytes(size))
    }
}
