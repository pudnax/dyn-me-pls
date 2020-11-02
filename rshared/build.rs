use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir should be set"));
    let lib_dir = manifest_dir
        .parent()
        .expect("manifesst dir should have a parent");
    println!("cargo:rustc-link-search={}", lib_dir.display());
}
