use engine::{
    draw::{image::DrawImage, Target},
    engine::TextureID,
    window::Size,
};

use crate::{camera::Camera, state::State, texture::Texture};

use self::{
    sprite_batch_renderer::SpriteBatchRenderer, texture_array_renderer::TextureArrayRenderer,
};

mod sprite_batch_renderer;
mod texture_array;
mod texture_array_renderer;
mod textures;

pub trait Renderer {
    fn resize(&mut self, state: &State, size: Size);

    fn add_texture_file(&mut self, path: &str) -> TextureID;
    fn set_texture_file(&mut self, path: &str, id: TextureID);

    fn add_texture(&mut self, texture: Texture) -> TextureID;
    fn texture_dimensions(&mut self, texture_id: TextureID) -> Option<(u16, u16)>;

    fn draw_images(&mut self, state: &State, parameters: Vec<DrawImage>, target: Target);

    fn render(&mut self, state: &State, world_camera: &Camera, ui_camera: &Camera);
}

pub fn initialize(state: &State) -> Box<dyn Renderer> {
    if state.limits.max_samplers_per_shader_stage <= 1000 {
        Box::new(SpriteBatchRenderer::initialize(state))
    } else {
        Box::new(TextureArrayRenderer::initialize(state))
    }
}
