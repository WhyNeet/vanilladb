use std::error::Error;

pub trait Serialize {
    fn serialize(self) -> Result<Box<[u8]>, Box<dyn Error>>;
    /// Returns the size of this type in bytes
    fn size(&self) -> u32;
}
