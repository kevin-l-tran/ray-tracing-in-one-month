use std::io::{self, Write};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let rbyte = (255.999 * pixel_color.r) as u8;
    let gbyte = (255.999 * pixel_color.g) as u8;
    let bbyte = (255.999 * pixel_color.b) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
