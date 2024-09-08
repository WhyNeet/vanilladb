use std::{cell::RefCell, io, rc::Rc};

use crate::{collection::collection::Collection, io::comet_io::CometIO};

pub struct Database {
    collections: Vec<Collection>,
    name: String,
    io: Rc<RefCell<dyn CometIO>>,
}

impl Database {
    pub fn new(name: String, io: Rc<RefCell<dyn CometIO>>) -> Self {
        Self {
            name,
            collections: Vec::new(),
            io,
        }
    }

    pub fn custom(
        collections: Vec<Collection>,
        name: String,
        io: Rc<RefCell<dyn CometIO>>,
    ) -> Self {
        Self {
            collections,
            name,
            io,
        }
    }

    pub fn create_collection(&mut self, name: String) -> io::Result<&mut Collection> {
        self.io.borrow().create_collection(&self.name, &name)?;
        let collection = Collection::new(name);
        self.collections.push(collection);
        Ok(self.collections.last_mut().unwrap())
    }

    pub fn collection(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|c| c.name() == name)
    }

    pub fn collections(&self) -> &[Collection] {
        &self.collections
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
