use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();

        assert!(length > 0.0, "cannot normalize a zero-length vector");

        Vec3 {
            x: (self.x / length),
            y: (self.y / length),
            z: (self.z / length),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, num: f64) -> Self {
        Self {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, num: f64) {
        *self = Self {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, num: f64) -> Self {
        assert!(num > 0.0, "cannot divide by 0");

        Self {
            x: self.x / num,
            y: self.y / num,
            z: self.z / num,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, num: f64) {
        assert!(num > 0.0, "cannot divide by 0");

        *self = Self {
            x: self.x / num,
            y: self.y / num,
            z: self.z / num,
        }
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.x * v2.x + v1.y + v2.y + v1.z + v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}
