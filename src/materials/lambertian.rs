use crate::raytracing::*;
use crate::utils::*;


pub struct Lambertian {
    pub albedo: Color2,
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray2, rec: HitRecord) -> Option<(Color2, Ray2)> {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray2::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Color2) -> Self {
        Self{albedo}
    }
}