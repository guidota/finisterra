use std::{
    cmp::{max, min},
    collections::HashMap,
    time::Duration,
};

use camera::Camera;
use engine::{
    draw::{image::DrawImage, Target},
    engine::{FontID, GameEngine, SoundID, TextureID},
    window::Size,
};
use fonts::Fonts;
use images::Images;
use input::WinitInputHelper;
use renderer::{Instructions, Renderer};
use sounds::Sounds;
use state::State;
use tokio::runtime::Handle;
use wgpu::PresentMode;

mod camera;
mod files;
mod fonts;
mod images;
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
    images: Images,
    pub(crate) renderer: Renderer,
    input: WinitInputHelper,

    depth_texture_view: wgpu::TextureView,
    depth_textures: HashMap<engine::window::Size, wgpu::TextureView>,
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

        let renderer = Renderer::initialize(&state.device, &state.config);
        let world_camera = Camera::initialize(state.size);
        let ui_camera = Camera::initialize(state.size);
        let fonts = Fonts::initialize();
        let _sounds = Sounds::initialize();
        let images = Images::initialize();
        let input = WinitInputHelper::new();

        let size = engine::window::Size {
            width: state.config.width as u16,
            height: state.config.height as u16,
        };
        let depth_texture_view = create_depth_texture(&state, size);

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
            depth_textures: HashMap::default(),
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

    fn add_texture(&mut self, path: &str) -> TextureID {
        self.images.add_file(path)
    }

    fn set_texture(&mut self, path: &str, id: TextureID) {
        self.images.set_file(id, path);
    }

    fn create_texture(&mut self, dimensions: engine::draw::Dimensions) -> TextureID {
        let texture = texture::Texture::new(&self.state.device, dimensions);

        self.images.add_texture(texture)
    }

    fn draw_image(
        &mut self,
        parameters: engine::draw::image::DrawImage,
        target: engine::draw::Target,
    ) {
        if self
            .images
            .load_texture(&self.state.device, &self.state.queue, parameters.index)
        {
            let texture_array = match target {
                Target::World | Target::UI => &mut self.renderer.texture_array,
                _ => &mut self.renderer.pre_render_texture_array,
            };
            if !texture_array.has_texture(parameters.index) {
                let texture = self.images.get(parameters.index).unwrap().unwrap();
                let view = texture.view.clone();
                let sampler = texture.sampler.clone();

                texture_array.push(parameters.index, view, sampler);
            }

            self.renderer.draw_image(parameters, target);
        } else {
            log::error!("[draw_image] with invalid texture");
        }
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
        if self
            .images
            .load_texture(&self.state.device, &self.state.queue, texture_id)
        {
            let texture_array = match target {
                Target::World | Target::UI => &mut self.renderer.texture_array,
                _ => &mut self.renderer.pre_render_texture_array,
            };
            if !texture_array.has_texture(texture_id) {
                let texture = self.images.get(texture_id).unwrap().unwrap();
                let view = texture.view.clone();
                let sampler = texture.sampler.clone();

                texture_array.push(texture_id, view, sampler);
            }
        }

        let offset_x = (parameters.text.total_width as f32 / 2.).round() as u16;

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

            self.renderer.draw_image(
                DrawImage {
                    position,
                    source,
                    color: parameters.color,
                    index: texture_id,
                },
                target,
            );
        }
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
        self.depth_texture_view = create_depth_texture(&self.state, size);
    }

    fn render(&mut self) {
        let Ok(frame) = self.state.surface.get_current_texture() else {
            log::error!("");
            return;
        };

        let Instructions {
            world_range,
            ui_range,
            to_textures_ranges,
        } = self.renderer.prepare(&self.state.device, &self.state.queue);

        {
            let mut commands = vec![];
            // Render to textures
            let mut encoder = self
                .state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            if let Some(bind_group) = self.renderer.pre_render_texture_array.get_bind_group() {
                for (texture_id, range) in to_textures_ranges {
                    if let Some(Some(texture)) = self.images.get(texture_id) {
                        let size = engine::window::Size {
                            width: texture.width,
                            height: texture.height,
                        };
                        let depth_texture_view = self
                            .depth_textures
                            .entry(size)
                            .or_insert_with(|| create_depth_texture(&self.state, size));

                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Render To Texture Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &texture.view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: depth_texture_view,
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(0.0),
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    },
                                ),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                        let dimensions = Size {
                            width: size.width,
                            height: size.height,
                        };
                        let target_camera = Camera::initialize(dimensions);
                        self.renderer.prepare_pass(&mut render_pass, bind_group);
                        self.renderer.render_range(
                            &mut render_pass,
                            range,
                            &target_camera.viewport,
                            target_camera.build_ui_view_projection_matrix(),
                        );
                    }
                }
            }
            commands.push(encoder.finish());

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

            commands.push(encoder.finish());

            let mut encoder =
                self.state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });
            if let Some(bind_group) = self.renderer.texture_array.get_bind_group() {
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

                    self.renderer.prepare_pass(&mut render_pass, bind_group);
                    self.renderer.render_range(
                        &mut render_pass,
                        world_range,
                        &self.world_camera.viewport,
                        self.world_camera.build_view_projection_matrix(true),
                    );
                    self.renderer.render_range(
                        &mut render_pass,
                        ui_range,
                        &self.ui_camera.viewport,
                        self.ui_camera.build_view_projection_matrix(false),
                    );
                }
            }
            commands.push(encoder.finish());

            self.state.queue.submit(commands);
            frame.present();
        }
    }

    fn finish(&self) {
        self.state.window.request_redraw();
    }
}

fn create_depth_texture(state: &State, size: engine::window::Size) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: size.width as u32,
        height: size.height as u32,
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
