use super::{Point32, Vec3};

pub struct Camera {
	origin: Point32,
	lower_left_corner: Point32,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new() -> Camera {
		let aspect_ratio = 16.0 / 9.0;
		let viewport_height = 2.0;
		let viewport_width = aspect_ratio * viewport_height;
		let focal_length = 1.0;
		
		let origin = Point32::new(0.0, 0.0, 0.0);
		let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
		let vertical = Vec3::new(0.0, viewport_height, 0.0);
		
		Camera{
			origin,
			horizontal,
			vertical,
			lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length),
		}
	}
}