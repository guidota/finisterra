use derive_builder::Builder;

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct Settings {
    pub window: WindowSettings,
    pub renderer: RendererSettings,
}

#[derive(Debug, Builder, Default, Clone)]
#[builder(setter(into))]
pub struct WindowSettings {
    pub window_width: usize,
    pub window_height: usize,
    pub window_title: String,
}

#[derive(Debug, Builder, Default, Clone)]
#[builder(setter(into))]
pub struct RendererSettings {
    pub present_mode: wgpu::PresentMode,
    pub base_path: String,
}
