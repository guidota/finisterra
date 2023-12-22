use std::time::Duration;

use camera::Camera;
use engine::{draw::image::DrawImage, engine::GameEngine};
use fonts::Fonts;
use images::Images;
use pollster::FutureExt as _;
use renderer::Renderer;
use sounds::Sounds;
use state::State;
use wgpu::PresentMode;
use winit_input_helper::WinitInputHelper;

mod camera;
mod files;
mod fonts;
mod images;
mod renderer;
mod sounds;
mod state;
mod texture;

pub struct Veril {
    state: State,
    delta: std::time::Duration,

    world_camera: Camera,
    ui_camera: Camera,
    fonts: Fonts,
    _sounds: Sounds,
    images: Images,
    renderer: Renderer,
    input: WinitInputHelper,

    depth_texture_view: wgpu::TextureView,
}

impl GameEngine for Veril {
    fn initialize(window: winit::window::Window, settings: &engine::settings::Settings) -> Self {
        let present_mode = if settings.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
        let state = State::initialize(window, present_mode).block_on();
        let renderer = Renderer::initialize(&state.device, &state.config);
        let world_camera = Camera::initialize(state.size);
        let ui_camera = Camera::initialize(state.size);
        let fonts = Fonts::initialize();
        let _sounds = Sounds::initialize();
        let images = Images::initialize();
        let input = WinitInputHelper::new();

        let depth_texture_view = create_depth_texture(&state);

        Self {
            state,
            delta: Duration::from_millis(0),

            world_camera,
            ui_camera,
            fonts,
            _sounds,
            images,
            renderer,
            input,

            depth_texture_view,
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

    fn get_mouse_position(&self) -> engine::input::mouse::Position {
        todo!()
    }

    fn mouse_clicked(&self) -> bool {
        self.input.mouse_pressed(0)
    }

    fn mouse_secondary_clicked(&self) -> bool {
        self.input.mouse_pressed(1)
    }

    fn add_texture(&mut self, path: &str) -> u64 {
        self.images.add_texture(path)
    }

    fn set_texture(&mut self, path: &str, id: u64) {
        self.images.set_texture(id, path);
    }

    fn draw_image(&mut self, id: u64, parameters: engine::draw::image::DrawImage) {
        self.renderer.draw_image(id, parameters);
    }

    fn add_font(&mut self, id: u64, path: &str, texture_id: u64) {
        self.fonts.add_font(id, texture_id, path);
    }

    fn draw_text(&mut self, id: u64, parameters: engine::draw::text::DrawText) {
        let Some(texture_id) = self.fonts.get_texture_id(id) else {
            log::error!("[draw_text] texture id for font {id} not found");
            return;
        };
        let parsed_text = {
            if let Some(parsed_text) = self.fonts.get_text(id, parameters.text) {
                Some(parsed_text)
            } else {
                self.fonts.parse_text(id, parameters.text)
            }
        };

        let Some((chars, total_width)) = parsed_text else {
            log::error!("[draw_text] trying to draw text and failed");
            return;
        };

        let offset_x = (*total_width as f32 / 2.).round() as u16;

        for char in chars {
            let mut position = parameters.position.clone();

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

            self.renderer.draw_image(
                texture_id,
                DrawImage {
                    position,
                    source,
                    color: parameters.color,
                },
            );
        }
    }

    fn add_sound(&mut self, _path: &str) -> u64 {
        todo!()
    }

    fn set_sound(&mut self, _path: &str, _id: u64) {
        todo!()
    }

    fn play_sound(&mut self, _id: u64, _parameters: engine::sound::PlaySound) {
        todo!()
    }

    fn play_music(&mut self, _id: u64, _parameters: engine::sound::PlayMusic) {
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
        self.world_camera.viewport.width =
            std::cmp::min(viewport.width as u32, window_size.width - viewport.x as u32) as f32;
        self.world_camera.viewport.height = std::cmp::min(
            viewport.height as u32,
            window_size.height - viewport.y as u32,
        ) as f32;
        self.world_camera.viewport.x = viewport.x;
        self.world_camera.viewport.y = viewport.y;
    }

    fn get_world_camera_zoom(&self) -> engine::camera::Zoom {
        self.world_camera.zoom
    }

    fn set_world_camera_zoom(&mut self, zoom: engine::camera::Zoom) {
        self.world_camera.zoom = zoom;
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
        self.depth_texture_view = create_depth_texture(&self.state);
    }

    fn render(&mut self) {
        let Ok(frame) = self.state.surface.get_current_texture() else {
            log::error!("");
            return;
        };

        let batches =
            self.renderer
                .prepare(&self.state.device, &self.state.queue, &mut self.images);

        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Clear render pass
        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        let clear = encoder.finish();

        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.render_batches(
                &mut render_pass,
                batches,
                (&self.world_camera, &self.ui_camera),
                self.images.textures(),
            );
        }
        let game_commands = encoder.finish();

        self.state.queue.submit([clear, game_commands]);
        frame.present();
    }

    fn finish(&self) {
        self.state.window.request_redraw();
    }
}

fn create_depth_texture(state: &State) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: state.config.width,
        height: state.config.height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24PlusStencil8,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[wgpu::TextureFormat::Depth24PlusStencil8],
    };
    let texture = state.device.create_texture(&desc);

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
