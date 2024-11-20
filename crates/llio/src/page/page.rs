use std::{
    io::{self, Read, Write},
    ptr,
};

use super::constants::PAGE_SIZE;

#[derive(Debug, Clone)]
pub struct Page {
    buffer: Box<[u8; PAGE_SIZE]>,
    occupied: u16,
    dirty: bool,
}

impl Page {
    pub fn new() -> Self {
        Page {
            buffer: Box::new([0u8; PAGE_SIZE]),
            // first two bytes store the occupied space
            occupied: 2,
            dirty: false,
        }
    }

    pub fn from_buffer(buffer: Box<[u8; PAGE_SIZE]>) -> Self {
        let occupied = u16::from_le_bytes((&buffer[..2].try_into()).unwrap()).max(2);

        let mut page = Self {
            occupied,
            buffer,
            dirty: false,
        };

        page.update_occupied(occupied);

        page
    }

    fn update_occupied(&mut self, occupied: u16) {
        self.occupied = occupied;
        let bytes = self.occupied.to_le_bytes();
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.as_mut_ptr(), bytes.len()) };
    }

    pub fn buffer(&self) -> &[u8; PAGE_SIZE] {
        &self.buffer
    }

    pub fn free(&self) -> u16 {
        PAGE_SIZE as u16 - self.occupied
    }

    pub fn occupied(&self) -> u16 {
        self.occupied
    }

    pub fn empty(&self) -> bool {
        self.occupied == 2
    }

    pub fn is_full(&self) -> bool {
        self.occupied == PAGE_SIZE as u16
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn write_at(&mut self, buf: &[u8], offset: u16) -> io::Result<usize> {
        let offset = if offset < self.occupied {
            self.occupied
        } else {
            offset
        };

        let bytes_to_write = buf.len().min(PAGE_SIZE - offset as usize);
        unsafe {
            ptr::copy(
                buf.as_ptr(),
                self.buffer.as_mut_ptr().add(offset as usize),
                bytes_to_write,
            )
        };

        self.dirty = true;

        self.update_occupied(offset + bytes_to_write as u16);

        Ok(bytes_to_write)
    }

    pub fn replace_at(&mut self, buf: &[u8], offset: u16) -> io::Result<usize> {
        let bytes_to_write = buf.len().min(PAGE_SIZE - offset as usize);
        unsafe {
            ptr::copy(
                buf.as_ptr(),
                self.buffer.as_mut_ptr().add(offset as usize),
                bytes_to_write,
            )
        };

        self.dirty = true;

        self.update_occupied(offset + bytes_to_write as u16);

        Ok(bytes_to_write)
    }

    pub fn erase_at(&mut self, size: usize, offset: u16) -> io::Result<usize> {
        let bytes_to_erase = size.min(PAGE_SIZE - offset as usize);

        unsafe {
            ptr::copy(
                vec![0u8; bytes_to_erase].as_ptr(),
                self.buffer.as_mut_ptr().add(offset as usize),
                bytes_to_erase,
            );
        };

        self.dirty = true;

        Ok(bytes_to_erase)
    }

    pub fn read_at(&mut self, buf: &mut [u8], offset: u16) -> io::Result<usize> {
        let bytes_to_read = buf.len().min(PAGE_SIZE - offset as usize);
        if bytes_to_read == 0 {
            return Ok(0);
        }

        unsafe {
            ptr::copy_nonoverlapping(
                &self.buffer[(offset as usize)..] as *const [u8] as *const u8,
                buf.as_mut_ptr(),
                bytes_to_read,
            )
        };

        Ok(bytes_to_read)
    }
}

impl Write for Page {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // if the length of the data is greater than the amount of free bytes, write till the end of the page
        self.write_at(buf, self.occupied)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.dirty = false;
        Ok(())
    }
}

impl Read for Page {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read_at(buf, 2)
    }
}
