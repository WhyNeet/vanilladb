use crate::document::constants::TOTAL_DOCUMENT_SIZE;

pub const PAGE_SIZE: usize = 4096; // 4 KiB
pub const DOCUMENTS_PER_PAGE: usize = PAGE_SIZE / TOTAL_DOCUMENT_SIZE;
