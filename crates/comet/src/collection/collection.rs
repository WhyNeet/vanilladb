use std::error::Error;

use crate::{
    cursor::cursor::Cursor,
    document::document::Document,
    page::{constants::PAGE_SIZE, page::Page},
    pager::Pager,
};

pub struct Collection {
    pager: Pager,
    name: String,
    page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection {
            pager: Pager::new(),
            name,
            page_loader: None,
        }
    }

    pub fn custom(
        pages: Vec<Page>,
        name: String,
        page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
    ) -> Self {
        let num_pages = pages.len() as u64;

        Self {
            name,
            page_loader,
            pager: Pager::with_pages(pages, num_pages),
        }
    }

    pub fn insert_document(&mut self, document: &Document) -> Result<(), Box<dyn Error>> {
        let bytes = document.serialize()?;
        self.pager.write_bytes(&bytes);

        Ok(())
    }

    pub fn pages(&self) -> Vec<&Page> {
        self.pager.pages()
    }

    pub fn pages_ref(&self) -> &[Page] {
        self.pager.pages_ref()
    }

    pub fn pages_ref_mut(&mut self) -> &mut [Page] {
        self.pager.pages_ref_mut()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cursor(&self) -> Cursor {
        self.pager.cursor()
    }
}
