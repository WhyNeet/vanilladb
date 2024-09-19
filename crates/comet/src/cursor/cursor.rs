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
        let bytes_to_remove = current_size;

        self.pager.borrow_mut().write_at(
            &vec![0u8; bytes_to_remove],
            Some((self.page, self.offset + 4)),
        )?;

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
