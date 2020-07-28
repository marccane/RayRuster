use crate::raytracing::*;
use cgmath::prelude::*;

pub struct Metal {
    pub albedo: Color2,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray2, rec: HitRecord) -> Option<(Color2, Ray2)> {
        let reflected = reflect(r_in.dir.normalize(), rec.normal);
        let scattered = Ray2::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        }
        else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color2) -> Self {
        Self{albedo}
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*v.dot(n)*n
}
