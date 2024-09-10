use core::str;
use std::{
    cell::RefCell,
    error::Error,
    fs,
    io::{self},
    path::Path,
    ptr,
    rc::Rc,
};

use crate::{
    collection::Collection, database::database::Database, io::comet_io::CometIo, page::Page,
};

pub struct Comet {
    databases: Vec<Database>,
    io: Rc<RefCell<CometIo>>,
}

impl Comet {
    pub fn new(io: Rc<RefCell<CometIo>>) -> Self {
        Comet {
            databases: Vec::new(),
            io,
        }
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(Path::new(self.io.borrow().data_dir()))?;

        for database in self.io.borrow().databases() {
            let db_collections = self.io.borrow();
            let db_collections = db_collections.collections(&database)?;
            let mut collections: Vec<Collection> = Vec::with_capacity(db_collections.len());
            for collection_data in db_collections {
                let pages = (0..collection_data.num_pages())
                    .map(|idx| {
                        self.io
                            .borrow()
                            .load_collection_page(&database, collection_data.name(), idx)
                            .unwrap()
                    })
                    .collect();
                let collection =
                    Collection::custom(pages, collection_data.name().to_string(), None);
                collections.push(collection);
            }

            let database = Database::custom(collections, database.to_string(), Rc::clone(&self.io));
            self.databases.push(database);
        }

        Ok(())
    }

    pub fn create_database(&mut self, name: String) -> io::Result<&mut Database> {
        self.io.borrow_mut().create_database(&name)?;
        let database = Database::new(name, Rc::clone(&self.io));
        self.databases.push(database);
        Ok(self.databases.last_mut().unwrap())
    }

    pub fn database(&mut self, name: &str) -> Option<&mut Database> {
        self.databases.iter_mut().find(|db| db.name() == name)
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.io.borrow_mut().load_fs()
    }

    fn flush_all(&mut self) -> io::Result<()> {
        for database in self.databases.iter() {
            for collection in database.collections().iter() {
                for (idx, page) in collection.pages_ref().iter().enumerate() {
                    self.io.borrow().flush_collection_page(
                        database.name(),
                        collection.name(),
                        idx as u64,
                        unsafe { (page as *const Page as *mut Page).as_mut().unwrap() },
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl Drop for Comet {
    fn drop(&mut self) {
        self.flush_all().unwrap();
    }
}
