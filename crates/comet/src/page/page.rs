use std::ffi::c_void;

use crate::document::{
    constants::{EMAIL_SIZE, ID_SIZE, TOTAL_DOCUMENT_SIZE, USERNAME_SIZE},
    document::Document,
};

use super::constants::{DOCUMENTS_PER_PAGE, PAGE_SIZE};

pub struct Page {
    buffer: Box<[u8; PAGE_SIZE]>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            buffer: Box::new([0u8; PAGE_SIZE]),
        }
    }

    pub fn with_buffer(buffer: Box<[u8; PAGE_SIZE]>) -> Self {
        Self { buffer }
    }

    pub fn retrieve_document_slot(&mut self, offset: usize) -> &mut [u8] {
        &mut self.buffer[offset..(offset + TOTAL_DOCUMENT_SIZE)]
    }

    pub fn find_by_id(&self, id: u64) -> Option<Document> {
        for i in 0..DOCUMENTS_PER_PAGE {
            let offset = i * TOTAL_DOCUMENT_SIZE;
            let buf = &self.buffer[(offset)..(offset + TOTAL_DOCUMENT_SIZE)];
            if buf[0..=ID_SIZE] == [0u8; ID_SIZE + 1] {
                return None;
            }
            let mut document = Document {
                id: 0,
                username: [0u8; USERNAME_SIZE],
                email: [0u8; EMAIL_SIZE],
            };
            document.deserialize(buf);
            if document.id == id {
                return Some(document);
            }
        }

        None
    }

    pub unsafe fn buffer_ptr(&self) -> *mut c_void {
        self.buffer.as_ptr() as *mut c_void
    }
}
