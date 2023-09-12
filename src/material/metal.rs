use rand::rngs::SmallRng;

use crate::{pixel::Pixel, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Metal {
    albedo: Pixel,
}

impl Metal {
    pub fn new(albedo: Pixel) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut SmallRng,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
    ) -> (Ray, Option<Pixel>) {
        let reflected = ray.direction().normalize().reflect(normal);
        let scattered = point.ray(reflected);
        (scattered, Some(self.albedo.clone()))
    }
}
