use crate::raytracing::{Intersectable, Ray2, Point32, HitRecord};
use cgmath::prelude::InnerSpace;

pub struct Sphere {
    center: Point32,
    radius: f32,
}

impl Sphere {  
    //Creates a Sphere given its center and radius.
    fn new(center: Point32, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
    
    //Creates a Sphere of radius 1 centered on the origin (0,0,0)
    fn new_default() -> Sphere {
        Sphere::new(Point32::new(0.0,0.0,0.0), 1.0)
    }

}

impl Intersectable for Sphere {

    fn intersect(&self, ray: &Ray2) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.magnitude2();
        let half_b = oc.dot(ray.dir);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            //-1.0
        } else {
            //(-half_b - discriminant.sqrt()) / a
        }

        Option::None
    }
}