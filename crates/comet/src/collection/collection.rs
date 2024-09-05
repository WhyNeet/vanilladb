use crate::{
    document::{constants::TOTAL_DOCUMENT_SIZE, document::Document},
    page::{
        constants::{DOCUMENTS_PER_PAGE, PAGE_SIZE},
        page::Page,
    },
};

pub struct Collection {
    num_documents: usize,
    pages: Vec<Page>,
    name: String,
    page_loader: Option<Box<dyn Fn(u64) -> [u8; PAGE_SIZE]>>,
    num_pages: u64,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection {
            num_documents: 0,
            pages: Vec::new(),
            name,
            page_loader: None,
            num_pages: 0,
        }
    }

    pub fn custom(
        num_documents: usize,
        pages: Vec<Page>,
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
        let page = if let Some(page) = self.pages.get_mut(page_idx) {
            page
        } else {
            let page = Page::new();
            self.pages.push(page);
            self.pages.last_mut().unwrap()
        };

        let offset = self.num_documents % DOCUMENTS_PER_PAGE;
        let byte_offset = offset * TOTAL_DOCUMENT_SIZE;

        let slot = page.retrieve_document_slot(byte_offset) as *mut [u8];

        slot
    }

    pub fn insert_document(&mut self, document: &Document) {
        let slot = self.create_document_slot();
        self.num_documents += 1;

        document.serialize(unsafe { slot.as_mut().unwrap() });
    }

    pub fn retrieve_document(&mut self, id: u64) -> Option<Document> {
        for idx in 0..self.num_pages {
            let page = if let Some(page) = self.pages.get(idx as usize) {
                page
            } else {
                if self.page_loader.is_none() {
                    return None;
                }

                let buffer = self.page_loader.as_ref().unwrap()(idx);
                let page = Page::with_buffer(Box::new(buffer));
                self.pages.push(page);
                self.pages.last().unwrap()
            };

            if let Some(doc) = page.find_by_id(id) {
                return Some(doc);
            }
        }

        None
    }

    pub fn pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn num_documents(&self) -> u64 {
        self.num_documents as u64
    }
}
