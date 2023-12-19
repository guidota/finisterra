pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub title: String,
    // pub textures_folder: String,
    pub present_mode: wgpu::PresentMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Roma".to_string(),
            present_mode: wgpu::PresentMode::AutoVsync,
            // textures_folder: "assets/textures".to_string(),
        }
    }
}
