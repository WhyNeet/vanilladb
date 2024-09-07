use std::error::Error;

use crate::{
    document::document::Document,
    page::{constants::PAGE_SIZE, page::Page},
};

pub struct Collection {
    pages: Vec<Page>,
    name: String,
    page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection {
            pages: vec![Page::new()],
            name,
            page_loader: None,
        }
    }

    pub fn custom(
        pages: Vec<Page>,
        name: String,
        page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
    ) -> Self {
        Self {
            name,
            page_loader,
            pages,
        }
    }

    pub fn insert_document(&mut self, document: &Document) -> Result<(), Box<dyn Error>> {
        let page = self.pages.last_mut().unwrap();
        let bytes = document.serialize()?;
        let bytes_written = page.write_to_buffer(&bytes[..]);
        if bytes_written < document.size() as usize {
            self.pages.push(Page::new());
            let page = self.pages.last_mut().unwrap();
            page.write_to_buffer(&bytes[bytes_written..]);
        }

        Ok(())
    }

    pub fn pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
