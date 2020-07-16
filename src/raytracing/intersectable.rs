use super::{Ray2, Point32};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray2) -> Option<Point32>;
}