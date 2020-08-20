use crate::raytracing::*;
use cgmath::prelude::*;

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray2, rec: HitRecord) -> Option<(Color2, Ray2)> {
        let attenuation = Color2::new(1.0,1.0,1.0);
		let etai_over_etat = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
		
		let unit_direction = r_in.dir.normalize();
		let refracted = refract(unit_direction, &rec.normal, etai_over_etat);
		
        let scattered = Ray2::new(rec.p, refracted);
        Some((attenuation, scattered))
    }
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self{ref_idx}
    }
}

pub fn refract(uv: Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (uv*-1.0).dot(*n);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude2()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
