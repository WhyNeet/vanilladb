use std::io;

use crate::page::Page;

pub trait CometIO {
    // fn flush_db(&self, database: &Database) -> io::Result<()>;
    // fn flush_collection(&self, db_path: &Path, collection: &Collection) -> io::Result<()>;
    // fn load_db(&mut self, name: &str) -> io::Result<Database>;
    fn load_fs(&mut self) -> io::Result<()>;
    fn flush_collection_page(
        &self,
        db: &str,
        collection: &str,
        idx: u64,
        page: &Page,
    ) -> io::Result<()>;
    fn load_collection_page(&self, db: &str, collection: &str, idx: u64) -> io::Result<Page>;
    fn data_dir(&self) -> &str;
}
