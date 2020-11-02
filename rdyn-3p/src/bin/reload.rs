use cstr::cstr;
use std::{error::Error, os::raw::c_char};

use libloading::{Library, Symbol};

use argh::FromArgs;

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
    compromise::set_hot_reload_enabled(args.watch);
    if args.watch {
        println!("Hot reloading enabled = there will be memore leaks!");
    }

    std::thread::spawn(run).join().unwrap();
    Ok(())
}

fn run() {
    let mut line = String::new();
    let stdin = std::io::stdin();

    println!("Here we go!");

    let n = 3;
    for _ in 0..n {
        load_and_print().unwrap();

        println!("-----------------------------");
        println!("Press Enter to go again, Ctrl-C to exit...");

        line.clear();
        stdin.read_line(&mut line).unwrap();
    }

    println!("Did {} rounds, stopping", n);
}

fn load_and_print() -> Result<(), libloading::Error> {
    let lib = Library::new("libgreet.so")?;
    unsafe {
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        greet(cstr!("reloading").as_ptr());
    }

    Ok(())
}
