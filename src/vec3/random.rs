use std::ops::Range;

use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleRange, SampleUniform, UniformFloat, UniformSampler},
        Standard, Uniform,
    },
    prelude::Distribution,
    Rng,
};

use super::{Vec3, ONE, ZERO};

impl Vec3 {
    pub fn random_unit_sphere<R: Rng>(rng: &mut R) -> Self {
        let mut new: Self = rng.gen_range(-1.0..1.0);
        while new.length() >= 1. {
            new = rng.gen_range(-1.0..1.0);
        }
        new.normalize()
    }
}

pub struct UniformVec3 {
    x: UniformFloat<f64>,
    y: UniformFloat<f64>,
    z: UniformFloat<f64>,
}

impl UniformSampler for UniformVec3 {
    type X = Vec3;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformVec3 {
            x: UniformFloat::new(low.borrow().x, high.borrow().x),
            y: UniformFloat::new(low.borrow().y, high.borrow().y),
            z: UniformFloat::new(low.borrow().z, high.borrow().z),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformVec3 {
            x: UniformFloat::new_inclusive(low.borrow().x, high.borrow().x),
            y: UniformFloat::new_inclusive(low.borrow().y, high.borrow().y),
            z: UniformFloat::new_inclusive(low.borrow().z, high.borrow().z),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec3 {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            z: self.z.sample(rng),
        }
    }
}

impl SampleUniform for Vec3 {
    type Sampler = UniformVec3;
}

impl SampleRange<Vec3> for Range<f64> {
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Vec3 {
        rng.sample(Uniform::new(Vec3::all(self.start), Vec3::all(self.end)))
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.sample(Uniform::new(ZERO, ONE))
    }
}
