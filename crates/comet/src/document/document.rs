use core::str;
use std::{collections::HashMap, error::Error, mem, ptr};

use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[derive(Debug)]
pub struct Document {
    map: HashMap<String, Field>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn append_field(&mut self, key: String, value: Field) -> &mut Self {
        self.map.insert(key, value);
        self
    }

    pub fn get_field(&self, key: &str) -> Option<&Field> {
        self.map.get(key)
    }

    pub fn remove_field(&mut self, key: &str) -> Option<Field> {
        self.map.remove(key)
    }

    pub fn new_with_fields(map: HashMap<String, Field>) -> Self {
        Document { map }
    }

    pub fn size(&self) -> u32 {
        self.map.size()
    }

    pub fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let size = self.map.size();
        let mut buffer = vec![0u8; mem::size_of::<u32>() + size as usize].into_boxed_slice();

        let size = size.to_le_bytes();
        unsafe { ptr::copy_nonoverlapping(size.as_ptr(), buffer.as_mut_ptr(), size.len()) };

        let data = self.map.serialize()?;
        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                buffer.as_mut_ptr().add(mem::size_of::<u32>()),
                data.len(),
            )
        };

        Ok(buffer)
    }

    pub fn deserialize(&mut self, src: &[u8]) -> Result<Self, Box<dyn Error>> {
        let map = HashMap::<String, Field>::deserialize(&src[mem::size_of::<u32>()..])?;

        Ok(Document { map })
    }
}
