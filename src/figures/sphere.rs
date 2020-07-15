use raytracing::{Intersectable, Ray2, Hit, Point32};

pub struct Sphere {
    center: Point32,
    radius: f32,
}

impl Sphere {  
    //Creates a Sphere of radius 1 centered on the origin (0,0,0)
    fn new() -> Sphere {
        Sphere{
            center: Point32(0.0,0.0,0.0),
            radius: 1,
        }
    }

    //Creates a Sphere given its center and radius.
    fn new(center: Point32, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Intersectable for Sphere {

    fn intersect(&self, ray: Ray2) -> Option<Hit> {
        //TODO
        Option::None
    }
}