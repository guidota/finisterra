use std::iter::once;

use crate::{
    resources::{camera::Camera2D, texture},
    settings::RendererSettings,
};

use self::{
    camera::Camera,
    render_pass::{create_color_attachment, create_depth_attachment, RenderPass},
    state::State,
    textures::Textures,
    vertex::Vertex,
};

mod camera;
mod render_pass;
mod state;
mod textures;
pub(crate) mod vertex;

pub struct Renderer {
    state: State,
    camera: Camera,
    textures: Textures,
    render_pass: RenderPass,
}

pub(crate) struct Instructions {
    // replace with bytes or something else
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) batches: Vec<Batch>,
}

pub(crate) struct Batch {
    pub(crate) texture_id: usize,
    pub(crate) size: u32,
}

impl Renderer {
    pub(crate) async fn new(settings: &RendererSettings, window: winit::window::Window) -> Self {
        let state = State::init(window, settings).await;
        let camera = Camera::init(&state.device);
        let textures = Textures::init(&state.device, &settings.base_path);
        let render_pass = RenderPass::init(
            &state.device,
            &state.config,
            &textures.bind_group_layout,
            &camera.bind_group_layout,
        );

        Self {
            state,
            camera,
            textures,
            render_pass,
        }
    }

    pub(crate) fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.state.resize(*new_size);
            self.render_pass
                .resize(&self.state.device, &self.state.config);
        }
    }

    pub(crate) fn window(&self) -> &winit::window::Window {
        &self.state.window
    }

    pub(crate) fn size(&self) -> &winit::dpi::PhysicalSize<u32> {
        &self.state.size
    }

    pub(crate) fn update_camera(&mut self, camera: &Camera2D) {
        let projection = camera.build_view_projection_matrix();
        self.camera.update_projection(&self.state.queue, projection);
    }

    pub(crate) fn prepare_texture(&mut self, id: usize) {
        self.textures
            .load_texture(&self.state.device, &self.state.queue, id);
    }

    pub(crate) fn get_texture(&self, id: &usize) -> Option<&texture::Texture> {
        self.textures.get_texture(id)
    }

    pub(crate) fn render(&mut self, instructions: Instructions) -> Result<(), wgpu::SurfaceError> {
        let frame = self.state.surface.get_current_texture()?;
        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let vertices = bytemuck::cast_slice(instructions.vertices.as_slice());
        self.state
            .queue
            .write_buffer(&self.render_pass.vertex_buffer, 0, vertices);

        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(create_color_attachment(view))],
                depth_stencil_attachment: Some(create_depth_attachment(
                    &self.render_pass.depth_texture_view,
                )),
            });

            render_pass.set_pipeline(&self.render_pass.pipeline);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.render_pass.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.render_pass.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            let mut offset = 0;
            for batch in instructions.batches {
                if let Some(bind_group) = self.textures.get_bind_group(&batch.texture_id) {
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.draw_indexed(offset..(offset + batch.size * 6), 0, 0..1);
                }
                offset += batch.size * 6;
            }
        }
        self.state.queue.submit(once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
