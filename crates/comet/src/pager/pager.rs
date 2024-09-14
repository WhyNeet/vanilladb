use std::io::{Read, Write};

use crate::io::comet_io::CometIo;

/// Pager is an abstraction over hardware pages on the drive
pub struct Pager {
    io: CometIo,
    last_free_page: u64,
}

impl Pager {
    pub fn new(io: CometIo) -> Self {
        Self {
            io,
            last_free_page: 0,
        }
    }
}

impl Write for Pager {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut bytes_written = 0;
        while bytes_written < buf.len() {
            let mut page = self.io.load_collection_page(self.last_free_page)?;
            bytes_written += page.write(&buf[bytes_written..])?;
            self.io.flush_collection_page(self.last_free_page, page)?;

            if bytes_written < buf.len() {
                self.last_free_page += 1;
            }
        }

        Ok(bytes_written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for Pager {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut bytes_read = 0;
        let mut page_idx = 0;
        while bytes_read < buf.len() {
            let mut page = self.io.load_collection_page(page_idx)?;
            bytes_read += page.read(&mut buf[bytes_read..])?;
            page_idx += 1;
        }

        Ok(bytes_read)
    }
}
