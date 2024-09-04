use core::str;
use std::{
    ffi::{c_void, CString},
    fs,
    io::{self, Error},
    os::fd::RawFd,
    path::Path,
    ptr,
};

use libc::{close, open, pread, pwrite, O_DIRECT, O_RDONLY, O_SYNC, O_WRONLY};

use crate::{
    collection::collection::{Collection, COLLECTION_MAX_PAGES},
    database::database::Database,
    page::constants::PAGE_SIZE,
};

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
        self.databases.iter_mut().find(|db| db.name() == name)
    }

    fn db_file_name(&self, db: &Database) -> String {
        format!("{}/{}.comet", self.data_dir, db.name())
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
            let pages_len = (pages.len() as u64).to_be_bytes();
            unsafe {
                ptr::copy_nonoverlapping(
                    pages_len.as_ptr(),
                    &mut collection_header as *mut u8,
                    pages_len.len(),
                )
            };
            // write collection name
            unsafe {
                ptr::copy_nonoverlapping(
                    collection.name().as_bytes().as_ptr() as *const u8,
                    collection_header[(pages_len.len())..].as_mut_ptr() as *mut u8,
                    collection.name().len(),
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
                let written =
                    unsafe { pwrite(descriptor, page.buffer_ptr(), PAGE_SIZE, bytes_written) };
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
            let mut pages = [0u8; 8];
            unsafe {
                ptr::copy_nonoverlapping(
                    &collection_header[0..8] as *const [u8] as *const u8,
                    &mut pages as *mut [u8; 8] as *mut u8,
                    8,
                )
            };
            let pages = u64::from_be_bytes(pages);
            let name = {
                let mut len = 0;
                while collection_header[8 + len] != b'\0' {
                    len += 1;
                }
                String::from_utf8_lossy(&collection_header[8..(len + 8)])
            }
            .to_string();

            let collection = Collection::custom(
                0,
                [ptr::null(); COLLECTION_MAX_PAGES],
                name.to_string(),
                Some(Box::new(move |idx| {
                    let page = [0u8; PAGE_SIZE];
                    unsafe {
                        pread(
                            descriptor,
                            page.as_ptr() as *mut c_void,
                            PAGE_SIZE,
                            byte_offset + (idx as i64 * PAGE_SIZE as i64),
                        )
                    };

                    page
                })),
                pages,
            );

            byte_offset += (pages as i64) * PAGE_SIZE as i64;

            collections.push(collection);
        }

        self.databases.push(Database::custom(collections, name));
    }
}
