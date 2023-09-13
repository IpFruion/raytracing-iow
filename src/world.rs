use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{
    color::Color,
    materials::{Material, Scatter},
    ray::Ray,
    shapes::{Hit, Hittable, Shape},
};

pub type World = Vec<Object>;

pub struct Cast {
    t: f64,
    pub bounce: Ray,
    pub color: Option<Color>,
}

pub trait Castable {
    fn cast(&self, rng: &mut SmallRng, ray: &Ray, cast_range: Range<f64>) -> Option<Cast>;
}

impl<C: Castable> Castable for &[C] {
    fn cast(&self, rng: &mut SmallRng, ray: &Ray, cast_range: Range<f64>) -> Option<Cast> {
        let mut cast = None;
        let mut cur_range = cast_range;

        for obj in self.iter() {
            if let Some(c) = obj.cast(rng, ray, cur_range.clone()) {
                cur_range.end = c.t;
                cast = Some(c);
            }
        }

        cast
    }
}

impl<C: Castable + ?Sized> Castable for Box<C> {
    fn cast(&self, rng: &mut SmallRng, ray: &Ray, cast_range: Range<f64>) -> Option<Cast> {
        (**self).cast(rng, ray, cast_range)
    }
}

pub struct Object {
    pub shape: Shape,
    pub mat: Material,
}

impl Object {
    pub fn new<H: Into<Shape>, M: Into<Material>>(shape: H, mat: M) -> Object {
        Self {
            shape: shape.into(),
            mat: mat.into(),
        }
    }
}

impl Castable for Object {
    fn cast(&self, rng: &mut SmallRng, ray: &Ray, cast_range: Range<f64>) -> Option<Cast> {
        if let Some(hit) = self.shape.hit(ray, cast_range) {
            let (bounce, color) = self.mat.scatter(rng, ray, &hit);
            return Some(Cast {
                t: hit.t,
                bounce,
                color,
            });
        }
        None
    }
}
