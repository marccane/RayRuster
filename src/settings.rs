pub const DEFAULT_RAYTRACING_DEPTH: i8 = 1;
pub const DEFAULT_WIDTH: i16 = 300;
pub const DEFAULT_HEIGHT: i16 = 300;

pub enum DisplayMode {
    WINDOW,
    FILE
}

pub struct Settings {
    pub display_mode: DisplayMode,
    pub ray_depth:  i8,
    pub width:      i16,
    pub height:     i16,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            display_mode: DisplayMode::FILE,
            ray_depth: 1,
            width: 300,
            height: 300,
        }
    }
}