//use cgmath::Point3;
use cgmath::{Vector3, Point3};

use crate::raytracing::Point32;

pub struct HitRecord {
    pub p: Point32,
    pub normal: Vector3<f32>,
    pub t: f32,
}