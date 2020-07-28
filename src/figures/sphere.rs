use crate::raytracing::*;
use crate::materials::*;
use cgmath::prelude::InnerSpace;

pub struct Sphere<'a> {
    pub center: Point32,
    pub radius: f32,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {  
    //Creates a Sphere given its center and radius.
    pub fn new(center: Point32, radius: f32, material: &'a dyn Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
    
    //Creates a Sphere of radius 1 centered on the origin (0,0,0)
    /*pub fn new_default() -> Sphere<'a> {
        Sphere::new(Point32::new(0.0,0.0,0.0), 1.0, &Lambertian{albedo: Color2::new(0.5,0.5,0.5)})
    }*/

}

impl<'a> Intersectable for Sphere<'a> {

    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.magnitude2();
        let half_b = oc.dot(ray.dir);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            
            //let mut res: Option<HitRecord> = Option::None;

            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a; //finish solving the equation
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let mut hit_rec = HitRecord::new_with_point_and_t(p, temp, self.material);
                let outward_normal = (hit_rec.p - self.center) / self.radius;
                hit_rec.set_face_normal(ray, outward_normal);
                return Some(hit_rec);
            }
            temp = (-half_b + root)/a;
            if temp > t_max && temp < t_min {
                let p = ray.at(temp);
                let mut hit_rec = HitRecord::new_with_point_and_t(p, temp, self.material);
                let outward_normal = (hit_rec.p - self.center) / self.radius;
                hit_rec.set_face_normal(ray, outward_normal);
                return Some(hit_rec);
            }
        }
           /*res
        } else {
            Option::None
        }*/
        Option::None
    }
}