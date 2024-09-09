use core::str;
use std::{
    cell::RefCell,
    fs,
    io::{self},
    path::Path,
    ptr,
    rc::Rc,
};

use crate::{
    collection::Collection, database::database::Database, io::comet_io::CometIO, page::Page,
};

pub struct Comet {
    databases: Vec<Database>,
    io: Rc<RefCell<dyn CometIO>>,
}

impl Comet {
    pub fn new(io: Rc<RefCell<dyn CometIO>>) -> Self {
        Comet {
            databases: Vec::new(),
            io,
        }
    }

    pub fn initialize(&self) -> io::Result<()> {
        fs::create_dir_all(Path::new(self.io.borrow().data_dir()))
    }

    pub fn create_database(&mut self, name: String) -> io::Result<&mut Database> {
        self.io.borrow().create_database(&name)?;
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
