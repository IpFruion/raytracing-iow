use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{
    ray::Ray,
    vec3::{Vec3, ONE},
};

use super::{Hit, Hittable};

pub struct Cube {
    vmin: Vec3,
    vmax: Vec3,
}

impl Cube {
    pub fn new_min_max(vmin: Vec3, vmax: Vec3) -> Self {
        Self { vmin, vmax }
    }
    pub fn new_center(center: Vec3, size: f64) -> Self {
        let diff = ONE * size * 0.5;
        let min = center - diff;
        let max = center + diff;
        Self::new_min_max(min, max)
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        todo!()
    }
}
