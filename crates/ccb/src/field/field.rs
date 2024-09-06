use std::{collections::HashMap, mem};

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
        self.value.serialize()
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
}
