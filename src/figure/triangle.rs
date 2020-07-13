struct Triangle {
    point_1: Point3,
    point_2: Point3,
    point_3: Point3,
}

impl Intersecable for Triangle {
    fn intersect(ray: Ray) -> Color {
        //TODO
        Color::new(0.0,0.0,0.0)
    }
}