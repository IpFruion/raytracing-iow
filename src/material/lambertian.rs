use approx::ulps_eq;
use rand::{rngs::SmallRng, SeedableRng};

use crate::{
    pixel::Pixel,
    ray::Ray,
    render::hit::HitRecord,
    vec3::{Vec3, ZERO},
};

use super::Material;

pub struct Lambertain {
    albedo: Pixel,
}

impl Lambertain {
    pub fn new(albedo: Pixel) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertain {
    fn scatter(
        &self,
        rng: &mut SmallRng,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
    ) -> (Ray, Option<Pixel>) {
        let mut direction = normal + Vec3::random_unit_sphere(rng);
        if ulps_eq!(direction, ZERO) {
            direction = normal
        }
        let scattered = point.ray(direction);
        (scattered, Some(self.albedo.clone()))
    }
}
