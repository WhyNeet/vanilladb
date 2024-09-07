use std::{ffi::c_void, ptr};

use super::constants::PAGE_SIZE;

pub struct Page {
    buffer: Box<[u8; PAGE_SIZE]>,
    free: usize,
}

impl Page {
    pub fn new() -> Self {
        Page {
            buffer: Box::new([0u8; PAGE_SIZE]),
            free: PAGE_SIZE,
        }
    }

    pub fn with_buffer(buffer: Box<[u8; PAGE_SIZE]>, free: usize) -> Self {
        Self { buffer, free }
    }

    /// Writes the data to buffer and returns the number of bytes written
    pub fn write_to_buffer(&mut self, data: &[u8]) -> usize {
        // if the length of the data is greater than the amount of free bytes, write till the end of the page
        let bytes_to_write = data.len().min(self.free);
        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer.as_mut_ptr().add(PAGE_SIZE - self.free),
                bytes_to_write,
            )
        };

        bytes_to_write
    }

    pub unsafe fn buffer_ptr(&self) -> *mut c_void {
        self.buffer.as_ptr() as *mut c_void
    }
}
