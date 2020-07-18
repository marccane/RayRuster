//use cgmath::Point3;
//use cgmath::{Vector3, Point3};

use cgmath::prelude::InnerSpace;
use crate::raytracing::{Point32, Vec3, Ray2};

pub struct HitRecord {
    pub p: Point32,
    pub normal: Vec3,
    //pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_with_point(p: Point32) -> HitRecord {
        HitRecord{ p, normal: Vec3::new(0.0,0.0,0.0), front_face: false }
    }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray2, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}