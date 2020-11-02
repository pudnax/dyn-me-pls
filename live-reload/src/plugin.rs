use libloading::Library;
use std::{os::raw::c_char, path::Path};

/// Represents a loaded instance of our plugin
/// We keep the `Library` together with function pointers
/// so that they go out of scope together.
pub struct Plugin {
    pub greet: unsafe extern "C" fn(name: *const c_char),
    _lib: Library,
}

impl Plugin {
    pub fn load(lib_path: &Path) -> Result<Self, libloading::Error> {
        let _lib = Library::new(lib_path)?;

        Ok(unsafe {
            Plugin {
                greet: *(_lib.get(b"greet")?),
                _lib,
            }
        })
    }
}
