use crate::page::Page;

pub struct Pager {
    pages: Vec<Page>,
    num_pages: u64,
    last_free_page: usize,
}

impl Pager {
    pub fn new() -> Self {
        Self {
            pages: vec![Page::new()],
            num_pages: 0,
            last_free_page: 0,
        }
    }

    pub fn with_pages(pages: Vec<Page>, num_pages: u64) -> Self {
        Self {
            last_free_page: if pages.len() > 0 { pages.len() - 1 } else { 0 },
            pages: if pages.len() > 0 {
                pages
            } else {
                vec![Page::new()]
            },
            num_pages,
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let page = self.pages.get_mut(self.last_free_page).unwrap();
        let mut bytes_written = page.write_to_buffer(&bytes[..]);
        loop {
            if bytes_written == bytes.len() {
                break;
            }

            if self.pages.len() == self.last_free_page + 1 {
                self.pages.push(Page::new());
                self.num_pages += 1;
            }
            self.last_free_page += 1;
            let page = self.pages.get_mut(self.last_free_page).unwrap();
            bytes_written += page.write_to_buffer(&bytes[bytes_written..]);
        }
    }

    pub fn pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }
}
