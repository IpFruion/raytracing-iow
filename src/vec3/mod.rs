use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::ray::Ray;

pub mod math;
pub mod random;

pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
pub const ONE: Vec3 = Vec3::new(1., 1., 1.);

// Vec3 represents color, locations, directions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn all(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(&self, n: Self) -> Self {
        *self - 2. * self.dot(n) * n
    }

    pub fn refract(&self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(n).min(1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn ray(&self, direction: Self) -> Ray {
        Ray::new(*self, direction)
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        <f64 as AbsDiffEq>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        <f64 as AbsDiffEq>::abs_diff_eq(&self.x, &other.x, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.y, &other.y, epsilon)
            && <f64 as AbsDiffEq>::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl RelativeEq for Vec3 {
    fn default_max_relative() -> Self::Epsilon {
        <f64 as RelativeEq>::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        <f64 as RelativeEq>::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && <f64 as RelativeEq>::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        <f64 as UlpsEq>::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        <f64 as UlpsEq>::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && <f64 as UlpsEq>::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && <f64 as UlpsEq>::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}
