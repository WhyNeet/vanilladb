use super::macro_impl::deserializable_number;

deserializable_number!(for u128, u64, u32, u16, u8, i128, i64, i32, i16, i8, f64, f32);

impl Deserialize for String {
    fn deserialize(from: Box<[u8]>) -> Result<Self, Box<dyn Error>> {
        Ok(String::from_utf8_lossy(&from[..]).to_string())
    }
}

impl Deserialize for bool {
    fn deserialize(from: Box<[u8]>) -> Result<Self, Box<dyn Error>> {
        Ok(from[0] != 0)
    }
}
