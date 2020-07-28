//use cgmath::Point3;
//use cgmath::{Vector3, Point3};

use cgmath::prelude::InnerSpace;
use crate::raytracing::{Point32, Vec3, Ray2, Material};

pub struct HitRecord<'a> {
    pub p: Point32,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new_with_point_and_t(p: Point32, t: f32, material: &'a dyn Material) -> HitRecord {
        HitRecord{ p, normal: Vec3::new(0.0,0.0,0.0), t, front_face: false, material}
    }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray2, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}