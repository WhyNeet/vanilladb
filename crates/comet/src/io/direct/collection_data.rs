use std::os::fd::RawFd;

pub struct CollectionData {
    descriptor: RawFd,
}

impl CollectionData {
    pub fn new(fd: RawFd) -> Self {
        Self { descriptor: fd }
    }
    pub fn descriptor(&self) -> RawFd {
        self.descriptor
    }
}
