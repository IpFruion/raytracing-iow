use crate::vec3::Vec3;

use super::viewport::Viewport;

#[derive(Debug)]
pub struct Screen {
    width: u64,
    height: u64,
}

impl Screen {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }

    pub fn new_aspect_ratio(width: u64, ratio: f64) -> Self {
        Self::new(width, ((width as f64 / ratio) as u64))
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn viewport(&self, height: f64) -> Viewport {
        Viewport {
            width: height * (self.width as f64) / (self.height as f64),
            height,
        }
    }
}
