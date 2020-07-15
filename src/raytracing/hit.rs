use raytracing::{Intersectable, Ray2, Point32};

pub struct Hit {
    pub incident_ray: Ray2,
    pub hit_point: Point32,
    pub hitted_figure: Intersectable,
}