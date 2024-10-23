use std::{collections::HashMap, mem, ptr};

use crate::{deserialize::Deserialize, field::Field};

use super::macro_impl::serializable_number;

serializable_number!(for u128, u64, u32, u16, u8, i128, i64, i32, i16, i8, f64, f32);

impl Serialize for String {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        Ok(self.bytes().collect())
    }

    fn size(&self) -> u32 {
        self.len() as u32
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        Ok(Box::new([*self as u8]))
    }

    fn size(&self) -> u32 {
        mem::size_of::<Self>() as u32
    }
}

/*

binary document field repr:

field_name (String) + \0
field_type (u8)
field_value_length (u32)
field_value ([u8])

*/

impl Serialize for HashMap<&str, Field> {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let size = self.size();
        let mut buffer = vec![0u8; size as usize].into_boxed_slice();

        let mut bytes_written: u32 = 0;
        for (key, value) in self.iter() {
            // append a null terminator
            let key = format!("{key}\0");
            unsafe {
                ptr::copy_nonoverlapping(
                    key.as_ptr(),
                    buffer.as_mut_ptr().add(bytes_written as usize),
                    key.len(),
                );
            };

            bytes_written += key.len() as u32;

            let value = value.serialize()?;
            unsafe {
                ptr::copy_nonoverlapping(
                    value.as_ptr(),
                    buffer.as_mut_ptr().add(bytes_written as usize),
                    value.len(),
                );
            };

            bytes_written += value.len() as u32;
        }

        Ok(buffer)
    }

    fn size(&self) -> u32 {
        self.iter()
            // acc + field_name + \0 + field_type + field_value_length + field_value
            .fold(0, |acc, (key, v)| {
                acc + (key.len() as u32)
                    + 1
                    + mem::size_of::<u8>() as u32
                    + mem::size_of::<u32>() as u32
                    + v.size()
            })
    }
}

impl Serialize for HashMap<String, Field> {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let size = self.size();
        let mut buffer = vec![0u8; size as usize].into_boxed_slice();

        let mut bytes_written: u32 = 0;
        for (key, value) in self.iter() {
            // append a null terminator
            let key = format!("{key}\0");
            unsafe {
                ptr::copy_nonoverlapping(
                    key.as_ptr(),
                    buffer.as_mut_ptr().add(bytes_written as usize),
                    key.len(),
                );
            };

            bytes_written += key.len() as u32;

            let value = value.serialize()?;
            unsafe {
                ptr::copy_nonoverlapping(
                    value.as_ptr(),
                    buffer.as_mut_ptr().add(bytes_written as usize),
                    value.len(),
                );
            };

            bytes_written += value.len() as u32;
        }

        Ok(buffer)
    }

    fn size(&self) -> u32 {
        self.iter()
            // acc + field_name + \0 + field_type + field_value_length + field_value
            .fold(0, |acc, (key, v)| {
                acc + (key.len() as u32)
                    + 1
                    + mem::size_of::<u8>() as u32
                    + mem::size_of::<u32>() as u32
                    + v.size()
            })
    }
}

impl Serialize for Vec<Field> {
    fn size(&self) -> u32 {
        mem::size_of::<u32>() as u32
            + self
                .iter()
                .map(|field| {
                    mem::size_of::<u8>() as u32 + mem::size_of::<u32>() as u32 + field.size()
                })
                .sum::<u32>()
    }

    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let size = self.size();
        let mut buffer = vec![0u8; size as usize].into_boxed_slice();

        unsafe {
            ptr::copy_nonoverlapping(
                size.to_le_bytes().as_ptr(),
                buffer.as_mut_ptr(),
                mem::size_of::<u32>(),
            );
        }

        let mut offset = 0;

        for field in self.iter() {
            let len = mem::size_of::<u8>() as u32 + mem::size_of::<u32>() as u32 + field.size();
            unsafe {
                ptr::copy_nonoverlapping(
                    field.serialize()?.as_ptr(),
                    (&mut buffer[(mem::size_of::<u32>() + offset as usize)..]).as_mut_ptr(),
                    len as usize,
                );
            }
            offset += len;
        }

        Ok(buffer)
    }
}

impl Deserialize for Vec<Field> {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        let size = u32::from_le_bytes((&from[..4]).try_into()?);
        let mut vec = Vec::new();

        let mut offset = 0;

        while size - offset > 4 {
            let field = Field::deserialize(&from[(4 + offset as usize)..])?;
            let size = field.size();

            vec.push(field);

            offset += mem::size_of::<u8>() as u32 + mem::size_of::<u32>() as u32 + size;
        }

        Ok(vec)
    }
}
