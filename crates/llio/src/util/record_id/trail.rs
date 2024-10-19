use std::{error::Error, mem, ptr};

use trail::{deserialize::Deserialize, serialize::Serialize};

use super::RecordId;

impl Serialize for RecordId {
    fn serialize(&self) -> Result<Box<[u8]>, Box<dyn Error>> {
        let mut buf = vec![0u8; 32 as usize].into_boxed_slice();
        unsafe {
            ptr::copy_nonoverlapping(
                (self.path().len() as u16).serialize()?.as_ptr(),
                (&mut buf[..2]).as_mut_ptr(),
                mem::size_of::<u16>(),
            );
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.path.serialize()?.as_ptr(),
                (&mut buf[2..]).as_mut_ptr(),
                self.path.size() as usize,
            );
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.offset().serialize()?.as_ptr(),
                (&mut buf[(2 + self.path.len())..]).as_mut_ptr(),
                self.offset.size() as usize,
            );
        }

        Ok(buf)
    }

    fn size(&self) -> u32 {
        mem::size_of::<u16>() as u32 + self.path.size() + self.offset.size()
    }
}

impl Deserialize for RecordId {
    fn deserialize(from: &[u8]) -> Result<Self, Box<dyn Error>> {
        let path_len = u16::deserialize(&from[..mem::size_of::<u16>()])?;
        let path = String::deserialize(
            &from[mem::size_of::<u16>()..(mem::size_of::<u16>() + path_len as usize)],
        )?;
        let offset = u64::deserialize(&from[(mem::size_of::<u16>() + path_len as usize)..])?;

        Ok(Self { path, offset })
    }
}
