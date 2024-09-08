use core::str;
use std::{
    fs,
    io::{self},
    path::Path,
};

use crate::{database::database::Database, io::comet_io::CometIO};

pub struct Comet<IO: CometIO> {
    databases: Vec<Database>,
    io: IO,
}

impl<IO: CometIO> Comet<IO> {
    pub fn new(io: IO) -> Self {
        Comet {
            databases: Vec::new(),
            io,
        }
    }

    pub fn initialize(&self) -> io::Result<()> {
        fs::create_dir_all(Path::new(self.io.data_dir()))
    }

    pub fn create_database(&mut self, name: String) -> &mut Database {
        let database = Database::new(name);
        self.databases.push(database);
        self.databases.last_mut().unwrap()
    }

    pub fn database(&mut self, name: &str) -> Option<&mut Database> {
        self.databases.iter_mut().find(|db| db.name() == name)
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.io.load_fs()
    }
}
