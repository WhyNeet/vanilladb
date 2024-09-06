macro_rules! deserializable_number {
    (for $($t:ty),+) => {
      use super::Deserialize;
      use std::{mem, convert::TryInto, error::Error};
        $(impl Deserialize for $t {
            fn deserialize(from: Box<[u8]>) -> Result<Self, Box<dyn Error>> {
              let array: [u8; mem::size_of::<Self>()] = (&from[..]).try_into()?;
                Ok(Self::from_le_bytes(array))
            }
        })*
    }
}

pub(crate) use deserializable_number;
