use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    fs,
    io::{self, Error},
    path::PathBuf,
};

use libc::{open, pread, pwrite, O_DIRECT, O_RDONLY, O_SYNC};

use crate::{
    io::{comet_io::CometIO, io_config::IOConfig},
    page::{Page, PAGE_SIZE},
    util,
};

use super::database_data::DatabaseData;

pub struct DirectIO {
    data_dir: Box<str>,
    databases: HashMap<String, DatabaseData>,
}

impl DirectIO {
    pub fn new(config: IOConfig) -> Self {
        Self {
            data_dir: config.data_dir(),
            databases: HashMap::new(),
        }
    }
}

impl DirectIO {
    fn load_db(&mut self, name: &str) -> io::Result<()> {
        let mut db = self
            .databases
            .insert(name.to_string(), DatabaseData::new())
            .unwrap();
        let db_dir_path = util::path::db_foler_path(&self.data_dir, name);

        // let mut collections: Vec<Collection> = Vec::new();

        for entity in db_dir_path.read_dir().unwrap() {
            let entity = entity?;
            let file_name = entity.file_name();
            let file_name = file_name.to_string_lossy();
            if !file_name.ends_with(".comet") {
                continue;
            }
            let file_path =
                CString::new(db_dir_path.join(file_name.to_string()).to_str().unwrap()).unwrap();
            let (entity_type, name) = file_name
                .rsplit_once('.')
                .unwrap()
                .0
                .split_once('-')
                .unwrap();

            let descriptor = unsafe { open(file_path.as_ptr(), O_RDONLY | O_DIRECT | O_SYNC) };
            if descriptor < 0 {
                return Err(Error::last_os_error());
            }

            match entity_type {
                "collection" => db.insert_collection(name.to_string(), descriptor),
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl CometIO for DirectIO {
    fn data_dir(&self) -> &str {
        &self.data_dir
    }

    fn create_database(&self, db: &str) -> io::Result<()> {
        fs::create_dir_all(PathBuf::from(&*self.data_dir).join(db))
    }

    fn create_collection(&self, db: &str, collection: &str) -> io::Result<()> {
        fs::File::create_new(PathBuf::from(&*self.data_dir).join(db).join(collection))?;
        Ok(())
    }

    fn load_fs(&mut self) -> std::io::Result<()> {
        for database in fs::read_dir(self.data_dir.to_string())? {
            let database = database?;
            let database_name = database.file_name();
            self.load_db(database_name.to_str().unwrap())?;
        }

        Ok(())
    }

    // fn flush_db(&self, database: &crate::database::Database) -> std::io::Result<()> {
    //     let db_dir_path = util::path::db_foler_path(&self.data_dir, database.name());
    //     let db_dir_path = db_dir_path.as_path();
    //     fs::create_dir_all(db_dir_path)?;

    //     for collection in database.collections() {
    //         self.flush_collection(db_dir_path, &collection)?;
    //     }

    //     Ok(())
    // }

    // fn flush_collection(
    //     &self,
    //     db_path: &Path,
    //     collection: &crate::collection::Collection,
    // ) -> std::io::Result<()> {
    //     let collection_path = db_path.join(format!("collection-{}.comet", collection.name()));
    //     let collection_path = CString::new(collection_path.to_str().unwrap()).unwrap();

    //     let pages = collection.pages();

    //     let descriptor: RawFd = unsafe {
    //         open(
    //             collection_path.as_ptr(),
    //             O_SYNC | O_DIRECT | O_WRONLY | O_CREAT,
    //         )
    //     };

    //     if descriptor < 0 {
    //         return Err(Error::last_os_error());
    //     }

    //     let mut bytes_written = 0;

    //     for page in pages {
    //         // write page with data
    //         let written =
    //             unsafe { pwrite(descriptor, page.buffer_ptr(), PAGE_SIZE, bytes_written) };
    //         if written < 0 {
    //             unsafe { close(descriptor) };
    //             return Err(Error::last_os_error());
    //         }
    //         bytes_written += written as i64;
    //     }

    //     unsafe {
    //         close(descriptor);
    //     }

    //     Ok(())
    // }

    // fn load_db(&mut self, name: &str) -> std::io::Result<crate::database::Database> {
    //     let db_dir_path = util::path::db_foler_path(&self.data_dir, name);

    //     let mut collections: Vec<Collection> = Vec::new();

    //     for entity in db_dir_path.read_dir().unwrap() {
    //         let entity = entity?;
    //         let file_name = entity.file_name();
    //         let file_name = file_name.to_string_lossy();
    //         if !file_name.ends_with(".comet") {
    //             continue;
    //         }
    //         let file_path =
    //             CString::new(db_dir_path.join(file_name.to_string()).to_str().unwrap()).unwrap();
    //         let (_entity_type, name) = file_name
    //             .rsplit_once('.')
    //             .unwrap()
    //             .0
    //             .split_once('-')
    //             .unwrap();
    //         // later, check if type is a collection/index/etc.

    //         let descriptor = unsafe { open(file_path.as_ptr(), O_RDONLY | O_DIRECT | O_SYNC) };
    //         if descriptor < 0 {
    //             return Err(Error::last_os_error());
    //         }

    //         let collection = Collection::custom(
    //             Vec::new(),
    //             name.to_string(),
    //             Some(Box::new(move |idx| {
    //                 let page = [0u8; PAGE_SIZE];
    //                 unsafe {
    //                     pread(
    //                         descriptor,
    //                         page.as_ptr() as *mut c_void,
    //                         PAGE_SIZE,
    //                         idx as i64 * PAGE_SIZE as i64,
    //                     )
    //                 };

    //                 page
    //             })),
    //         );

    //         collections.push(collection);
    //     }

    //     let database = Database::custom(collections, name.to_string());

    //     Ok(database)
    // }

    fn flush_collection_page(
        &self,
        db: &str,
        collection: &str,
        idx: u64,
        page: &crate::page::Page,
    ) -> std::io::Result<()> {
        let descriptor = self
            .databases
            .get(db)
            .ok_or(Error::other(format!("database \"{db}\" does not exist")))?
            .collection(collection)
            .ok_or(Error::other(format!(
                "collection \"{collection}\" in \"{db}\" does not exist"
            )))?
            .descriptor();

        let bytes_written = unsafe {
            pwrite(
                descriptor,
                page.buffer_ptr(),
                PAGE_SIZE,
                (idx * PAGE_SIZE as u64) as i64,
            )
        };

        if bytes_written < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn load_collection_page(&self, db: &str, collection: &str, idx: u64) -> io::Result<Page> {
        let descriptor = self
            .databases
            .get(db)
            .ok_or(Error::other(format!("database \"{db}\" does not exist")))?
            .collection(collection)
            .ok_or(Error::other(format!(
                "collection \"{collection}\" in \"{db}\" does not exist"
            )))?
            .descriptor();

        let mut buffer = Box::new([0u8; PAGE_SIZE]);
        unsafe {
            pread(
                descriptor,
                buffer.as_mut_ptr() as *mut c_void,
                PAGE_SIZE,
                (PAGE_SIZE as u64 * idx) as i64,
            )
        };

        let page = Page::from_buffer(buffer);

        Ok(page)
    }
}
