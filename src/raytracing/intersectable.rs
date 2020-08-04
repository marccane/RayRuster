use super::{Ray2, HitRecord};

pub trait Intersectable: Sync {
    fn intersect(&self, ray: &Ray2, t_min: f32, t_max:f32) -> Option<HitRecord>;
}