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

    pub fn flush(&self) -> io::Result<()> {
        for database in self.databases.iter() {
            self.io.flush_db(database)?
        }

        Ok(())
    }

    pub fn load(&mut self) -> io::Result<()> {
        let db_files = Path::new(self.io.data_dir())
            .read_dir()?
            .map(|f| f.unwrap().file_name());

        for db in db_files {
            let db = self.load_db(db.to_str().unwrap())?;
            self.databases.push(db);
        }

        Ok(())
    }

    fn load_db(&mut self, db: &str) -> io::Result<Database> {
        self.io.load_db(&db)
    }
}
