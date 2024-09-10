use std::{error::Error, fs, io, path::PathBuf};

use crate::{collection::collection::Collection, io::io_config::IoConfig};

pub struct Database {
    collections: Vec<Collection>,
    name: String,
    config: IoConfig,
}

impl Database {
    pub fn new(name: String, config: IoConfig) -> io::Result<Self> {
        fs::create_dir_all(PathBuf::from(&config.data_dir()[..]).join(&name));

        Ok(Self {
            name,
            collections: Vec::new(),
            config,
        })
    }

    pub fn create_collection(&mut self, name: String) -> Result<&mut Collection, Box<dyn Error>> {
        let collection = Collection::new(&self.name, name, self.config.clone())?;
        self.collections.push(collection);
        Ok(self.collections.last_mut().unwrap())
    }

    pub fn collection(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|c| c.name() == name)
    }

    pub fn collections(&self) -> &[Collection] {
        &self.collections
    }

    pub fn collections_mut(&mut self) -> &mut [Collection] {
        &mut self.collections
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
