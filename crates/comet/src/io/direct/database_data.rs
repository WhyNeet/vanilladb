use std::{collections::HashMap, os::fd::RawFd};

use super::collection_data::CollectionData;

pub struct DatabaseData {
    /// A mapping of collection names to their file descriptors
    collections: HashMap<String, CollectionData>,
}

impl DatabaseData {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }

    pub fn insert_collection(&mut self, name: String, data: CollectionData) {
        self.collections.insert(name, data);
    }

    pub fn collection(&self, name: &str) -> Option<&CollectionData> {
        self.collections.get(name)
    }

    pub fn collections(&self) -> Vec<&CollectionData> {
        self.collections.values().collect()
    }
}
