use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{ray::Ray, utils::Interval, vec3::Vec3};

use super::{Hit, Hittable};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new<C: Into<Vec3>>(center: C, radius: f64) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        let direction = ray.direction();

        let camera_to_sphere = ray.origin() - self.center;

        let a = direction.length_squared();
        let half_b = camera_to_sphere.dot(direction);
        let c = camera_to_sphere.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !hit_range.surrounds(&root) {
            root = (-half_b + sqrtd) / a;
            if !hit_range.surrounds(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        Some(Hit::new(ray, root, point, normal))
    }
}
