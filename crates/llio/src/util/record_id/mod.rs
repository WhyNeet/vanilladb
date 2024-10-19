pub mod trail;

#[derive(Debug)]
pub struct RecordId {
    path: String,
    offset: u64,
}

impl RecordId {
    pub fn new(path: String, offset: u64) -> Self {
        Self { path, offset }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }
}
