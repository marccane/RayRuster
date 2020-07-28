use super::{Color2, Ray2, HitRecord};

pub trait Material {
    fn scatter(&self, r_in: Ray2, rec: HitRecord) -> Option<(Color2, Ray2)>;
}