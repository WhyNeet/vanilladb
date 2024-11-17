use std::{error::Error, mem, ptr};

use trail::{deserialize::Deserialize, serialize::Serialize};

use super::RecordId;

impl Serialize for RecordId {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let size = self.size() as usize;
        let mut buf = vec![0u8; size].into_boxed_slice();
        unsafe {
            ptr::copy_nonoverlapping(
                (size as u32).serialize()?.as_ptr(),
                (&mut buf[..mem::size_of::<u32>()]).as_mut_ptr(),
                mem::size_of::<u32>(),
            );
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.path.serialize()?.as_ptr(),
                (&mut buf[mem::size_of::<u32>()..]).as_mut_ptr(),
                self.path.size() as usize,
            );
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.offset().serialize()?.as_ptr(),
                (&mut buf[(mem::size_of::<u32>() + self.path.len())..]).as_mut_ptr(),
                self.offset.size() as usize,
            );
        }

        Ok(buf)
    }

    fn size(&self) -> u32 {
        mem::size_of::<u32>() as u32 + self.path.size() + self.offset.size()
    }
}

impl Deserialize for RecordId {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        let record_len = u32::deserialize(&from[..mem::size_of::<u32>()])? as usize;
        let path = String::deserialize(
            &from[mem::size_of::<u32>()..(record_len - mem::size_of::<u64>())],
        )?;
        let offset = u64::deserialize(&from[(mem::size_of::<u32>() + path.len())..record_len])?;

        Ok(Self { path, offset })
    }
}
