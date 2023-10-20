use raytracing_iow::{
    materials::{dielectric::Dielectric, lambertian::Lambertain, metal::Metal},
    vec3::Vec3,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct Object {
    pub shape: Shape,
    pub material: Material,
}

impl Into<raytracing_iow::world::Object> for Object {
    fn into(self) -> raytracing_iow::world::Object {
        raytracing_iow::world::Object::new(self.shape, self.material)
    }
}

#[derive(Deserialize, ToSchema)]
pub enum Shape {
    Sphere(Sphere),
}

impl Into<raytracing_iow::shapes::Shape> for Shape {
    fn into(self) -> raytracing_iow::shapes::Shape {
        match self {
            Shape::Sphere(s) => raytracing_iow::shapes::Shape::Sphere(s.into()),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub enum Sphere {
    Stationary { center: Vec3, radius: f64 },
}

impl Into<raytracing_iow::shapes::sphere::Sphere> for Sphere {
    fn into(self) -> raytracing_iow::shapes::sphere::Sphere {
        match self {
            Sphere::Stationary { center, radius } => {
                raytracing_iow::shapes::sphere::Sphere::new(center, radius)
            }
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub enum Material {
    Lambertain { color: Color },
    Metal { color: Color, fuzziness: f64 },
    Dielectric { index_of_refraction: f64 },
}

impl Into<raytracing_iow::materials::Material> for Material {
    fn into(self) -> raytracing_iow::materials::Material {
        match self {
            Material::Lambertain { color } => raytracing_iow::materials::Material::Lambertain(
                Lambertain::new((color.r, color.g, color.b).into()),
            ),
            Material::Metal { color, fuzziness } => raytracing_iow::materials::Material::Metal(
                Metal::new((color.r, color.g, color.b).into(), fuzziness),
            ),
            Material::Dielectric {
                index_of_refraction,
            } => raytracing_iow::materials::Material::Dielectric(Dielectric::new(
                index_of_refraction,
            )),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
