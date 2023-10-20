pub mod camera;
pub mod screen;
pub mod viewport;

use crate::{vec3::Vec3, world::World};

use self::{
    camera::{Camera, Defocus},
    screen::Screen,
};

pub struct PixelLocator {
    delta_u: Vec3,
    delta_v: Vec3,
    upper_left_loc: Vec3,
    defocus: Defocus,
}

impl PixelLocator {
    pub fn from_screen_and_camera(screen: &Screen, camera: &Camera) -> Self {
        let (viewport, defocus) = camera.viewport(screen);
        let delta_u = viewport.u / (screen.width() as f64);
        let delta_v = viewport.v / (screen.height() as f64);
        Self {
            delta_u,
            delta_v,
            upper_left_loc: viewport.upper_left + 0.5 * (delta_u + delta_v),
            defocus,
        }
    }

    pub fn adjust_pixel_loc(&self, pixel_loc: Vec3, dx: f64, dy: f64) -> Vec3 {
        pixel_loc + (dx * self.delta_u) + (dy * self.delta_v)
    }

    pub fn defocus_pixel(&self, pixel_loc: Vec3, dx: f64, dy: f64) -> Vec3 {
        pixel_loc + (dx * self.defocus.disk_u) + (dy * self.defocus.disk_v)
    }

    pub fn pixel_center(&self, x: u64, y: u64) -> Vec3 {
        self.upper_left_loc + ((x as f64) * self.delta_u) + ((y as f64) * self.delta_v)
    }
}
