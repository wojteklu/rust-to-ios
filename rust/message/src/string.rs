use std::ffi::CStr;
use libc::{c_char, size_t};
use std::string::String;

pub trait FromChar {
    fn from_char(char: *const c_char) -> String;
}

impl FromChar for String {
    fn from_char(char: *const c_char) -> String {
        let c_str: &CStr = unsafe { CStr::from_ptr(char) };
        let byte_slice = c_str.to_bytes().to_vec();

        match String::from_utf8(byte_slice) {
            Ok(s)  => return s,
            Err(_) => return String::from(""),
        }
    }
}

#[repr(C)]
pub struct ByteSlice {
    pub bytes: *const u8,
    pub length: size_t,
}

impl<'a> From<&'a str> for ByteSlice {
    fn from(s: &'a str) -> Self {
        ByteSlice{
            bytes: s.as_ptr(),
            length: s.len() as size_t,
        }
    }
}
