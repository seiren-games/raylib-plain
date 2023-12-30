use std::ffi::{CString, c_char};

pub fn str_to_c_char(s: &str) -> *const c_char {
    // Convert Rust string to C string
    let c_string = CString::new(s).unwrap();

    // Get a pointer and keep it from losing its ownership
    let ptr = c_string.as_ptr();
    std::mem::forget(c_string);

    ptr
}
