use core::str;
use std::{
    error::Error,
    fs,
    io::{self},
};

use crate::{database::database::Database, io::io_config::IoConfig};

pub struct Comet {
    databases: Vec<Database>,
    config: IoConfig,
}

impl Comet {
    pub fn new(config: IoConfig) -> Self {
        Comet {
            databases: Vec::new(),
            config,
        }
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.config.data_dir()[..])?;

        Ok(())
    }

    pub fn create_database(&mut self, name: String) -> io::Result<&mut Database> {
        let database = Database::new(name, self.config.clone());
        self.databases.push(database?);
        Ok(self.databases.last_mut().unwrap())
    }

    pub fn database(&mut self, name: &str) -> Option<&mut Database> {
        self.databases.iter_mut().find(|db| db.name() == name)
    }
}
