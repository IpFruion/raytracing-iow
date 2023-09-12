use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{
    material::Material,
    ray::Ray,
    render::hit::{HitRecord, Hittable},
    utils::Interval,
    vec3::Vec3,
};

#[derive(Debug)]
pub struct Sphere<M> {
    center: Vec3,
    radius: f64,
    mat: M,
}

impl<M> Sphere<M> {
    pub fn new<C: Into<Vec3>>(mat: M, center: C, radius: f64) -> Self {
        Self {
            center: center.into(),
            radius,
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, rng: &mut SmallRng, ray: &Ray, hit_range: Range<f64>) -> Option<HitRecord> {
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
        let (ray, color) = self.mat.scatter(rng, ray, point, normal);

        Some(HitRecord {
            t: root,
            ray,
            color,
        })
    }
}
