use std::{
    fmt::Display,
    ops::{Add, AddAssign, DivAssign, Sub, SubAssign},
};

use image::Rgb;

pub const EPSILON: f64 = 1e-6;

#[derive(Clone, Copy)]
pub struct Pixel(pub f32, pub f32, pub f32);

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl DivAssign<u8> for Pixel {
    fn div_assign(&mut self, rhs: u8) {
        self.0 /= rhs as f32;
        self.1 /= rhs as f32;
        self.2 /= rhs as f32;
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixel {{r: {} g: {} b: {} }}", self.0, self.1, self.2)
    }
}
