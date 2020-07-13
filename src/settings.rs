pub struct Settings {
    pub ray_depth: i8,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            ray_depth: 1,
        }
    }
}