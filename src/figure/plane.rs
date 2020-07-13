struct Plane {
    A: f32,
    B: f32,
    C: f32,
    D: f32,
}

impl Intersecable for Plane {
    fn intersect(ray: Ray) -> Color {
        //TODO
        Color::new(0.0,0.0,0.0)
    }
}