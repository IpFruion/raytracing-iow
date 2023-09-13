use rand::{rngs::SmallRng, Rng};

use crate::{
    color::{Color, WHITE},
    ray::Ray,
    shapes::Hit,
    vec3::Vec3,
};

use super::Scatter;

pub struct Dielectric {
    /// Index of Refraction
    ir: f64,
}

impl Dielectric {
    /// New Dielectric with an index of refraction
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, rng: &mut SmallRng, ray: &Ray, hit: &Hit) -> (Ray, Option<Color>) {
        let refraction_ratio = if hit.is_front_face {
            self.ir.recip()
        } else {
            self.ir
        };
        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refrace = refraction_ratio * sin_theta > 1.0;
        let reflectance = reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>();

        let direction = if reflectance {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_ratio)
        };

        (hit.point.ray(direction), Some(WHITE))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
