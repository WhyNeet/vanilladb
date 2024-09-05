use std::{io, path::Path};

use crate::{collection::Collection, database::Database};

pub trait CometIO {
    fn flush_db(&self, database: &Database) -> io::Result<()>;
    fn flush_collection(&self, db_path: &Path, collection: &Collection) -> io::Result<()>;
    fn load_db(&mut self, name: &str) -> io::Result<Database>;
    fn data_dir(&self) -> &str;
}
