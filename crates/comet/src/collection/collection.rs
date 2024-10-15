use std::{cell::RefCell, error::Error, io::Write, path::PathBuf, rc::Rc};

use llio::io::direct::DirectFileIo;

use crate::{
    cursor::cursor::Cursor, document::document::Document, io::io_config::IoConfig, pager::Pager,
};

pub struct Collection {
    pager: Rc<RefCell<Pager>>,
    name: String,
}

impl Collection {
    pub fn new(db: &str, name: String, config: IoConfig) -> Result<Self, Box<dyn Error>> {
        let collection_file_path = PathBuf::from(&config.data_dir()[..]).join(db).join(&name);
        let io = DirectFileIo::new(collection_file_path.to_str().unwrap())?;
        let pager = Pager::new(io);

        Ok(Collection {
            pager: Rc::new(RefCell::new(pager)),
            name,
        })
    }

    pub fn insert_document(&mut self, document: &Document) -> Result<(), Box<dyn Error>> {
        let bytes = document.serialize()?;
        self.pager.borrow_mut().write(&bytes)?;

        Ok(())
    }

    pub fn cursor(&self) -> Cursor {
        Cursor::new(Rc::clone(&self.pager))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
