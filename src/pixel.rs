use std::{
    fmt::Display,
    ops::{Add, AddAssign, Bound, Mul, RangeBounds},
};

use crate::{utils::Interval, vec3::Vec3};

pub const WHITE: Pixel = Pixel::new(1., 1., 1.);
pub const BLACK: Pixel = Pixel::new(0., 0., 0.);
pub const SKY_BLUE: Pixel = Pixel::new(0.5, 0.7, 1.);
pub const ORANGE: Pixel = Pixel::new(250. / 255., 121. / 255., 35. / 255.);
pub const PURPLE: Pixel = Pixel::new(71. / 255., 5. / 255., 158. / 255.);

#[derive(Debug, Clone, Copy)]
pub struct Pixel(Vec3);

impl Pixel {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn clamp<I: Interval<f64>>(self, interval: I) -> Self {
        Self(Vec3::new(
            interval.clamp(self.0.x),
            interval.clamp(self.0.y),
            interval.clamp(self.0.z),
        ))
    }
}

impl From<Vec3> for Pixel {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<(f64, f64, f64)> for Pixel {
    fn from(value: (f64, f64, f64)) -> Self {
        Self(value.into())
    }
}

impl Mul<f64> for Pixel {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Pixel(rhs * self.0)
    }
}

impl Mul<Pixel> for f64 {
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output {
        rhs * self
    }
}

impl Mul for Pixel {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Add for Pixel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pixel(self.0 + rhs.0)
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Display for Pixel {
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
