macro_rules! serializable_number {
    (for $($t:ty),+) => {
      use super::Serialize;
        $(impl Serialize for $t {
            fn serialize(self) -> Box<[u8]> {
                Box::new(self.to_le_bytes())
            }
        })*
    }
}

pub(crate) use serializable_number;
