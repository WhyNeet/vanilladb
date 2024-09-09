pub struct DocumentPtr<'a> {
    offset: u16,
    page: usize,
    blocks: Vec<&'a [u8]>,
    size: usize,
}

impl<'a> DocumentPtr<'a> {
    pub fn new(page: usize, offset: u16, blocks: Vec<&'a [u8]>, size: usize) -> Self {
        Self {
            page,
            blocks,
            offset,
            size,
        }
    }

    pub fn buffer(&'a self) -> &'a [&'a [u8]] {
        &self.blocks
    }
    pub fn offset(&self) -> u16 {
        self.offset
    }
    pub fn page(&self) -> usize {
        self.page
    }
    pub fn size(&self) -> usize {
        self.size
    }
}
