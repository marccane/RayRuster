use crate::raytracing::{Intersectable, Ray2, HitRecord};

pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    //Creates a plane given its properties.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }

    //Creates a default Plane
    pub fn new_default() -> Plane {
        Plane::new(1.0, 1.0, 1.0, 0.0)
    }

}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord> {
        //TODO
        Option::None
    }
}