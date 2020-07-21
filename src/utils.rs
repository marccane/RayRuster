pub const INFINITY: f32 = f32::MAX;
pub const PI: f32 = 3.1415926535897932384525433832;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {min} else if x > max {max} else {x}
}