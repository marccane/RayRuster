use crate::raytracing::Vec3;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use cgmath::prelude::InnerSpace;

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

//Vec3 Utils

//pub fn random_vector(rng: rand::rngs::thread::ThreadRngs) -> Vec3 {
///Random Vec3 inside the 1x1x1 box
#[inline]
pub fn random_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>())
}

#[inline]
pub fn random_double(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let distr = Uniform::from(min..max);
    distr.sample(&mut rng)
}

pub fn random_vector_range(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let distr = Uniform::from(min..max);
    Vec3::new(distr.sample(&mut rng),distr.sample(&mut rng),distr.sample(&mut rng))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop{
        let res = random_vector_range(-1.0,1.0);
        if res.magnitude2() < 1.0 {
            return res;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double(0.0, 2.0*PI);
    let z = random_double(-1.0, 1.0);
    let r = (1.0-z*z).sqrt();
    Vec3::new(r*a.cos(), r*a.sin(), z)
}