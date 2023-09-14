#![allow(unused)]
pub mod cube;
pub mod sphere;

use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{color::Color, ray::Ray, vec3::Vec3};

use self::{cube::Cube, sphere::Sphere};

/// Describes a shape that is hittable
pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit>;
}

impl<H: Hittable + ?Sized> Hittable for Box<H> {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        (**self).hit(ray, hit_range)
    }
}

impl<H: Hittable> Hittable for &[H] {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        let mut hit = None;
        let mut cur_range = hit_range;

        for obj in self.iter() {
            if let Some(h) = obj.hit(ray, cur_range.clone()) {
                cur_range.end = h.t;
                hit = Some(h);
            }
        }

        hit
    }
}

pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub is_front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, t: f64, point: Vec3, normal: Vec3) -> Self {
        let is_front_face = ray.direction().dot(normal) < 0.;
        Self {
            t,
            point,
            is_front_face,
            normal: if is_front_face { normal } else { -normal },
        }
    }
}

pub enum Shape {
    Sphere(Sphere),
    Cube(Cube),
    Custom(Box<dyn Hittable + Send + Sync>),
}

macro_rules! from_shape {
    ($name:ident) => {
        impl From<$name> for Shape {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}

from_shape!(Sphere);
from_shape!(Cube);

impl From<Box<dyn Hittable + Send + Sync>> for Shape {
    fn from(value: Box<dyn Hittable + Send + Sync>) -> Self {
        Self::Custom(value)
    }
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, hit_range: Range<f64>) -> Option<Hit> {
        match self {
            Shape::Sphere(h) => h.hit(ray, hit_range),
            Shape::Cube(h) => h.hit(ray, hit_range),
            Shape::Custom(h) => h.hit(ray, hit_range),
        }
    }
}
