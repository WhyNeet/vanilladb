use std::{
    ffi::{c_void, CString},
    fs,
    io::Error,
    os::fd::RawFd,
    path::Path,
    ptr,
};

use libc::{close, open, pread, pwrite, O_CREAT, O_DIRECT, O_RDONLY, O_SYNC, O_WRONLY};

use crate::{
    collection::Collection,
    database::Database,
    io::{comet_io::CometIO, io_config::IOConfig},
    page::PAGE_SIZE,
    util,
};

pub struct DirectIO {
    data_dir: Box<str>,
}

impl DirectIO {
    pub fn new(config: IOConfig) -> Self {
        Self {
            data_dir: config.data_dir(),
        }
    }
}

impl CometIO for DirectIO {
    fn data_dir(&self) -> &str {
        &self.data_dir
    }

    fn flush_db(&self, database: &crate::database::Database) -> std::io::Result<()> {
        let db_dir_path = util::path::db_foler_path(&self.data_dir, database.name());
        let db_dir_path = db_dir_path.as_path();
        fs::create_dir_all(db_dir_path)?;

        for collection in database.collections() {
            self.flush_collection(db_dir_path, &collection)?;
        }

        Ok(())
    }

    fn flush_collection(
        &self,
        db_path: &Path,
        collection: &crate::collection::Collection,
    ) -> std::io::Result<()> {
        let collection_path = db_path.join(format!("collection-{}.comet", collection.name()));
        let collection_path = CString::new(collection_path.to_str().unwrap()).unwrap();

        let pages = collection.pages();

        // prepare collection header
        let mut collection_header = [0u8; PAGE_SIZE];
        let pages_len = (pages.len() as u64).to_le_bytes();
        unsafe {
            ptr::copy_nonoverlapping(
                pages_len.as_ptr(),
                &mut collection_header as *mut u8,
                pages_len.len(),
            )
        };

        let num_documents = collection.num_documents().to_le_bytes();
        unsafe {
            ptr::copy_nonoverlapping(
                num_documents.as_ptr(),
                collection_header.as_mut_ptr().add(pages_len.len()),
                num_documents.len(),
            )
        };

        let descriptor: RawFd = unsafe {
            open(
                collection_path.as_ptr(),
                O_SYNC | O_DIRECT | O_WRONLY | O_CREAT,
            )
        };

        if descriptor < 0 {
            return Err(Error::last_os_error());
        }

        let mut bytes_written = 0;

        // write collection header
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
            // write page with data
            let written =
                unsafe { pwrite(descriptor, page.buffer_ptr(), PAGE_SIZE, bytes_written) };
            if written < 0 {
                unsafe { close(descriptor) };
                return Err(Error::last_os_error());
            }
            bytes_written += written as i64;
        }

        unsafe {
            close(descriptor);
        }

        Ok(())
    }

    fn load_db(&mut self, name: &str) -> std::io::Result<crate::database::Database> {
        let db_dir_path = util::path::db_foler_path(&self.data_dir, name);

        let mut collections: Vec<Collection> = Vec::new();

        for entity in db_dir_path.read_dir().unwrap() {
            let entity = entity?;
            let file_name = entity.file_name();
            let file_name = file_name.to_string_lossy();
            if !file_name.ends_with(".comet") {
                continue;
            }
            let file_path =
                CString::new(db_dir_path.join(file_name.to_string()).to_str().unwrap()).unwrap();
            let (_entity_type, name) = file_name
                .rsplit_once('.')
                .unwrap()
                .0
                .split_once('-')
                .unwrap();
            // later, check if type is a collection/index/etc.

            let descriptor = unsafe { open(file_path.as_ptr(), O_RDONLY | O_DIRECT | O_SYNC) };
            if descriptor < 0 {
                return Err(Error::last_os_error());
            }

            let mut collection_header = [0u8; PAGE_SIZE];
            let read = unsafe {
                pread(
                    descriptor,
                    collection_header.as_mut_ptr() as *mut c_void,
                    PAGE_SIZE,
                    0,
                )
            };

            if read < 0 {
                return Err(Error::last_os_error());
            } else if read == 0 {
                return Err(Error::other("empty collection file"));
            }

            let mut num_pages = [0u8; 8];
            unsafe {
                ptr::copy_nonoverlapping(
                    collection_header.as_ptr() as *const u8,
                    num_pages.as_mut_ptr(),
                    num_pages.len(),
                )
            };
            let num_pages = u64::from_le_bytes(num_pages);

            let mut num_documents = [0u8; 8];
            unsafe {
                ptr::copy_nonoverlapping(
                    collection_header.as_ptr().add(8) as *const u8,
                    num_documents.as_mut_ptr(),
                    num_documents.len(),
                )
            };
            let num_documents = u64::from_le_bytes(num_documents);

            let collection = Collection::custom(
                num_documents as usize,
                Vec::new(),
                name.to_string(),
                Some(Box::new(move |idx| {
                    let page = [0u8; PAGE_SIZE];
                    unsafe {
                        pread(
                            descriptor,
                            page.as_ptr() as *mut c_void,
                            PAGE_SIZE,
                            // first page is collection metadata
                            (PAGE_SIZE as i64) + (idx as i64 * PAGE_SIZE as i64),
                        )
                    };

                    page
                })),
                num_pages,
            );

            collections.push(collection);
        }

        let database = Database::custom(collections, name.to_string());

        Ok(database)
    }
}
