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
    pub fn new_center<C: Into<Vec3>>(center: C, size: f64) -> Self {
        let center = center.into();
        let diff = ONE * size * 0.5;
        let min = center - diff;
        let max = center + diff;
        Self::new_min_max(min, max)
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        let direction = ray.direction();
        let cmin = (self.vmin - ray.origin()) / direction;
        let cmax = (self.vmax - ray.origin()) / direction;

        let tmin = ((cmin.x.min(cmax.x)).max(cmin.y.min(cmax.y))).max(cmin.z.min(cmax.z));
        let tmax = ((cmin.x.max(cmax.x)).min(cmin.y.max(cmax.y))).min(cmin.z.max(cmax.z));
        if tmax < 0.0 || tmin > tmax {
            return None;
        }
        let point = ray.at(tmin);
        let center = (self.vmin + self.vmax) / 2.0;
        // let rel_c_center = ray.origin() + direction * tmin - (self.vmin + self.vmax) / 2.0;
        //
        // let s = rel_c_center.signum();
        // let a = rel_c_center.abs();
        //
        // let normal: Vec3 = if a.z > a.y {
        //     if a.z > a.x {
        //         (0., 0., s.z)
        //     } else {
        //         (s.x, 0., 0.)
        //     }
        // } else {
        //     if a.y > a.x {
        //         (0., s.y, 0.)
        //     } else {
        //         (s.x, 0., 0.)
        //     }
        // }
        // .into();

        Some(Hit::new(
            ray,
            tmin,
            point,
            (point - Vec3::new(0.0, 0.0, -1.0)).normalize(),
        ))
    }
}
