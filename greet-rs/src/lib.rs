use common::{FrameContext, Pixel};
use std::{ffi::CStr, os::raw::c_char};

/// # Safety
/// Pointer must be valid, and point to a null-terminated
/// string. What happens othervise is UB.
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) {
    let cstr = CStr::from_ptr(name);
    println!("Hello, {}! Kappa", cstr.to_str().unwrap());
}

struct Canvas<'a> {
    cx: &'a mut FrameContext,
}

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn dist(&self, other: Self) -> f64 {
        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let x2 = other.x as f64;
        let y2 = other.y as f64;

        let dx = x2 - x1;
        let dy = y2 - y1;
        (dx * dx + dy * dy).sqrt()
    }
}

impl<'a> Canvas<'a> {
    fn width(&self) -> isize {
        self.cx.width as isize
    }
    fn height(&self) -> isize {
        self.cx.height as isize
    }
    fn point(&mut self, Point { x, y }: Point, color: Pixel) {
        if x < 0 || x >= self.width() {
            return;
        }
        if y < 0 || y >= self.height() {
            return;
        }

        let index = x + y * self.width();
        let p = &mut self.cx.pixels()[index as usize];

        *p = color;
    }

    fn clear(&mut self, color: Pixel) {
        for p in self.cx.pixels() {
            *p = color;
        }
    }
}

#[no_mangle]
pub extern "C" fn draw(cx: &mut FrameContext) {
    let mut canvas = Canvas { cx };

    canvas.clear(Pixel {
        r: 80,
        g: 80,
        b: 80,
        z: 0,
    });

    let pos_x = (canvas.cx.ticks / 8) % canvas.width() as usize;
    let pos_y = canvas.cx.height / 2;
    let side = 40;

    for x in pos_x..(pos_x + side) {
        for y in pos_y..(pos_y + side) {
            canvas.point(
                Point {
                    x: x as _,
                    y: y as _,
                },
                Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                    z: 0,
                },
            )
        }
    }

    let center = Point {
        x: (canvas.cx.width / 2) as _,
        y: (canvas.cx.height / 2) as _,
    };

    for x in 0..canvas.width() as isize {
        for y in 0..canvas.height() as isize {
            let pos = Point { x, y };
            let dist = pos.dist(center);
            let offset = (canvas.cx.ticks as f64 * 0.01).sin() * 3.;
            if dist > 50.0 - offset.powf(2.) && dist < 160. + offset {
                canvas.point(
                    pos,
                    Pixel {
                        r: 80,
                        g: 80,
                        b: 180,
                        z: 0,
                    },
                );
            }
        }
    }
}
