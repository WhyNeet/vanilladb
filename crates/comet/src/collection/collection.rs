use std::ptr;

use crate::{
    document::{constants::TOTAL_DOCUMENT_SIZE, document::Document},
    page::{
        constants::{DOCUMENTS_PER_PAGE, PAGE_SIZE},
        page::Page,
    },
};

pub const COLLECTION_MAX_PAGES: usize = 1000;

pub struct Collection {
    num_documents: usize,
    pages: [*const Page; COLLECTION_MAX_PAGES],
    name: String,
    page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
    num_pages: u64,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection {
            num_documents: 0,
            pages: [ptr::null(); COLLECTION_MAX_PAGES],
            name,
            page_loader: None,
            num_pages: 0,
        }
    }

    pub fn custom(
        num_documents: usize,
        pages: [*const Page; COLLECTION_MAX_PAGES],
        name: String,
        page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
        num_pages: u64,
    ) -> Self {
        Self {
            name,
            num_documents,
            num_pages,
            page_loader,
            pages,
        }
    }

    fn create_document_slot(&mut self) -> *mut [u8] {
        let page_idx = self.num_documents / DOCUMENTS_PER_PAGE;
        let page = self.pages[page_idx as usize];
        let page = if page.is_null() {
            // heap-allocate the page
            let page = Box::new(Page::new());
            let raw = Box::into_raw(page) as *const Page;
            self.pages[page_idx] = raw;
            self.num_pages += 1;
            raw
        } else {
            page
        };

        let mut page = unsafe { Box::from_raw(page as *mut Page) };

        let offset = self.num_documents % DOCUMENTS_PER_PAGE;
        let byte_offset = offset * TOTAL_DOCUMENT_SIZE;

        let slot = page.retrieve_document_slot(byte_offset) as *mut [u8];

        let _ = Box::into_raw(page);

        slot
    }

    pub fn insert_document(&mut self, document: &Document) {
        let slot = self.create_document_slot();
        self.num_documents += 1;

        document.serialize(unsafe { slot.as_mut().unwrap() });
    }

    pub fn retrieve_document(&mut self, id: u64) -> Option<Document> {
        for idx in 0..self.num_pages {
            let page = self.pages[idx as usize] as *mut Page;
            if page.is_null() {
                if self.page_loader.is_none() {
                    return None;
                }

                let buffer = self.page_loader.as_ref().unwrap()(idx);
                let page_read = Box::new(Page::with_buffer(buffer));
                self.pages[idx as usize] = Box::into_raw(page_read);
            }

            let page = unsafe { ptr::read(self.pages[idx as usize] as *mut Page) };
            if let Some(doc) = page.find_by_id(id) {
                return Some(doc);
            }
        }

        None
    }

    pub fn pages(&self) -> Vec<&Page> {
        self.pages
            .iter()
            .take_while(|page| !page.is_null())
            .map(|page| unsafe { page.as_ref().unwrap() })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        for page in self.pages {
            if page.is_null() {
                continue;
            }
            unsafe { drop(Box::from_raw(page.cast_mut())) };
        }
    }
}
