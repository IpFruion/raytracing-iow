use std::ops::Deref;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    pixel::{Pixel, BLACK, SKY_BLUE, WHITE},
    ray::Ray,
    vec3::{Vec3, ONE},
};

use super::{hit::Hittable, viewport::Viewport, PixelLocator, Renderer, World};

#[derive(Debug)]
pub struct Camera {
    center: Vec3,
    focal_length: f64,
    viewport: Viewport,
    samples_per_pixel: u32,
    max_depth: u32,
    rng: SmallRng,
}

impl Camera {
    pub fn new(
        center: Vec3,
        focal_length: f64,
        viewport: Viewport,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        Self {
            center,
            focal_length,
            viewport,
            samples_per_pixel,
            max_depth,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn upper_left(&self) -> Vec3 {
        let u = self.viewport.u();
        let v = self.viewport.v();
        let view_direction: Vec3 = (0., 0., self.focal_length).into();

        self.center - view_direction - u / 2. - v / 2.
    }

    pub fn get_color(
        &mut self,
        world: &World,
        pixel_locator: &PixelLocator,
        pixel_loc: Vec3,
    ) -> Pixel {
        let mut pixel = BLACK;
        for i in 0..self.samples_per_pixel {
            // Adds antialising
            let px = -0.5 + self.rng.gen::<f64>();
            let py = -0.5 + self.rng.gen::<f64>();
            let pixel_sample = pixel_locator.adjust_pixel_loc(pixel_loc, px, py);

            let ray_direction = pixel_sample - self.center;
            let ray = self.center.ray(ray_direction);

            pixel += self.ray_color(ray, world);
        }

        let pixel = (self.samples_per_pixel as f64).recip() * pixel;
        pixel.clamp(0.0..0.9999)
    }

    fn ray_color(&mut self, ray: Ray, world: &World) -> Pixel {
        let mut stack = vec![(ray, WHITE, 0)];
        let mut output = BLACK;
        while let Some((cur, attenuation, depth)) = stack.pop() {
            if let Some(hit) = world.deref().cast(&mut self.rng, &cur) {
                // let on_unit_sphere = Vec3::random_unit_sphere(&mut self.rng);
                // let direction = hit.normal + on_unit_sphere;
                let new_att = hit.color.map(|c| attenuation * c).unwrap_or(BLACK);
                stack.push((hit.ray, new_att, depth + 1))
            } else {
                // ray stopped bouncing
                output = attenuation * self.render_skybox(&cur);
                break;
            }
            if depth >= self.max_depth {
                output = BLACK;
                break;
            }
        }
        output
    }

    fn render_skybox(&self, ray: &Ray) -> Pixel {
        let direction = ray.direction();
        let gradiant = 0.5 * (direction.y + 1.0);
        Pixel::from((1.0 - gradiant) * WHITE + gradiant * SKY_BLUE)
    }
}
