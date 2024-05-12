//! an **experimental** api to the micro-sd card of the vex v5 brain

use alloc::{boxed::Box, vec, string::ToString};
use libc::{c_void, fclose, fopen, fprintf, fread, fseek, ftell, fwrite, FILE, SEEK_END, SEEK_SET};

/// Creates a `C` compatible string from a rust one
#[macro_export]
macro_rules! cstr {
    ($str:expr) => {{
        let mut bytes = $str.as_bytes().to_vec();
        bytes.push(0u8);
        bytes.into_boxed_slice()
            .as_ptr()
            // as *const i8
    }}
}

/// A safer wrapper around a file stream
pub struct File(pub(crate) *mut FILE);

impl File {
    /// Creates a **new** file at the specified location with only write permissions, returns None on failure
    #[inline]
    pub fn create(path: &str) -> Option<Self> {
        let file = unsafe { fopen(cstr!("/usd/".to_string() + path), cstr!("wb")) };
        if file.is_null() { return None };
        Some(File(file))
    }

    /// Opens an **existing** file at the specified location with only read permissions, returns None of failure
    #[inline]
    pub fn open(path: &str) -> Option<Self> {
        let file = unsafe { fopen(cstr!("/usd/".to_string() + path), cstr!("rb")) };
        if file.is_null() { return None };
        Some(File(file))
    }

    /// Reads the entire contents of a file, returns None upon error
    #[inline]
    pub fn read_file(self) -> Option<Box<[u8]>> {
        unsafe {
            // get file length
            fseek(self.0, 0, SEEK_END);
            let len = ftell(self.0);
            fseek(self.0, 0, SEEK_SET);

            if len < 0 { return None };

            // read self.0 contents
            let contents = vec![0u8; len as usize].into_boxed_slice();
            fread(contents.as_ptr() as *mut c_void, 1, len as usize, self.0);

            // close self.0
            fclose(self.0);

            Some(contents)
        }
    }

    /// Writes a string to the file, returns error code if there is one
    #[inline]
    pub fn write_str(&mut self, string: &str) -> i32 {
        unsafe { fprintf(self.0, cstr!(string)) }
    }

    /// Writes binary to a file
    #[inline]
    pub fn write(&mut self, buffer: &[u8]) {
        unsafe { fwrite(buffer.as_ptr() as *const c_void, buffer.len(), 1, self.0) };
    }

    /// Provides unsafe access to the internal file-stream (you should know what you're doing)
    #[inline]
    pub unsafe fn internal_fs(&self) -> *mut FILE {
        self.0
    }
    
    /// Closes a the file-stream, returns error code if there is one
    #[inline]
    pub fn finish(self) -> i32 {
        unsafe { fclose(self.0) }
    }
}

impl Drop for File {
    #[inline]
    fn drop(&mut self) {
        unsafe { fclose(self.0) };
    }
}
