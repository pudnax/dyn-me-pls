use std::os::raw::c_char;

use libloading::{Library, Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib = Library::new("./target/debug/libgreet.so")?;
    unsafe {
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        greet(cstr::cstr!("fresh 3p macros").as_ptr());
    }
    Ok(())
}
