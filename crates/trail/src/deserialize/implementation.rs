use std::{collections::HashMap, io::Error as IoError};

use crate::field::Field;

use super::macro_impl::deserializable_number;

deserializable_number!(for u128, u64, u32, u16, u8, i128, i64, i32, i16, i8, f64, f32);

impl Deserialize for String {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(String::from_utf8_lossy(&from[..]).to_string())
    }
}

impl Deserialize for bool {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(from[0] != 0)
    }
}

impl Deserialize for HashMap<String, Field> {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut map = HashMap::new();

        let mut byte_offset = 0;

        while byte_offset < from.len() {
            let field_name_length = 'block: {
                for idx in byte_offset..from.len() {
                    if from[idx] == 0 && from[idx - 1] != b'\\' {
                        break 'block Some(idx);
                    }
                }

                None
            }
            .ok_or(IoError::new(
                std::io::ErrorKind::InvalidData,
                "invalid ccb input",
            ))?;

            let field_name = String::deserialize(&from[byte_offset..field_name_length])?;
            // offset + name length +
            byte_offset = field_name_length + 1;
            let field_length = u32::deserialize(
                // add 1 to skip field type
                &from[(byte_offset + 1)..(byte_offset + 1 + mem::size_of::<u32>())],
            )?;
            let field = Field::deserialize(
                &from[byte_offset
                    ..(byte_offset + 1 + mem::size_of::<u32>() + field_length as usize)],
            )?;

            byte_offset += 1 + mem::size_of::<u32>() + field_length as usize;

            map.insert(field_name, field);
        }

        Ok(map)
    }
}
