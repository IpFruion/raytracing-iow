use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{ray::Ray, utils::Interval, vec3::Vec3};

use super::{Hit, Hittable};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    center_vec: Option<Vec3>,
    radius: f64,
}

impl Sphere {
    pub fn new<C: Into<Vec3>>(center: C, radius: f64) -> Self {
        Self {
            center: center.into(),
            center_vec: None,
            radius,
        }
    }

    pub fn new_moving<F: Into<Vec3>, T: Into<Vec3>>(from: F, to: T, radius: f64) -> Self {
        let center = from.into();
        Self {
            center,
            center_vec: Some(to.into() - center),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        let center = self.center;
        let center = self
            .center_vec
            .map(|v| center + ray.time() * v)
            .unwrap_or(center);

        let direction = ray.direction();

        let camera_to_sphere = ray.origin() - center;

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
        let normal = (point - center) / self.radius;
        Some(Hit::new(ray, root, point, normal))
    }
}
