use super::guard::FileGuard;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use libc::{O_RDONLY, c_void, open, read};
use likely_stable::unlikely;
use std::{fs::File, io::Read};
use stringzilla::sz;
extern crate alloc;
use alloc::ffi::CString;

pub fn read_file(file: &str) -> Result<CompactString> {
    let mut file = File::open(file)?;
    let mut buffer = [0u8; 16];
    let _ = file.read(&mut buffer)?;
    let pos = sz::find(buffer, b"\n");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    let buffer = CompactString::from_utf8(buffer)?;
    Ok(buffer)
}

pub fn read_to_byte(file: &str) -> Result<[u8; 16]> {
    let c_file = CString::new(file)?;
    let mut buffer = [0u8; 16];
    unsafe {
        let fd = open(c_file.as_ptr(), O_RDONLY);
        let _fd_guard = FileGuard::new(fd);
        if unlikely(fd == -1) {
            return Err(anyhow!("Cannot open file."));
        }

        let bytes_read = read(fd, buffer.as_mut_ptr().cast::<c_void>(), 16);

        if unlikely(bytes_read == -1) {
            return Err(anyhow!("Cannot read file."));
        }
    }

    Ok(buffer)
}
