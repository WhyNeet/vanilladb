use std::io;

use crate::database::Database;

pub trait CometIO {
    fn flush_db(&self, database: &Database) -> io::Result<()>;
    fn load_db(&mut self, name: &str) -> io::Result<Database>;
}
