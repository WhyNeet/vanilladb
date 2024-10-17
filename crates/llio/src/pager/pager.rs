use std::io::{self, Read, Write};

use crate::io::direct::DirectFileIo;

/// Pager is an abstraction over hardware pages on the drive
pub struct Pager {
    io: DirectFileIo,
    last_free_page: u64,
}

impl Pager {
    pub fn new(io: DirectFileIo) -> Self {
        Self {
            io,
            last_free_page: 0,
        }
    }

    pub fn read_at(&self, buf: &mut [u8], offset: (u64, u16)) -> io::Result<usize> {
        let mut bytes_read = 0;
        let mut page_idx = offset.0;
        let mut page = self.io.load_page(page_idx)?;
        bytes_read += page.read_at(&mut buf[bytes_read..], offset.1)?;
        page_idx += 1;
        while bytes_read < buf.len() {
            let mut page = self.io.load_page(page_idx)?;
            bytes_read += page.read(&mut buf[bytes_read..])?;
            page_idx += 1;
        }

        Ok(bytes_read)
    }

    pub fn buffer(&self, offset: u64) -> io::Result<Box<[u8]>> {
        let page = self.io.load_page(offset)?;
        Ok(page
            .buffer()
            .into_iter()
            .map(|&b| b)
            .collect::<Vec<u8>>()
            .into_boxed_slice())
    }

    pub fn write_at(&mut self, buf: &[u8], offset: (u64, u16)) -> io::Result<(usize, u64)> {
        let mut bytes_written = 0;
        let mut page_idx = offset.0;
        let mut page = self.io.load_page(page_idx)?;
        bytes_written += page.write_at(&buf[bytes_written..], offset.1)?;
        self.io.flush_page(page_idx, page)?;

        while bytes_written < buf.len() {
            page_idx += 1;
            let mut page = self.io.load_page(page_idx)?;
            bytes_written += page.write_at(&buf[bytes_written..], 2)?;
            self.io.flush_page(page_idx, page)?;
        }

        self.last_free_page = page_idx;

        Ok((bytes_written, page_idx))
    }

    pub fn erase_at(&mut self, size: usize, offset: (u64, u16)) -> io::Result<usize> {
        let mut bytes_erased = 0;
        let mut page_idx = offset.0;
        let mut page = self.io.load_page(page_idx)?;
        bytes_erased += page.erase_at(size, offset.1)?;
        self.io.flush_page(page_idx, page)?;

        while bytes_erased < size {
            page_idx += 1;
            let mut page = self.io.load_page(page_idx)?;
            bytes_erased += page.erase_at(size - bytes_erased, 2)?;
            self.io.flush_page(page_idx, page)?;
        }

        Ok(bytes_erased)
    }

    pub fn replace_at(&mut self, buf: &[u8], offset: (u64, u16)) -> io::Result<usize> {
        let mut bytes_written = 0;
        let mut page_idx = offset.0;
        let mut page = self.io.load_page(page_idx)?;
        bytes_written += page.replace_at(&buf[bytes_written..], offset.1)?;
        self.io.flush_page(page_idx, page)?;

        while bytes_written < buf.len() {
            page_idx += 1;
            let mut page = self.io.load_page(page_idx)?;
            bytes_written += page.replace_at(&buf[bytes_written..], 2)?;
            self.io.flush_page(page_idx, page)?;
        }

        self.last_free_page = page_idx;

        Ok(bytes_written)
    }
}

impl Write for Pager {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(self.write_at(buf, (self.last_free_page, 2))?.0)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for Pager {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read_at(buf, (0, 0))
    }
}
