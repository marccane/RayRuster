use raytracing::{Intersectable, Ray2, Hit};

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    //Creates a default Plane
    fn new() {
        Plane {
            a: 1,
            b: 1,
            c: 1,
            d: 0,
        }
    }

    //Creates a plane given its properties.
    fn new(a: f32, b: f32, c: f32, d: f32) {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray2) -> Option<Hit> {
        //TODO
        Option::None
    }
}