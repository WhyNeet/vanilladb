use std::mem;

use crate::page::{Page, PAGE_SIZE};

use super::document_ptr::DocumentPtr;

pub struct Cursor<'a> {
    page_idx: usize,
    pages: &'a [Page],
    current_ptr: Option<DocumentPtr<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(pages: &'a [Page]) -> Self {
        Self {
            pages,
            page_idx: 0,
            current_ptr: None,
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = DocumentPtr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let document_offset = self
            .current_ptr
            .as_ref()
            .map(|ptr| ptr.offset() as usize + ptr.size())
            .unwrap_or(2) as usize;
        let (document_offset, advance_pages) = if document_offset > PAGE_SIZE {
            (document_offset % PAGE_SIZE, document_offset / PAGE_SIZE)
        } else {
            (document_offset, 0)
        };
        self.page_idx += advance_pages;

        let mut document_blocks: Vec<&[u8]> = Vec::new();

        let start_page_idx = self.page_idx;
        let page = &self.pages[self.page_idx as usize];
        let document_size = page.read_bytes(document_offset, mem::size_of::<u32>());
        document_blocks.push(document_size);
        let document_size = if document_size.len() != mem::size_of::<u32>() {
            self.page_idx += 1;

            let page = &self.pages[self.page_idx as usize];
            let remainder = page.read_bytes(2, mem::size_of::<u32>() - document_size.len());

            document_blocks.push(remainder);

            [document_size, remainder].concat()
        } else {
            document_size.to_vec()
        };
        let document_size = u32::from_le_bytes(document_size.try_into().unwrap());

        let page = &self.pages[self.page_idx];

        let mut bytes_left = document_size as usize;
        while bytes_left > 0 {
            let chunk = page.read_bytes(
                document_offset + mem::size_of::<u32>(),
                document_size as usize,
            );
            bytes_left -= chunk.len();
            document_blocks.push(chunk);
        }

        let document_ptr = DocumentPtr::new(
            start_page_idx,
            document_offset as u16,
            document_blocks,
            document_size as usize,
        );

        Some(document_ptr)
    }
}
