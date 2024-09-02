// example table entity
use std::ptr;

pub const ID_SIZE: usize = 8; // 8 bytes
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;

pub const TOTAL_DOCUMENT_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub struct TableEntity {
    pub id: u64,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

impl TableEntity {
    pub fn new(id: u64, username: &str, email: &str) -> Self {
        let mut username_bytes = [0u8; USERNAME_SIZE];
        let mut email_bytes = [0u8; EMAIL_SIZE];

        unsafe { TableEntity::write_to_buffer(username.as_bytes(), &mut username_bytes, 0) };
        unsafe { TableEntity::write_to_buffer(email.as_bytes(), &mut email_bytes, 0) };

        TableEntity {
            id,
            username: username_bytes,
            email: email_bytes,
        }
    }

    pub fn serialize(&self, dest: &mut [u8]) {
        let id = self.id.to_ne_bytes();
        unsafe { TableEntity::write_to_buffer(&id, dest, 0) };
        unsafe { TableEntity::write_to_buffer(&self.username, dest, ID_SIZE) }
        unsafe { TableEntity::write_to_buffer(&self.email, dest, ID_SIZE + USERNAME_SIZE) }
    }

    unsafe fn write_to_buffer(from: &[u8], to: &mut [u8], offset: usize) {
        let from_ptr: *const u8 = from.as_ptr();
        let to_ptr: *mut u8 = to.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(from_ptr, to_ptr, from.len())
    }
}
