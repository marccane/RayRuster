//use cgmath::Point3;
use cgmath::{Vector3/*, Point3*/};

use crate::raytracing::Point32;

pub struct HitRecord {
    p: Point32,
    normal: Vector3<f32>,
    t: f32,
}