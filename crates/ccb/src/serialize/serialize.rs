use std::error::Error;

pub trait Serialize: Sized {
    fn serialize(self) -> Result<Box<[u8]>, Box<dyn Error>>;
}
