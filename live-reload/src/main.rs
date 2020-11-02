use argh::FromArgs;
use cstr::cstr;
use libloading::{Library, Symbol};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    error::Error,
    ffi::CString,
    os::raw::c_char,
    path::{Path, PathBuf},
};

mod plugin;
use plugin::Plugin;

compromise::register!();

#[derive(FromArgs)]
/// Greet
struct Args {
    /// whether "hot reloading" should be enabled
    #[argh(switch)]
    watch: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();
    println!("{:?}", args.watch);
    compromise::set_hot_reload_enabled(args.watch);
    if args.watch {
        println!("Hot reloading enabled - there will be memory leaks!");
    }

    let base = PathBuf::from("target").canonicalize().unwrap();
    let libname = "libgreet.so";
    let relative_path = PathBuf::from("debug").join(libname);
    let absolute_path = base.join(&relative_path);

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate({
        move |res: Result<notify::Event, _>| match res {
            Ok(event) => {
                if let notify::EventKind::Create(_) = event.kind {
                    if event.paths.iter().any(|x| x.ends_with(&relative_path)) {
                        tx.send(()).unwrap();
                    }
                }
            }
            Err(e) => println!("watch error: {}", e),
        }
    })
    .unwrap();
    watcher.watch(&base, RecursiveMode::Recursive).unwrap();

    let mut plugin = Some(Plugin::load(&absolute_path).unwrap());
    let start = std::time::SystemTime::now();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        if rx.try_recv().is_ok() {
            println!("==== Reloading ====");
            plugin = Some(Plugin::load(&absolute_path)?);
        }

        if let Some(plugin) = plugin.as_ref() {
            let s = format!("We've been running for {:?}", start.elapsed().unwrap());
            let s = CString::new(s)?;
            unsafe { (plugin.greet)(s.as_ptr()) };
        }
    }
}

fn _step(lib_path: &Path) -> Result<(), libloading::Error> {
    let lib = Library::new(lib_path)?;
    unsafe {
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        #[allow(clippy::transmute_ptr_to_ref)]
        greet(cstr!("saturday").as_ptr());
    }
    Ok(())
}
