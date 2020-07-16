
mod ray;
mod intersectable;
mod point3;
mod color;
mod hit_record;

pub type Vec3 = cgmath::Vector3<f32>;

pub use ray::Ray2;

pub use intersectable::Intersectable;

pub use point3::Point32;

pub use color::Color2;

pub use hit_record::HitRecord;