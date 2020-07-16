use crate::raytracing::{Intersectable, Ray2, Point32, HitRecord};

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    //Creates a plane given its properties.
    fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }

    //Creates a default Plane
    fn new_default() -> Plane {
        Plane::new(1.0, 1.0, 1.0, 0.0)
    }

}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray2) -> Option<HitRecord> {
        //TODO
        Option::None
    }
}