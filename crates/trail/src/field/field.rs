use std::{collections::HashMap, ffi::c_void, mem, ptr};

use crate::{deserialize::Deserialize, serialize::Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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

impl Deserialize for FieldType {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(match from[0] {
            0 => FieldType::String,
            1 => FieldType::Byte,
            2 => FieldType::UByte,
            3 => FieldType::Int32,
            4 => FieldType::UInt32,
            5 => FieldType::Int64,
            6 => FieldType::UInt64,
            7 => FieldType::Float32,
            8 => FieldType::Float64,
            9 => FieldType::Map,
            _ => unreachable!(),
        })
    }
}

impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        self.serialize().unwrap() == other.serialize().unwrap()
    }
}

#[derive(Debug)]
pub struct Field {
    field_type: FieldType,
    value: Box<dyn Serialize>,
}

impl Serialize for Field {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        // just the binary for value
        let value_buffer = self.value.serialize()?;
        let mut full_buffer = vec![0u8; self.size() as usize].into_boxed_slice();

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
        self.field_type.size() + mem::size_of::<u32>() as u32 + self.value.size()
    }
}

impl Deserialize for Field {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let field_type = FieldType::deserialize(&[from[0]])?;
        let field_length = u32::deserialize(&from[1..5])?;

        let field_start: usize = 5;
        let field_end = field_start + field_length as usize;

        let value: Box<dyn Serialize> = match field_type {
            FieldType::String => Box::new(String::deserialize(&from[field_start..field_end])?),
            FieldType::Byte => Box::new(i8::deserialize(&from[field_start..field_end])?),
            FieldType::UByte => Box::new(u8::deserialize(&from[field_start..field_end])?),
            FieldType::Int32 => Box::new(i32::deserialize(&from[field_start..field_end])?),
            FieldType::UInt32 => Box::new(u32::deserialize(&from[field_start..field_end])?),
            FieldType::Int64 => Box::new(i64::deserialize(&from[field_start..field_end])?),
            FieldType::UInt64 => Box::new(u64::deserialize(&from[field_start..field_end])?),
            FieldType::Float32 => Box::new(f32::deserialize(&from[field_start..field_end])?),
            FieldType::Float64 => Box::new(f64::deserialize(&from[field_start..field_end])?),
            FieldType::Map => Box::new(HashMap::<String, Field>::deserialize(
                &from[field_start..field_end],
            )?),
        };

        Ok(Self { field_type, value })
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

    pub fn value(&self) -> &Box<dyn Serialize> {
        &self.value
    }

    pub fn value_as_string(&self) -> &str {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const String).as_ref().unwrap() }
    }

    pub fn value_as_byte(&self) -> &i8 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const i8).as_ref().unwrap() }
    }

    pub fn value_as_ubyte(&self) -> &u8 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const u8).as_ref().unwrap() }
    }

    pub fn value_as_int32(&self) -> &i32 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const i32).as_ref().unwrap() }
    }

    pub fn value_as_uint32(&self) -> &u32 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const u32).as_ref().unwrap() }
    }

    pub fn value_as_int64(&self) -> &i64 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const i64).as_ref().unwrap() }
    }

    pub fn value_as_uint64(&self) -> &u64 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const u64).as_ref().unwrap() }
    }

    pub fn value_as_float32(&self) -> &f32 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const f32).as_ref().unwrap() }
    }

    pub fn value_as_float64(&self) -> &f64 {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const f64).as_ref().unwrap() }
    }

    pub fn value_as_str_map(&self) -> &HashMap<&str, Field> {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const HashMap<&str, Field>).as_ref().unwrap() }
    }

    pub fn value_as_map(&self) -> &HashMap<String, Field> {
        let ptr = self.value.as_ref() as *const _ as *const c_void;

        unsafe { (ptr as *const HashMap<String, Field>).as_ref().unwrap() }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        if self.field_type != other.field_type {
            return false;
        }

        match self.field_type {
            FieldType::String => self.value_as_string() == other.value_as_string(),
            FieldType::Byte => self.value_as_byte() == other.value_as_byte(),
            FieldType::UByte => self.value_as_ubyte() == other.value_as_ubyte(),
            FieldType::Int32 => self.value_as_int32() == other.value_as_int32(),
            FieldType::UInt32 => self.value_as_uint32() == other.value_as_uint32(),
            FieldType::Int64 => self.value_as_int64() == other.value_as_int64(),
            FieldType::UInt64 => self.value_as_uint64() == other.value_as_uint64(),
            FieldType::Float32 => self.value_as_float32() == other.value_as_float32(),
            FieldType::Float64 => self.value_as_float64() == other.value_as_float64(),
            FieldType::Map => self.value_as_str_map() == other.value_as_str_map(),
        }
    }
}
