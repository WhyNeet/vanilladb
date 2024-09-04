// example table entity
use core::str;
use std::{
    ffi::{c_void, CString},
    fs,
    io::{self, BufRead, BufReader, Error},
    os::fd::RawFd,
    path::Path,
    ptr,
};

use libc::{close, open, pread, pwrite, O_DIRECT, O_RDONLY, O_SYNC, O_WRONLY};

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

#[derive(Debug)]
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

    pub fn pages(&self) -> Vec<&Page> {
        self.pages
            .iter()
            .take_while(|page| !page.is_null())
            .map(|page| unsafe { page.as_ref().unwrap() })
            .collect()
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

    pub fn collection(&self, name: &str) -> Option<&Collection> {
        self.collections.iter().find(|c| c.name == name)
    }

    pub fn collections(&self) -> &[Collection] {
        &self.collections
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

    pub fn database(&mut self, name: &str) -> Option<&mut Database> {
        self.databases.iter_mut().find(|db| db.name == name)
    }

    fn db_file_name(&self, db: &Database) -> String {
        format!("{}/{}.comet", self.data_dir, db.name)
    }

    fn create_db_file(&self, database: &Database) -> io::Result<()> {
        let file_name = self.db_file_name(database);
        let file_path = Path::new(&file_name);
        if !file_path.exists() {
            fs::File::create_new(file_path)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        for database in self.databases.iter() {
            self.flush_db(database)?
        }

        Ok(())
    }

    /// Direct I/O database flush
    fn flush_db(&self, database: &Database) -> io::Result<()> {
        let file_path = CString::new(self.db_file_name(database)).unwrap();
        let descriptor: RawFd = unsafe { open(file_path.as_ptr(), O_SYNC | O_DIRECT | O_WRONLY) };

        if descriptor < 0 {
            return Err(Error::last_os_error());
        }

        let mut bytes_written = 0;

        for collection in database.collections() {
            let pages = collection.pages();
            let mut collection_header = [0u8; PAGE_SIZE];
            // u8 will fit the max pages limit
            collection_header[0] = pages.len() as u8;
            // write collection name
            unsafe {
                ptr::copy_nonoverlapping(
                    collection.name.as_bytes().as_ptr() as *const u8,
                    collection_header[1..].as_mut_ptr() as *mut u8,
                    collection.name.len(),
                );
            }
            let written = unsafe {
                pwrite(
                    descriptor,
                    collection_header.as_ptr() as *const c_void,
                    PAGE_SIZE,
                    bytes_written,
                )
            };
            if written < 0 {
                unsafe { close(descriptor) };
                return Err(Error::last_os_error());
            }
            bytes_written += written as i64;
            for page in pages {
                let written = unsafe {
                    pwrite(
                        descriptor,
                        page.buffer.as_ptr() as *const c_void,
                        PAGE_SIZE,
                        bytes_written,
                    )
                };
                if written < 0 {
                    unsafe { close(descriptor) };
                    return Err(Error::last_os_error());
                }
                bytes_written += written as i64;
            }
        }

        unsafe {
            close(descriptor);
        }

        Ok(())
    }

    pub fn load(&mut self) -> io::Result<()> {
        let db_files = Path::new(&self.data_dir)
            .read_dir()?
            .map(|f| f.unwrap().file_name())
            .map(|name| {
                name.to_string_lossy()
                    .rsplit_once(".")
                    .unwrap()
                    .0
                    .to_string()
            });

        for db in db_files {
            self.load_db(db);
        }

        Ok(())
    }

    fn load_db(&mut self, name: String) {
        let file_path = CString::new(format!("{}/{name}.comet", self.data_dir)).unwrap();
        let descriptor = unsafe { open(file_path.as_ptr(), O_SYNC | O_DIRECT | O_RDONLY) };

        let mut collections: Vec<Collection> = Vec::new();

        let mut byte_offset = 0;
        loop {
            let collection_header = [0u8; PAGE_SIZE];
            let read = unsafe {
                pread(
                    descriptor,
                    collection_header.as_ptr() as *mut c_void,
                    PAGE_SIZE,
                    byte_offset,
                )
            };

            if read <= 0 {
                break;
            }

            byte_offset += read as i64;
            let pages = collection_header[0];
            let name = {
                let mut len = 0;
                while collection_header[1 + len] != b'\0' {
                    len += 1;
                }
                String::from_utf8_lossy(&collection_header[1..(len + 1)])
            };
            let mut page_ptrs: Vec<Box<Page>> = Vec::new();

            for idx in 0..pages {
                let page = Box::new(Page {
                    buffer: [0u8; PAGE_SIZE],
                });
                let read = unsafe {
                    pread(
                        descriptor,
                        page.buffer.as_ptr() as *mut c_void,
                        PAGE_SIZE,
                        byte_offset,
                    )
                };

                byte_offset += read as i64;

                page_ptrs.push(page);
            }

            let collection = Collection {
                name: name.to_string(),
                num_documents: 0,
                pages: {
                    let mut buf: [*const Page; COLLECTION_MAX_PAGES] =
                        [ptr::null(); COLLECTION_MAX_PAGES];
                    for (idx, page) in page_ptrs.into_iter().enumerate() {
                        let raw = Box::into_raw(page) as *const Page;
                        buf[idx] = raw;
                    }

                    buf
                },
            };

            collections.push(collection);
        }

        self.databases.push(Database { collections, name });
    }
}
