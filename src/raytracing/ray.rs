//use cgmath::Vector3;
use super::{Point32, Vec3};
//use crate::raytracing::{Point32, Vec3};

pub struct Ray2 {
    pub origin: Point32,
    pub dir: Vec3,
}

impl Ray2 {
    pub fn new(origin: Point32, dir: Vec3) -> Self {
        Self{origin, dir}
    }

    pub fn at(&self, t: f32) -> Point32 {
        self.origin + t * self.dir
    }
}