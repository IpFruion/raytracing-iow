use rand::rngs::SmallRng;

use crate::{pixel::Pixel, ray::Ray, vec3::Vec3};

pub mod lambertian;
pub mod metal;

pub trait Material {
    /// Scatters the ray in the material
    /// Params is the ray going into the material that has it the object.
    /// Output is the outging ray of the scatter and the color that was at that spot.
    fn scatter(
        &self,
        rng: &mut SmallRng,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
    ) -> (Ray, Option<Pixel>);
}
