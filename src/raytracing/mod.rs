mod ray;
mod intersectable;
mod point3;
mod color;
mod material;
mod hit_record;
mod intersectable_list;
mod camera;

pub type Vec3 = cgmath::Vector3<f32>;
pub use ray::Ray2;
pub use intersectable::Intersectable;
pub use point3::Point32;
pub use color::Color2;
pub use material::Material;
pub use hit_record::HitRecord;
pub use intersectable_list::IntersectableList;
pub use camera::Camera;
