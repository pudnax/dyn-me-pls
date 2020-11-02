use std::{ffi::c_void, ffi::CString, os::raw::c_char, os::raw::c_int};

#[link(name = "dl")]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
    fn dlclose(handle: *const c_void);
}

pub const RTLD_LAZY: c_int = 0x00001;

fn main() {
    let lib_name = CString::new("./target/debug/libgreet.so").unwrap();
    let lib = unsafe { dlopen(lib_name.as_ptr(), RTLD_LAZY) };
    if lib.is_null() {
        panic!("could not open library");
    }

    let greet_name = CString::new("greet").unwrap();
    let greet = unsafe { dlsym(lib, greet_name.as_ptr()) };

    type Greet = unsafe extern "C" fn(name: *const c_char);
    use std::mem::transmute;
    let greet: Greet = unsafe { transmute(greet) };

    let name = CString::new("fresh coffee").unwrap();
    unsafe {
        greet(name.as_ptr());
    }

    unsafe {
        dlclose(lib);
    }
}
