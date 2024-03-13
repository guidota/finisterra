use std::{
    cmp::{max, min},
    time::Duration,
};

use camera::Camera;
use engine::{
    draw::{image::DrawImage, Target},
    engine::{FontID, GameEngine, SoundID, TextureID},
};
use fonts::Fonts;
use input::WinitInputHelper;
use renderer::Renderer;
use sounds::Sounds;
use state::State;
use tokio::runtime::Handle;
use wgpu::PresentMode;

mod camera;
mod files;
mod fonts;
mod input;
mod renderer;
mod sounds;
mod state;
mod texture;

pub struct Roma {
    state: State,
    delta: std::time::Duration,

    world_camera: Camera,
    ui_camera: Camera,
    fonts: Fonts,
    _sounds: Sounds,
    renderer: Box<dyn Renderer>,
    input: WinitInputHelper,
}

impl GameEngine for Roma {
    fn initialize(window: winit::window::Window, settings: &engine::settings::Settings) -> Self {
        let present_mode = if settings.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
        let state = tokio::task::block_in_place(|| {
            Handle::current().block_on(State::initialize(window, present_mode))
        });

        let renderer = renderer::initialize(&state);
        let world_camera = Camera::initialize(state.size, true);
        let ui_camera = Camera::initialize(state.size, false);
        let fonts = Fonts::initialize();
        let _sounds = Sounds::initialize();
        let input = WinitInputHelper::new();

        Self {
            state,
            delta: Duration::from_millis(0),

            world_camera,
            ui_camera,
            fonts,
            _sounds,
            renderer,
            input,
        }
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>) {
        self.input.update(event);
    }

    fn key_pressed(&self, key: engine::input::keyboard::KeyCode) -> bool {
        self.input.key_pressed(key)
    }

    fn key_released(&self, key: engine::input::keyboard::KeyCode) -> bool {
        self.input.key_released(key)
    }

    fn key_held(&self, key: engine::input::keyboard::KeyCode) -> bool {
        self.input.key_held(key)
    }

    fn held_keys(&self) -> Vec<engine::input::keyboard::Key> {
        self.input.held_keys()
    }

    fn pressed_keys(&self) -> Vec<engine::input::keyboard::Key> {
        self.input.pressed_keys()
    }

    fn released_keys(&self) -> Vec<engine::input::keyboard::Key> {
        self.input.released_keys()
    }

    fn mouse_position(&self) -> engine::input::mouse::Position {
        if let Some((x, y)) = self.input.cursor() {
            let height = self.get_window_size().height as f32;
            engine::input::mouse::Position { x, y: height - y }
        } else {
            engine::input::mouse::Position { x: 0., y: 0. }
        }
    }

    fn mouse_clicked(&self) -> bool {
        self.input.mouse_pressed(winit::event::MouseButton::Left)
    }

    fn mouse_held(&self) -> bool {
        self.input.mouse_held(winit::event::MouseButton::Left)
    }

    fn mouse_released(&self) -> bool {
        self.input.mouse_released(winit::event::MouseButton::Left)
    }

    fn mouse_secondary_clicked(&self) -> bool {
        self.input.mouse_pressed(winit::event::MouseButton::Right)
    }

    fn set_mouse_cursor(&mut self, cursor: winit::window::CursorIcon) {
        self.state.window.set_cursor_icon(cursor)
    }

    fn add_texture(&mut self, path: &str) -> TextureID {
        self.renderer.add_texture_file(path)
    }

    fn set_texture(&mut self, path: &str, id: TextureID) {
        self.renderer.set_texture_file(path, id);
    }

    fn create_texture(&mut self, dimensions: engine::draw::Dimensions) -> TextureID {
        let texture = texture::Texture::new(&self.state.device, dimensions);
        self.renderer.add_texture(texture)
    }

    fn texture_dimensions(&mut self, texture_id: TextureID) -> Option<(u16, u16)> {
        self.renderer.texture_dimensions(texture_id)
    }

    fn draw_image(
        &mut self,
        parameters: engine::draw::image::DrawImage,
        target: engine::draw::Target,
    ) {
        self.renderer
            .draw_images(&self.state, vec![parameters], target);
    }

    fn add_font(&mut self, id: FontID, path: &str, texture_id: TextureID) {
        self.fonts.add_font(id, texture_id, path);
    }

    fn parse_text(&mut self, id: FontID, text: &str) -> Option<engine::draw::text::ParsedText> {
        self.fonts.parse_text(id, text)
    }

    fn draw_text(&mut self, id: FontID, parameters: engine::draw::text::DrawText, target: Target) {
        let Some(texture_id) = self.fonts.get_texture_id(id) else {
            log::error!("[draw_text] texture id for font {id} not found");
            return;
        };

        let offset_x = (parameters.text.total_width as f32 / 2.).round() as u16 - 1;

        let mut draws = vec![];
        for char in &parameters.text.chars {
            let mut position = parameters.position;

            let x = char.screen_rect.x;
            let y = char.screen_rect.y;
            let source = [
                char.page_rect.x as u16,
                char.page_rect.y as u16,
                char.screen_rect.width as u16,
                char.screen_rect.height as u16,
            ];

            position.x += x as u16;
            position.x -= offset_x;
            position.y += y as u16;

            draws.push(DrawImage {
                position,
                source,
                color: parameters.color,
                index: texture_id,
            });
        }
        self.renderer.draw_images(&self.state, draws, target);
    }

    fn add_sound(&mut self, _path: &str) -> SoundID {
        todo!()
    }

    fn set_sound(&mut self, _path: &str, _id: SoundID) {
        todo!()
    }

    fn play_sound(&mut self, _id: SoundID, _parameters: engine::sound::PlaySound) {
        todo!()
    }

    fn play_music(&mut self, _id: SoundID, _parameters: engine::sound::PlayMusic) {
        todo!()
    }

    fn stop_music(&mut self) {
        todo!()
    }

    fn get_world_camera_viewport(&self) -> engine::camera::Viewport {
        self.world_camera.viewport
    }

    fn set_world_camera_viewport(&mut self, viewport: engine::camera::Viewport) {
        let window_size = self.get_window_size();
        self.world_camera.viewport.width = max(
            1,
            min(viewport.width as u16, window_size.width - viewport.x as u16),
        ) as f32;
        self.world_camera.viewport.height = max(
            1,
            min(
                viewport.height as u16,
                window_size.height - viewport.y as u16,
            ),
        ) as f32;
        self.world_camera.viewport.x = max(0, viewport.x as u32) as f32;
        self.world_camera.viewport.y = max(0, viewport.y as u32) as f32;
    }

    fn get_camera_zoom(&self) -> engine::camera::Zoom {
        self.world_camera.zoom
    }

    fn set_camera_zoom(&mut self, zoom: engine::camera::Zoom) {
        self.world_camera.zoom = zoom;
        self.ui_camera.zoom = zoom;
    }

    fn get_world_camera_position(&self) -> engine::camera::Position {
        self.world_camera.position
    }

    fn set_world_camera_position(&mut self, position: engine::camera::Position) {
        self.world_camera.position = position;
    }

    fn get_ui_camera_viewport(&self) -> engine::camera::Viewport {
        self.ui_camera.viewport
    }

    fn set_ui_camera_viewport(&mut self, viewport: engine::camera::Viewport) {
        self.ui_camera.viewport = viewport;
    }

    fn set_delta(&mut self, delta: std::time::Duration) {
        self.delta = delta;
    }

    fn get_delta(&self) -> std::time::Duration {
        self.delta
    }

    fn get_window_size(&self) -> engine::window::Size {
        self.state.size
    }

    fn set_window_size(&mut self, size: engine::window::Size) {
        self.state.resize(size);
        self.renderer.resize(&self.state, size);
    }

    fn render(&mut self) {
        self.renderer
            .render(&self.state, &self.world_camera, &self.ui_camera);
    }

    fn finish(&self) {
        self.state.window.request_redraw();
    }
}
