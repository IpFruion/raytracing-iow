use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul},
};

use crate::{utils::Interval, vec3::Vec3};

pub const BLACK: Color = Color::new(0., 0., 0.);
pub const WHITE: Color = Color::new(1., 1., 1.);
pub const SKY_BLUE: Color = Color::new(0.5, 0.7, 1.);

#[derive(Debug, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn into_arr(self) -> [f64; 3] {
        [self.0.x, self.0.y, self.0.z]
    }

    pub fn clamp<I: Interval<f64>>(self, interval: I) -> Self {
        Self(Vec3::new(
            interval.clamp(self.0.x),
            interval.clamp(self.0.y),
            interval.clamp(self.0.z),
        ))
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(BLACK)
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        Self(value.into())
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(rhs * self.0)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel = &self.0;
        write!(
            f,
            "{} {} {}",
            (pixel.x * 255.) as u8,
            (pixel.y * 255.) as u8,
            (pixel.z * 255.) as u8
        )
    }
}
