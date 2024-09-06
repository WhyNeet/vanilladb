use std::{collections::HashMap, io::Error as IoError};

use crate::{
    field::{Field, FieldType},
    serialize::Serialize,
};

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

impl Deserialize for HashMap<String, Field> {
    fn deserialize(from: Box<[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte_offset = 0;

        let mut map = HashMap::new();
        while byte_offset < from.len() {
            let field_name_length = 'block: {
                for idx in 1..from.len() {
                    if from[idx] == 0 && from[idx - 1] == b'\\' {
                        break 'block Some(idx);
                    }
                }

                None
            }
            .ok_or(IoError::new(
                std::io::ErrorKind::InvalidData,
                "invalid ccb input",
            ))?;

            let field_name =
                String::deserialize(from[0..field_name_length].to_vec().into_boxed_slice())?;
            // offset + name length + null
            byte_offset += field_name.len() + 1;
            let field_length = u32::deserialize(
                from[(byte_offset + 1)..(byte_offset + 1 + mem::size_of::<u32>())]
                    .to_vec()
                    .into_boxed_slice(),
            )?;
            let field = Field::deserialize(
                from[byte_offset..(byte_offset + field_length as usize)]
                    .to_vec()
                    .into_boxed_slice(),
            )?;

            map.insert(field_name, field);
        }

        Ok(map)
    }
}
