use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{assign_math, clone_math};

use super::Vec3;

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

clone_math!(Vec3, Add, add, +=);
clone_math!(Vec3, Sub, sub, -=);
clone_math!(Vec3, Mul, mul, *=);

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
