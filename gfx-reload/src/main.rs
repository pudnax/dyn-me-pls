use argh::FromArgs;
use common::{FrameContext, Pixel};
use minifb::{Key, Window, WindowOptions};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();
    compromise::set_hot_reload_enabled(args.watch);
    if args.watch {
        println!("Hot reloading enabled - there will be memory leaks!");
    }

    let base = PathBuf::from("target").canonicalize()?;
    let libname = "libgreet.so";

    #[cfg(debug_assertions)]
    let target_path = PathBuf::from("debug").join(libname);

    #[cfg(not(debug_assertions))]
    let target_path = PathBuf::from("release").join(libname);

    let absolute_path = base.join(&target_path);

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate({
        move |res: Result<notify::Event, _>| match res {
            Ok(event) => {
                if let notify::EventKind::Create(_) = event.kind {
                    if event.paths.iter().any(|x| x.ends_with(&target_path)) {
                        tx.send(()).unwrap();
                    }
                }
            }
            Err(e) => println!("watch error: {}", e),
        }
    })?;
    watcher.watch(&base, RecursiveMode::Recursive)?;

    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(WIDTH * HEIGHT);
    for _ in 0..pixels.capacity() {
        pixels.push(Pixel {
            z: 0,
            r: 0,
            g: 0,
            b: 0,
        });
    }

    let mut window = Window::new("Playground", WIDTH, HEIGHT, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut plugin = Some(Plugin::load(&absolute_path)?);
    let start = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if rx.try_recv().is_ok() {
            plugin = None;
            println!("==== Reloading ====");
            plugin = Some(Plugin::load(&absolute_path)?);
        }

        if let Some(plugin) = plugin.as_ref() {
            let mut cx = FrameContext {
                width: WIDTH,
                height: HEIGHT,
                pixels: &mut pixels[0],
                ticks: start.elapsed().unwrap().as_millis() as usize,
            };
            (plugin.draw)(&mut cx)
        }

        window.update_with_buffer(
            #[allow(clippy::transmute_ptr_to_ptr)]
            unsafe {
                std::mem::transmute(pixels.as_slice())
            },
            WIDTH,
            HEIGHT,
        )?;
    }

    Ok(())
}
