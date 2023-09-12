use std::ops::Range;

use rand::rngs::SmallRng;

use crate::{pixel::Pixel, ray::Ray, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, rng: &mut SmallRng, ray: &Ray, hit_range: Range<f64>) -> Option<HitRecord>;

    fn cast(&self, rng: &mut SmallRng, ray: &Ray) -> Option<HitRecord> {
        self.hit(rng, ray, 0.001..f64::INFINITY)
    }

    fn boxed(self) -> Box<dyn Hittable>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<H: Hittable + ?Sized> Hittable for Box<H> {
    fn hit(&self, rng: &mut SmallRng, ray: &Ray, hit_range: Range<f64>) -> Option<HitRecord> {
        (**self).hit(rng, ray, hit_range)
    }
}

impl<H: Hittable> Hittable for &[H] {
    fn hit(&self, rng: &mut SmallRng, ray: &Ray, hit_range: Range<f64>) -> Option<HitRecord> {
        let mut hit = None;
        let mut cur_range = hit_range;

        for obj in self.iter() {
            if let Some(h) = obj.hit(rng, ray, cur_range.clone()) {
                cur_range.end = h.t;
                hit = Some(h);
            }
        }

        hit
    }
}

pub struct HitRecord {
    pub t: f64,
    pub color: Option<Pixel>,
    pub ray: Ray,
}
