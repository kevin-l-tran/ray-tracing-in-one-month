use std::io;
use std::io::Write;

use crate::color::{Color, write_color};

mod color;
mod vec3;

fn main() -> io::Result<()> {
    let image_width = 256i32;
    let image_height = 256i32;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().expect("failed to flush");

        for i in 0..image_width {
            let pixel_color = Color {
                r: i as f64 / (image_width - 1) as f64,
                g: j as f64 / (image_height - 1) as f64,
                b: 0.0,
            };

            write_color(&mut out, &pixel_color)?;
        }
    }

    Ok(())
}
