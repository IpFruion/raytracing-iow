use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{assign_math, clone_math};

use super::Vec3;

impl Vec3 {
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2. * self.dot(n) * n
    }

    pub fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(n).min(1.);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

assign_math!(Vec3, AddAssign, add_assign, +=);
assign_math!(Vec3, SubAssign, sub_assign, -=);
assign_math!(Vec3, MulAssign, mul_assign, *=);
assign_math!(Vec3, DivAssign, div_assign, /=);

clone_math!(Vec3, Add, add, +);
clone_math!(Vec3, Sub, sub, -);
clone_math!(Vec3, Mul, mul, *);
clone_math!(Vec3, Div, div, /);

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs + self.x,
            y: rhs + self.y,
            z: rhs + self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        rhs.recip() * self
    }
}
