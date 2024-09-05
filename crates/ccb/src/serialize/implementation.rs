use super::macro_impl::serializable_number;

serializable_number!(for usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

impl Serialize for String {
    fn serialize(self) -> Box<[u8]> {
        self.into_bytes().into_boxed_slice()
    }
}

impl Serialize for bool {
    fn serialize(self) -> Box<[u8]> {
        Box::new([self as u8])
    }
}
