use common::FrameContext;
use libloading::Library;
use std::path::Path;

pub struct Plugin {
    pub draw: extern "C" fn(fc: &mut FrameContext),
    _lib: Library,
}

impl Plugin {
    pub fn load(lib_path: &Path) -> Result<Self, libloading::Error> {
        let _lib = Library::new(lib_path)?;

        Ok(unsafe {
            Plugin {
                draw: *(_lib.get(b"draw")?),
                _lib,
            }
        })
    }
}
