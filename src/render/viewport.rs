use crate::vec3::Vec3;

use super::{camera::CameraConfig, screen::Screen};

#[derive(Debug)]
pub enum ViewportConfig {
    Standard { height: f64 },
    Fov { vertical_fov: f64 },
}

impl ViewportConfig {
    pub fn get_dims(&self, screen: &Screen, focus_dist: f64) -> (f64, f64) {
        match self {
            Self::Standard { height } => (
                *height,
                height * (screen.width() as f64) / (screen.height() as f64),
            ),
            Self::Fov { vertical_fov } => {
                let h = (vertical_fov.to_radians() / 2.).tan();
                let height = 2. * h * focus_dist;

                (
                    height,
                    height * (screen.width() as f64) / (screen.height() as f64),
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Viewport {
    pub u: Vec3,
    pub v: Vec3,
    pub upper_left: Vec3,
}
