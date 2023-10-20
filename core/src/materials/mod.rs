use rand::rngs::SmallRng;

use crate::{color::Color, ray::Ray, shapes::Hit};

use self::{dielectric::Dielectric, lambertian::Lambertain, metal::Metal};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Scatter {
    /// Scatters the ray in the material
    /// Params is the ray going into the material that has it the object.
    /// Output is the outging ray of the scatter and the color that was at that spot.
    fn scatter(&self, rng: &mut SmallRng, ray: &Ray, hit: &Hit) -> (Ray, Option<Color>);
}

pub enum Material {
    Dielectric(Dielectric),
    Lambertain(Lambertain),
    Metal(Metal),
    Custom(Box<dyn Scatter + Send + Sync>),
}

macro_rules! mat_from {
    ($name:ident) => {
        impl From<$name> for Material {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}
mat_from!(Dielectric);
mat_from!(Lambertain);
mat_from!(Metal);

impl From<Box<dyn Scatter + Send + Sync>> for Material {
    fn from(value: Box<dyn Scatter + Send + Sync>) -> Self {
        Self::Custom(value)
    }
}

impl Scatter for Material {
    fn scatter(&self, rng: &mut SmallRng, ray: &Ray, hit: &Hit) -> (Ray, Option<Color>) {
        match self {
            Material::Dielectric(m) => m.scatter(rng, ray, hit),
            Material::Lambertain(m) => m.scatter(rng, ray, hit),
            Material::Metal(m) => m.scatter(rng, ray, hit),
            Material::Custom(m) => m.scatter(rng, ray, hit),
        }
    }
}
