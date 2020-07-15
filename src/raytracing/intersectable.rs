use raytracing::{Ray2, Hit};

pub trait Intersectable {
    fn intersect(ray: Ray2) -> Option<Hit>;
}