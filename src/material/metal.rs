use rand::rngs::SmallRng;

use crate::{color::Color, ray::Ray, shapes::Hit, vec3::Vec3};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut SmallRng, ray: &Ray, hit: &Hit) -> (Ray, Option<Color>) {
        let reflected = ray.direction().normalize().reflect(hit.normal);
        let scattered = hit
            .point
            .ray(reflected + self.fuzziness * Vec3::random_unit_sphere(rng));
        if scattered.direction().dot(hit.normal) < 0. {
            return (scattered, None);
        }
        (scattered, Some(self.albedo.clone()))
    }
}
