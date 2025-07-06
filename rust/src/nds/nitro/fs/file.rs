use alloc::{boxed::Box, ffi::NulError};
use core::ffi::{CStr, c_char, c_void};

use thiserror::Error;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FSFile {
    _data: [u8; 0x48],
}

unsafe extern "C" {
    pub fn FS_InitFile(p_file: *mut FSFile);
    pub fn FS_OpenFile(p_file: *mut FSFile, path: *const c_char) -> bool;
    pub fn FS_CloseFile(p_file: *mut FSFile) -> bool;
    pub fn FS_ReadFile(p_file: *mut FSFile, dst: *mut c_void, len: i32) -> i32;
}

#[derive(Error, Debug)]
pub enum FileOpenError<'a> {
    #[error("error opening file {path:?}")]
    NitroSdk { path: &'a CStr },
    #[error("path contains a nul byte")]
    PathContainsNul(#[from] NulError),
}
#[derive(Error, Debug)]
pub enum FileCloseError {
    #[error("error closing file")]
    NitroSdk,
}
#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("error reading file")]
    NitroSdk,
}

impl FSFile {
    pub fn new() -> Box<Self> {
        let mut file = Box::new(FSFile { _data: [0; 0x48] });
        unsafe {
            FS_InitFile(&mut *file);
        }
        file
    }

    pub fn open<'a>(&mut self, path: &'a CStr) -> Result<(), FileOpenError<'a>> {
        unsafe { FS_OpenFile(self, path.as_ptr()) }
            .then_some(())
            .ok_or(FileOpenError::NitroSdk { path })
    }

    pub fn close(&mut self) -> Result<(), FileCloseError> {
        if self.is_file() || self.is_dir() {
            unsafe { FS_CloseFile(self) }
                .then_some(())
                .ok_or(FileCloseError::NitroSdk)
        } else {
            Ok(())
        }
    }

    /// # Safety
    /// This function writes directly into `dst` without any additional checks.
    pub unsafe fn read_to_address(&mut self, dst: *mut u8, len: i32) -> Result<i32, FileReadError> {
        let res = unsafe { FS_ReadFile(self, dst.cast(), len) };
        if res == -1 {
            Err(FileReadError::NitroSdk)
        } else {
            Ok(res)
        }
    }
    pub fn read(&mut self, dst: &mut [u8]) -> Result<i32, FileReadError> {
        unsafe {
            self.read_to_address(
                dst.as_mut_ptr(),
                dst.len()
                    .try_into()
                    .expect("file read size doesn't fit into i32"),
            )
        }
    }

    #[inline(always)]
    pub fn stat(&self) -> u32 {
        u32::from_ne_bytes(self._data[0x0C..0x10].try_into().unwrap())
    }
    #[inline(always)]
    pub fn is_file(&self) -> bool {
        self.stat() & 0x00000010 != 0
    }
    #[inline(always)]
    pub fn is_dir(&self) -> bool {
        self.stat() & 0x00000020 != 0
    }

    #[inline(always)]
    fn _file_own_id(&self) -> u32 {
        u32::from_ne_bytes(self._data[0x20..0x24].try_into().unwrap())
    }
    #[inline(always)]
    fn file_top(&self) -> u32 {
        u32::from_ne_bytes(self._data[0x24..0x28].try_into().unwrap())
    }
    #[inline(always)]
    fn file_bottom(&self) -> u32 {
        u32::from_ne_bytes(self._data[0x28..0x2C].try_into().unwrap())
    }
    #[inline(always)]
    fn file_pos(&self) -> u32 {
        u32::from_ne_bytes(self._data[0x2C..0x30].try_into().unwrap())
    }

    #[inline(always)]
    pub fn length(&self) -> u32 {
        self.file_bottom() - self.file_top()
    }
    #[inline(always)]
    pub fn position(&self) -> u32 {
        self.file_pos() - self.file_top()
    }
}
impl Drop for FSFile {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub fn open<'a>(path: &'a CStr) -> Result<Box<FSFile>, FileOpenError<'a>> {
    let mut file = FSFile::new();
    file.open(path)?;
    Ok(file)
}
