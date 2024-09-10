use std::os::fd::RawFd;

pub struct CollectionData {
    descriptor: RawFd,
    num_pages: u64,
    name: String,
}

impl CollectionData {
    pub fn new(fd: RawFd, num_pages: u64, name: String) -> Self {
        Self {
            descriptor: fd,
            num_pages,
            name,
        }
    }

    pub fn descriptor(&self) -> RawFd {
        self.descriptor
    }

    pub fn num_pages(&self) -> u64 {
        self.num_pages
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
