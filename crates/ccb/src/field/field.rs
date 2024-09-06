use std::{collections::HashMap, mem, ptr};

use crate::serialize::Serialize;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum FieldType {
    String = 0,
    Byte = 1,
    UByte = 2,
    Int32 = 3,
    UInt32 = 4,
    Int64 = 5,
    UInt64 = 6,
    Float32 = 7,
    Float64 = 8,
    Map = 9,
}

impl Serialize for FieldType {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        Ok(Box::new((*self as u8).to_le_bytes()))
    }

    fn size(&self) -> u32 {
        mem::size_of::<Self>() as u32
    }
}

pub struct Field {
    field_type: FieldType,
    value: Box<dyn Serialize>,
}

impl Serialize for Field {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        // just the binary for value
        let value_buffer = self.value.serialize()?;

        // binary for field type (type + length + value)
        let full_buffer_length = mem::size_of::<u8>() + mem::size_of::<u32>() + value_buffer.len();
        let mut full_buffer = vec![0u8; full_buffer_length].into_boxed_slice();

        unsafe {
            ptr::copy_nonoverlapping(
                (self.field_type as u8).to_le_bytes().as_ptr(),
                full_buffer.as_mut_ptr(),
                1,
            );

            let value_buffer_len = (value_buffer.len() as u32).to_le_bytes();

            ptr::copy_nonoverlapping(
                value_buffer_len.as_ptr(),
                full_buffer.as_mut_ptr().add(1),
                value_buffer_len.len(),
            );

            ptr::copy_nonoverlapping(
                value_buffer.as_ptr(),
                full_buffer
                    .as_mut_ptr()
                    .add(mem::size_of::<u8>() + mem::size_of::<u32>()),
                value_buffer.len(),
            );
        };

        Ok(full_buffer)
    }

    fn size(&self) -> u32 {
        self.value.size()
    }
}

impl Field {
    pub fn string(value: String) -> Self {
        Self {
            field_type: FieldType::String,
            value: Box::new(value),
        }
    }

    pub fn byte(value: i8) -> Self {
        Self {
            field_type: FieldType::Byte,
            value: Box::new(value),
        }
    }

    pub fn ubyte(value: u8) -> Self {
        Self {
            field_type: FieldType::UByte,
            value: Box::new(value),
        }
    }

    pub fn int32(value: i32) -> Self {
        Self {
            field_type: FieldType::Int32,
            value: Box::new(value),
        }
    }

    pub fn uint32(value: u32) -> Self {
        Self {
            field_type: FieldType::UInt32,
            value: Box::new(value),
        }
    }

    pub fn int64(value: i64) -> Self {
        Self {
            field_type: FieldType::Int64,
            value: Box::new(value),
        }
    }

    pub fn uint64(value: u64) -> Self {
        Self {
            field_type: FieldType::UInt64,
            value: Box::new(value),
        }
    }

    pub fn float32(value: f32) -> Self {
        Self {
            field_type: FieldType::Float32,
            value: Box::new(value),
        }
    }

    pub fn float64(value: f64) -> Self {
        Self {
            field_type: FieldType::Float64,
            value: Box::new(value),
        }
    }

    pub fn map_str<'a: 'static>(value: HashMap<&'a str, Field>) -> Self {
        Self {
            field_type: FieldType::Map,
            value: Box::new(value),
        }
    }

    pub fn map(value: HashMap<String, Field>) -> Self {
        Self {
            field_type: FieldType::Map,
            value: Box::new(value),
        }
    }
}
