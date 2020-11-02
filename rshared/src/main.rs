use std::{ffi::CString, os::raw::c_char};

#[link(name = "greet")]
extern "C" {
    fn greet(name: *const c_char);
}

fn main() {
    let name = CString::new("fresh coffee").unwrap();
    unsafe {
        greet(name.as_ptr());
    }
}
