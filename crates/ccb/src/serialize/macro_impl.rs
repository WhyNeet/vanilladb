macro_rules! serializable_number {
    (for $($t:ty),+) => {
      use super::Serialize;
      use std::error::Error;
        $(impl Serialize for $t {
            fn serialize(self) -> Result<Box<[u8]>, Box<dyn Error>> {
                Ok(Box::new(self.to_le_bytes()))
            }
        })*
    }
}

pub(crate) use serializable_number;
