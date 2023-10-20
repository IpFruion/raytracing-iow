use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::{Color, BLACK, SKY_BLUE, WHITE},
    ray::Ray,
    vec3::Vec3,
};

use super::{
    screen::Screen,
    viewport::{Viewport, ViewportConfig},
    PixelLocator, World,
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CameraConfig {
    /// Camera position
    pub pos: Vec3,
    /// Look at position
    pub look_at: Vec3,
    /// Camera Up direction
    pub up: Vec3,
    /// Number of samples per pixel antialising
    pub samples_per_pixel: u32,
    /// Max ray bounce depth
    pub max_depth: u32,
    // Variation angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from camera pos point to plane of perfect focus
    pub focus_dist: f64,
}

pub struct Defocus {
    pub disk_u: Vec3,
    pub disk_v: Vec3,
}

#[derive(Debug)]
pub struct Camera {
    config: CameraConfig,
    viewport_config: ViewportConfig,
    focal_length: f64,
}

impl Camera {
    pub fn new(config: CameraConfig, viewport_config: ViewportConfig) -> Self {
        let focal_length = (config.pos - config.look_at).length();
        Self {
            config,
            viewport_config,
            focal_length,
        }
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn config(&self) -> &CameraConfig {
        &self.config
    }

    pub fn viewport(&self, screen: &Screen) -> (Viewport, Defocus) {
        let (height, width) = self
            .viewport_config
            .get_dims(screen, self.config.focus_dist);
        let w = (self.config.pos - self.config.look_at).normalize();
        let u = self.config.up.cross(w).normalize();
        let v = w.cross(u);

        let defocus_radius =
            self.config.focus_dist * (self.config.defocus_angle / 2.).to_radians().tan();
        let defocus = Defocus {
            disk_u: u * defocus_radius,
            disk_v: v * defocus_radius,
        };

        let u = width * u;
        let v = height * -v;

        let viewport = Viewport {
            u,
            v,
            upper_left: self.config.pos - (self.config.focus_dist * w) - u / 2. - v / 2.,
        };

        (viewport, defocus)
    }

    pub fn get_color(&self, world: &World, pixel_locator: &PixelLocator, pixel_loc: Vec3) -> Color {
        let defocus_angle = self.config.defocus_angle;
        let pos = self.config.pos;
        let max_depth = self.config.max_depth;
        let pixel = BLACK
            + (0..self.config.samples_per_pixel)
                .into_par_iter()
                .map_init(
                    || SmallRng::from_entropy(),
                    |rng, _| {
                        // Adds antialising
                        let px = -0.5 + rng.gen::<f64>();
                        let py = -0.5 + rng.gen::<f64>();
                        let pixel_sample = pixel_locator.adjust_pixel_loc(pixel_loc, px, py);
                        let ray_origin = if defocus_angle <= 0. {
                            pos
                        } else {
                            let p = Vec3::random_in_unit_disk(rng);
                            pixel_locator.defocus_pixel(pos, p.x, p.y)
                        };
                        let ray_direction = pixel_sample - ray_origin;
                        let ray_time = rng.gen::<f64>();

                        let ray = ray_origin.ray_timed(ray_direction, ray_time);
                        Self::ray_color(rng, ray, world, max_depth)
                    },
                )
                .sum();

        let pixel = (self.config.samples_per_pixel as f64).recip() * pixel;
        pixel.clamp(0.0..0.9999)
    }

    fn ray_color(rng: &mut SmallRng, ray: Ray, world: &World, max_depth: u32) -> Color {
        let mut stack = vec![(ray, WHITE, 0)];
        let mut output = BLACK;
        while let Some((cur, attenuation, depth)) = stack.pop() {
            if let Some(cast) = world.cast(rng, &cur, 0.001..f64::INFINITY) {
                let new_att = cast.color.map(|c| attenuation * c).unwrap_or(BLACK);
                stack.push((cast.bounce, new_att, depth + 1))
            } else {
                // ray stopped bouncing
                output = attenuation * Self::render_skybox(&cur);
                break;
            }
            if depth >= max_depth {
                output = BLACK;
                break;
            }
        }
        output
    }

    fn render_skybox(ray: &Ray) -> Color {
        let direction = ray.direction();
        let gradiant = 0.5 * (direction.y + 1.0);
        Color::from((1.0 - gradiant) * WHITE + gradiant * SKY_BLUE)
    }
}
