pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub title: String,
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Game".to_string(),
            vsync: false,
        }
    }
}
