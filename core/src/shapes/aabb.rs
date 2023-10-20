use std::ops::Range;

/// Axis Aligned Bounding Box
pub struct AABB {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

