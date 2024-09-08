use std::{ffi::c_void, ptr};

use super::constants::PAGE_SIZE;

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
        Self {
            occupied: u16::from_le_bytes((&buffer[..2].try_into()).unwrap()),
            buffer,
            dirty: false,
        }
    }

    fn update_occupied(&mut self, occupied: u16) {
        self.occupied = occupied;
        let bytes = self.occupied.to_le_bytes();
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.as_mut_ptr(), bytes.len()) };
    }

    /// Writes the data to buffer and returns the number of bytes written
    pub fn write_to_buffer(&mut self, data: &[u8]) -> usize {
        // if the length of the data is greater than the amount of free bytes, write till the end of the page
        let bytes_to_write = data
            .len()
            .min(((PAGE_SIZE as u16) - self.occupied) as usize);
        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer.as_mut_ptr().add(self.occupied as usize),
                bytes_to_write,
            )
        };

        self.dirty = true;

        self.update_occupied(self.occupied + bytes_to_write as u16);

        bytes_to_write
    }

    pub unsafe fn buffer_ptr(&self) -> *mut c_void {
        self.buffer.as_ptr() as *mut c_void
    }

    pub fn is_full(&self) -> bool {
        self.occupied == PAGE_SIZE as u16
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn after_flush(&mut self) {
        self.dirty = false
    }
}
