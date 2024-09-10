use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{c_void, CString},
    fs,
    io::{self, Error, ErrorKind},
    os::{fd::AsRawFd, unix::fs::MetadataExt},
    path::PathBuf,
    rc::Rc,
};

use libc::{open, pread, pwrite, O_DIRECT, O_RDONLY, O_SYNC};

use crate::{
    io::io_config::IOConfig,
    page::{Page, PAGE_SIZE},
    util,
};

use super::{collection_data::CollectionData, database_data::DatabaseData};

pub struct CometIo {
    data_dir: Box<str>,
    databases: HashMap<String, DatabaseData>,
}

impl CometIo {
    pub fn new(config: IOConfig) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data_dir: config.data_dir(),
            databases: HashMap::new(),
        }))
    }
}

impl CometIo {
    fn load_db(&mut self, name: &str) -> io::Result<()> {
        let mut db = self
            .databases
            .insert(name.to_string(), DatabaseData::new())
            .unwrap();
        let db_dir_path = util::path::db_foler_path(&self.data_dir, name);

        for entity in db_dir_path.read_dir().unwrap() {
            let entity = entity?;
            let file_name = entity.file_name();
            let num_pages = entity.metadata()?.size() / PAGE_SIZE as u64;
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

            println!("collection: {name}");

            match entity_type {
                "collection" => db.insert_collection(
                    name.to_string(),
                    CollectionData::new(descriptor, num_pages, name.to_string()),
                ),
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl CometIo {
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }

    pub fn create_database(&mut self, db: &str) -> io::Result<()> {
        self.databases.insert(db.to_string(), DatabaseData::new());
        fs::create_dir_all(PathBuf::from(&*self.data_dir).join(db))
    }

    pub fn create_collection(&mut self, db: &str, collection: &str) -> io::Result<()> {
        let file = fs::File::create_new(
            PathBuf::from(&*self.data_dir)
                .join(db)
                .join(format!("collection-{collection}.comet")),
        );

        if let Ok(file) = file {
            self.databases
                .get_mut(db)
                .ok_or(Error::other(format!("database \"{db}\" does not exist")))?
                .insert_collection(
                    collection.to_string(),
                    CollectionData::new(file.as_raw_fd(), 0, collection.to_string()),
                );
        } else if let Err(e) = file {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn load_fs(&mut self) -> std::io::Result<()> {
        for database in fs::read_dir(self.data_dir.to_string())? {
            let database = database?;
            let database_name = database.file_name();
            self.load_db(database_name.to_str().unwrap())?;
        }

        Ok(())
    }

    pub fn flush_collection_page(
        &self,
        db: &str,
        collection: &str,
        idx: u64,
        page: &mut crate::page::Page,
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

        page.after_flush();

        if bytes_written < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn load_collection_page(&self, db: &str, collection: &str, idx: u64) -> io::Result<Page> {
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

    pub fn databases(&self) -> Vec<&str> {
        self.databases.keys().map(|val| val.as_str()).collect()
    }

    pub fn collections(
        &self,
        db: &str,
    ) -> Result<Vec<&CollectionData>, Box<dyn std::error::Error>> {
        Ok(self
            .databases
            .get(db)
            .ok_or(Error::other(format!("database \"{db}\" does not exist")))?
            .collections())
    }
}
