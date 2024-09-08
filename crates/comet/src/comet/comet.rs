use core::str;
use std::{
    cell::RefCell,
    fs,
    io::{self},
    path::Path,
    rc::Rc,
};

use crate::{database::database::Database, io::comet_io::CometIO};

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
}
