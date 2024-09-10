use std::{error::Error, io::Write};

use crate::{
    document::document::Document,
    io::{comet_io::CometIo, io_config::IoConfig},
    pager::Pager,
};

pub struct Collection {
    pager: Pager,
    name: String,
}

impl Collection {
    pub fn new(db: &str, name: String, config: IoConfig) -> Result<Self, Box<dyn Error>> {
        let io = CometIo::new(db, &name, config)?;
        let pager = Pager::new(io);

        Ok(Collection { pager, name })
    }

    pub fn insert_document(&mut self, document: &Document) -> Result<(), Box<dyn Error>> {
        let bytes = document.serialize()?;
        self.pager.write(&bytes)?;

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
