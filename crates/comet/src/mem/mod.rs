use core::str;
// example table entity
use std::ptr;

pub const ID_SIZE: usize = 8; // 8 bytes for a 64-bit integer
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;

pub const TOTAL_DOCUMENT_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub struct CollectionEntity {
    pub id: u64,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

impl CollectionEntity {
    pub fn new(id: u64, username: &str, email: &str) -> Self {
        let mut username_bytes = [0u8; USERNAME_SIZE];
        let mut email_bytes = [0u8; EMAIL_SIZE];

        unsafe { CollectionEntity::write_to_buffer(username.as_bytes(), &mut username_bytes, 0) };
        unsafe { CollectionEntity::write_to_buffer(email.as_bytes(), &mut email_bytes, 0) };

        CollectionEntity {
            id,
            username: username_bytes,
            email: email_bytes,
        }
    }

    pub fn serialize(&self, dest: &mut [u8]) {
        let id = self.id.to_ne_bytes();
        unsafe { CollectionEntity::write_to_buffer(&id, dest, 0) };
        unsafe { CollectionEntity::write_to_buffer(&self.username, dest, ID_SIZE) }
        unsafe { CollectionEntity::write_to_buffer(&self.email, dest, ID_SIZE + USERNAME_SIZE) }
    }

    pub fn deserialize(&mut self, src: &[u8]) {
        let mut id_buffer = [0u8; ID_SIZE];
        unsafe { CollectionEntity::write_to_buffer(&src[..ID_SIZE], &mut id_buffer, 0) }
        self.id = u64::from_ne_bytes(id_buffer);

        unsafe {
            CollectionEntity::write_to_buffer(
                &src[ID_SIZE..(ID_SIZE + USERNAME_SIZE)],
                &mut self.username,
                0,
            )
        }
        unsafe {
            CollectionEntity::write_to_buffer(&src[(ID_SIZE + USERNAME_SIZE)..], &mut self.email, 0)
        }
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

pub const COLLECTION_MAX_PAGES: usize = 100;
pub const ENTITIES_PER_PAGE: usize = PAGE_SIZE / TOTAL_DOCUMENT_SIZE;

pub struct Collection {
    pub num_documents: u64,
    pub pages: [Page; COLLECTION_MAX_PAGES],
}

pub const PAGE_SIZE: usize = 4096; // 4 KiB

pub struct Page {
    pub buffer: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new() -> Self {
        Page {
            buffer: [0u8; PAGE_SIZE],
        }
    }
}
