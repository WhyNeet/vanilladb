use core::str;
use std::ptr;

use super::constants::{EMAIL_SIZE, ID_SIZE, USERNAME_SIZE};

pub struct Document {
    pub id: u64,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

impl Document {
    pub fn new(id: u64, username: &str, email: &str) -> Self {
        let mut username_bytes = [0u8; USERNAME_SIZE];
        let mut email_bytes = [0u8; EMAIL_SIZE];

        unsafe { Document::write_to_buffer(username.as_bytes(), &mut username_bytes, 0) };
        unsafe { Document::write_to_buffer(email.as_bytes(), &mut email_bytes, 0) };

        Document {
            id,
            username: username_bytes,
            email: email_bytes,
        }
    }

    pub fn serialize(&self, dest: &mut [u8]) {
        let id = self.id.to_be_bytes();
        unsafe { Document::write_to_buffer(&id, dest, 0) };
        unsafe { Document::write_to_buffer(&self.username, dest, ID_SIZE) }
        unsafe { Document::write_to_buffer(&self.email, dest, ID_SIZE + USERNAME_SIZE) }
    }

    pub fn deserialize(&mut self, src: &[u8]) {
        let mut id_buffer = [0u8; ID_SIZE];
        unsafe { Document::write_to_buffer(&src[..ID_SIZE], &mut id_buffer, 0) }
        self.id = u64::from_ne_bytes(id_buffer);

        unsafe {
            Document::write_to_buffer(
                &src[ID_SIZE..(ID_SIZE + USERNAME_SIZE)],
                &mut self.username,
                0,
            )
        }
        unsafe { Document::write_to_buffer(&src[(ID_SIZE + USERNAME_SIZE)..], &mut self.email, 0) }
    }

    pub fn display(&self) {
        println!("-- table entity {} --", self.id);
        println!("username: {}", unsafe {
            str::from_utf8_unchecked(&self.username)
        });
        println!("email: {}", unsafe {
            str::from_utf8_unchecked(&self.email)
        });
    }

    unsafe fn write_to_buffer(from: &[u8], to: &mut [u8], offset: usize) {
        let from_ptr: *const u8 = from.as_ptr();
        let to_ptr: *mut u8 = to.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(from_ptr, to_ptr, from.len())
    }
}
