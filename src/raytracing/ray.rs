//use cgmath::Vector3;
use super::{Point32, Vec3};
//use crate::raytracing::{Point32, Vec3};

//pub type Ray2 = Vector3<f32>; //emosido
pub struct Ray2 {
    pub origin: Point32,
    pub dir: Vec3,
}

impl Ray2 {
    pub fn at(&self, t: f32) -> Point32 {
        self.origin + t * self.dir
    }
}