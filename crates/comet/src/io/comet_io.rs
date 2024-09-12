use std::{
    ffi::{c_void, CString},
    io::{self, Error, Write},
    os::fd::RawFd,
    path::PathBuf,
    ptr,
};

use libc::{fstat, open, pread, pwrite, stat, O_CREAT, O_DIRECT, O_RDWR, S_IRUSR, S_IWUSR};

use crate::{
    io::io_config::IoConfig,
    page::{Page, PAGE_SIZE},
};

pub const IO_FLUSH_BUFFER_SIZE: usize = 5;

pub struct CometIo {
    fd: RawFd,
    total_pages: u64,
    flush_buffer: [Option<(*mut Page, u64)>; IO_FLUSH_BUFFER_SIZE],
    flush_buffer_pages: usize,
}

impl CometIo {
    pub fn new(
        db: &str,
        collection: &str,
        config: IoConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let collection_file_path = PathBuf::from(&config.data_dir()[..])
            .join(db)
            .join(collection);
        let (fd, size) = CometIo::get_file_data(collection_file_path);
        let total_pages = size / PAGE_SIZE as u64;

        Ok(Self {
            fd,
            total_pages,
            flush_buffer: [None; IO_FLUSH_BUFFER_SIZE],
            flush_buffer_pages: 0,
        })
    }

    fn get_file_data(path: PathBuf) -> (RawFd, u64) {
        let path = CString::new(path.to_str().unwrap()).unwrap();
        let fd = unsafe {
            open(
                path.as_ptr(),
                O_CREAT | O_RDWR | O_DIRECT,
                S_IRUSR | S_IWUSR,
            )
        };

        let mut file_stat: stat = unsafe { std::mem::zeroed() };
        unsafe { fstat(fd, &mut file_stat) };

        (fd, file_stat.st_size as u64)
    }
}

impl CometIo {
    pub fn total_pages(&self) -> u64 {
        self.total_pages
    }

    fn flush_pages(&mut self) -> io::Result<()> {
        for page in self.flush_buffer.iter().take_while(|page| page.is_some()) {
            let (page, idx) = page.unwrap();

            let bytes_written = unsafe {
                pwrite(
                    self.fd,
                    page.as_ref().unwrap().buffer().as_ptr() as *const c_void,
                    PAGE_SIZE,
                    (idx * PAGE_SIZE as u64) as i64,
                )
            };

            if bytes_written < 0 {
                return Err(Error::last_os_error());
            }

            unsafe { page.as_mut().unwrap() }.flush()?;
        }

        self.flush_buffer = [None; IO_FLUSH_BUFFER_SIZE];
        self.flush_buffer_pages = 0;

        Ok(())
    }

    pub fn flush_collection_page(
        &mut self,
        idx: u64,
        page: &mut crate::page::Page,
    ) -> std::io::Result<()> {
        if self.flush_buffer_pages == self.flush_buffer.len() {
            self.flush_pages()?;
        }

        self.flush_buffer[self.flush_buffer_pages] = Some((page, idx));
        self.flush_buffer_pages += 1;

        Ok(())
    }

    pub fn load_collection_page(&self, idx: u64) -> io::Result<Page> {
        let mut buffer = Box::new([0u8; PAGE_SIZE]);
        unsafe {
            pread(
                self.fd,
                buffer.as_mut_ptr() as *mut c_void,
                PAGE_SIZE,
                (PAGE_SIZE as u64 * idx) as i64,
            )
        };

        let page = Page::from_buffer(buffer);

        Ok(page)
    }
}

impl Drop for CometIo {
    fn drop(&mut self) {
        self.flush_pages().unwrap();
    }
}
