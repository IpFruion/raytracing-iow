use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{
    color::Color,
    material::Material,
    ray::Ray,
    shapes::{Hit, Hittable},
};

pub type World = Vec<Box<dyn Castable>>;

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

pub struct Object<H, M> {
    pub shape: H,
    pub mat: M,
}

impl<H: Hittable + 'static, M: Material + 'static> Object<H, M> {
    pub fn new_boxed(shape: H, mat: M) -> Box<dyn Castable> {
        Box::new(Self { shape, mat })
    }
}

impl<H: Hittable, M: Material> Castable for Object<H, M> {
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
