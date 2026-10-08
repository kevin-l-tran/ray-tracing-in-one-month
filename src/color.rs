use std::{
    io::{self, Write},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{interval::Interval, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, vec: Color) -> Color {
        Color {
            r: vec.r * self,
            g: vec.g * self,
            b: vec.b * self,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, num: f64) -> Self {
        Self {
            r: self.r * num,
            g: self.g * num,
            b: self.b * num,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, num: f64) {
        *self = Self {
            r: self.r * num,
            g: self.g * num,
            b: self.b * num,
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color {
            r: value.x,
            g: value.y,
            b: value.z,
        }
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let intensity = Interval::new(0.0, 0.9999999999999999);

    let rbyte = (255.0 * intensity.clamp(pixel_color.r)) as u8;
    let gbyte = (255.0 * intensity.clamp(pixel_color.g)) as u8;
    let bbyte = (255.0 * intensity.clamp(pixel_color.b)) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
