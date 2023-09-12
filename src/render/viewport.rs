use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Viewport {
    pub(super) width: f64,
    pub(super) height: f64,
}

impl Viewport {
    pub fn u(&self) -> Vec3 {
        (self.width, 0., 0.).into()
    }

    pub fn v(&self) -> Vec3 {
        (0., -self.height, 0.).into()
    }
}
