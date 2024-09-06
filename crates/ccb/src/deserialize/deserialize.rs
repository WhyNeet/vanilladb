use std::error::Error;

pub trait Deserialize: Sized {
    fn deserialize(from: Box<[u8]>) -> Result<Self, Box<dyn Error>>;
}
