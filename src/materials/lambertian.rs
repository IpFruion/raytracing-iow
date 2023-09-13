use approx::ulps_eq;
use rand::{rngs::SmallRng, SeedableRng};

use crate::{
    color::Color,
    ray::Ray,
    shapes::Hit,
    vec3::{Vec3, ZERO},
};

use super::Scatter;

pub struct Lambertain {
    albedo: Color,
}

impl Lambertain {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertain {
    fn scatter(&self, rng: &mut SmallRng, ray: &Ray, hit: &Hit) -> (Ray, Option<Color>) {
        let mut direction = hit.normal + Vec3::random_unit_sphere(rng);
        if ulps_eq!(direction, ZERO) {
            direction = hit.normal
        }
        let scattered = hit.point.ray(direction);
        (scattered, Some(self.albedo.clone()))
    }
}
