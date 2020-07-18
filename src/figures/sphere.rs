use crate::raytracing::{Intersectable, Vec3, Ray2, Point32, HitRecord};
use cgmath::prelude::InnerSpace;

pub struct Sphere {
    center: Point32,
    radius: f32,
}

impl Sphere {  
    //Creates a Sphere given its center and radius.
    pub fn new(center: Point32, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
    
    //Creates a Sphere of radius 1 centered on the origin (0,0,0)
    pub fn new_default() -> Sphere {
        Sphere::new(Point32::new(0.0,0.0,0.0), 1.0)
    }

}

impl Intersectable for Sphere {

    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.magnitude2();
        let half_b = oc.dot(ray.dir);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            
            let mut res: Option<HitRecord> = Option::None;

            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a; //finish solving the equation
            if temp < t_min && temp > t_max {
                let mut hit_rec = HitRecord::new_with_point(ray.at(temp));
                let outward_normal = (hit_rec.p - self.center) / self.radius;
                hit_rec.set_face_normal(ray, outward_normal);
                res = Some(hit_rec);
            }
            temp = (-half_b + root)/a;
            if temp > t_min && temp < t_max {
                let mut hit_rec = HitRecord::new_with_point(ray.at(temp));
                let outward_normal = (hit_rec.p - self.center) / self.radius;
                hit_rec.set_face_normal(ray, outward_normal);
                res = Some(hit_rec);
            }
            res
        } else {
            Option::None
        }
    }
}