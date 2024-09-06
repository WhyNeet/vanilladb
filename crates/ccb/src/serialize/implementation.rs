use std::mem;

use super::macro_impl::serializable_number;

serializable_number!(for usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

impl Serialize for String {
    fn serialize(self) -> Result<Box<[u8]>, Box<dyn Error>> {
        Ok(self.into_bytes().into_boxed_slice())
    }

    fn size(&self) -> u32 {
        self.len() as u32
    }
}

impl Serialize for bool {
    fn serialize(self) -> Result<Box<[u8]>, Box<dyn Error>> {
        Ok(Box::new([self as u8]))
    }

    fn size(&self) -> u32 {
        mem::size_of::<Self>() as u32
    }
}
