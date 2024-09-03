// example table entity
use core::str;
use libc::{c_void, close, open, posix_memalign, pwrite, O_CREAT, O_DIRECT, O_RDWR, O_TRUNC};
use std::{ffi::CString, fs, io, path::Path, ptr};

pub const ID_SIZE: usize = 8; // 8 bytes for a 64-bit integer
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 255;

pub const TOTAL_DOCUMENT_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

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
        let id = self.id.to_ne_bytes();
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

pub const COLLECTION_MAX_PAGES: usize = 100;
pub const DOCUMENTS_PER_PAGE: usize = PAGE_SIZE / TOTAL_DOCUMENT_SIZE;

pub struct Collection {
    num_documents: usize,
    pages: [*const Page; COLLECTION_MAX_PAGES],
    name: String,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection {
            num_documents: 0,
            pages: [ptr::null(); COLLECTION_MAX_PAGES],
            name,
        }
    }

    fn create_document_slot(&mut self) -> *mut [u8] {
        let page_idx = self.num_documents / DOCUMENTS_PER_PAGE;
        let page = self.pages[page_idx as usize];
        let page = if page.is_null() {
            // heap-allocate the page
            let page = Box::new(Page::new());
            let raw = Box::into_raw(page) as *const Page;
            self.pages[page_idx] = raw;
            raw
        } else {
            page
        };

        let mut page = unsafe { Box::from_raw(page as *mut Page) };

        let offset = self.num_documents % DOCUMENTS_PER_PAGE;
        let byte_offset = offset * TOTAL_DOCUMENT_SIZE;

        let slot = page.retrieve_document_slot(byte_offset) as *mut [u8];

        let _ = Box::into_raw(page);

        slot
    }

    pub fn insert_document(&mut self, document: &Document) {
        let slot = self.create_document_slot();
        self.num_documents += 1;

        document.serialize(unsafe { slot.as_mut().unwrap() });
    }

    pub fn retrieve_document(&self, id: u64) -> Option<Document> {
        for page in self.pages {
            if page.is_null() {
                return None;
            }

            let page = unsafe { ptr::read(page) };
            if let Some(doc) = page.find_by_id(id) {
                return Some(doc);
            }
        }

        None
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        for page in self.pages {
            if page.is_null() {
                continue;
            }
            unsafe { drop(Box::from_raw(page.cast_mut())) };
        }
    }
}

pub const PAGE_SIZE: usize = 4096; // 4 KiB

pub struct Page {
    buffer: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new() -> Self {
        Page {
            buffer: [0u8; PAGE_SIZE],
        }
    }

    pub fn retrieve_document_slot(&mut self, offset: usize) -> &mut [u8] {
        &mut self.buffer[offset..(offset + TOTAL_DOCUMENT_SIZE)]
    }

    pub fn find_by_id(&self, id: u64) -> Option<Document> {
        for i in 0..DOCUMENTS_PER_PAGE {
            let offset = i * TOTAL_DOCUMENT_SIZE;
            let buf = &self.buffer[(offset)..(offset + TOTAL_DOCUMENT_SIZE)];
            if buf[0..=ID_SIZE] == [0u8; ID_SIZE + 1] {
                return None;
            }
            let mut document = Document {
                id: 0,
                username: [0u8; USERNAME_SIZE],
                email: [0u8; EMAIL_SIZE],
            };
            document.deserialize(buf);
            if document.id == id {
                return Some(document);
            }
        }

        None
    }
}

pub struct Database {
    collections: Vec<Collection>,
    name: String,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            collections: Vec::new(),
        }
    }

    pub fn create_collection(&mut self, name: String) -> &mut Collection {
        let collection = Collection::new(name);
        self.collections.push(collection);
        self.collections.last_mut().unwrap()
    }

    pub fn collection(&self, name: String) -> Option<&Collection> {
        self.collections.iter().find(|c| c.name == name)
    }
}

pub struct Comet {
    data_dir: String,
    databases: Vec<Database>,
}

impl Comet {
    pub fn new(data_dir: String) -> Self {
        Comet {
            data_dir,
            databases: Vec::new(),
        }
    }

    pub fn initialize(&self) -> io::Result<()> {
        fs::create_dir_all(Path::new(&self.data_dir))
    }

    pub fn create_database(&mut self, name: String) -> &mut Database {
        let database = Database::new(name);
        self.create_db_file(&database).unwrap();
        self.databases.push(database);
        self.databases.last_mut().unwrap()
    }

    fn create_db_file(&self, database: &Database) -> io::Result<()> {
        let file_name = format!("{}/{}.comet", self.data_dir, database.name);
        let file_path = Path::new(&file_name);
        if !file_path.exists() {
            fs::File::create_new(file_path)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
