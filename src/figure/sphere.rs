struct Sphere {
    center: Point3,
    radius: f64,
}

impl Intersecable for Sphere {
    fn intersect(ray: Ray) -> Color {
        //TODO
        Color::new(0.0,0.0,0.0)
    }
}