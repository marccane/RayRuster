use super::{Ray2, Point32, HitRecord};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray2) -> Option<HitRecord>;
}