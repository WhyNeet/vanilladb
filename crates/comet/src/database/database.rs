use crate::collection::collection::Collection;

pub struct Database {
    collections: Vec<Collection>,
    name: String,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            collections: Vec::new(),
        }
    }

    pub fn custom(collections: Vec<Collection>, name: String) -> Self {
        Self { collections, name }
    }

    pub fn create_collection(&mut self, name: String) -> &mut Collection {
        let collection = Collection::new(name);
        self.collections.push(collection);
        self.collections.last_mut().unwrap()
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
