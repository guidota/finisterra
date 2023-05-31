#[derive(Default, Debug)]
pub struct Settings {
    pub cache_static_layers: bool,
    pub cache_entities: bool,
    pub use_atlases: bool,
    pub preload_textures: bool,
    pub draw_layer_0: bool,
    pub draw_layer_1: bool,
    pub draw_layer_2: bool,
    pub draw_layer_3: bool,
    pub draw_ui: bool,
    pub draw_names: bool,
}
