use roma::{
    assets::{app_base_path, Image},
    bytemuck,
    render::{
        camera::create_camera_bind_group_layout,
        renderer::{LayoutKind, PhysicalSize},
        texture::{
            self, create_render_pipeline, create_texture_bind_group_layout, Texture, Vertex,
        },
    },
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Extent3d, RenderPipeline,
};

use crate::{RenderData, Renderer};

use super::components::Position;

impl RenderData {
    pub fn prepare_texture(&mut self, image: &Image, renderer: &Renderer) {
        if self.textures.contains_key(&image.file) {
            return;
        }

        let bind_group_layout = self.get_bind_group_layout(LayoutKind::Texture, renderer);
        let file_path = app_base_path()
            .join(&format!("assets/{}.png", image.file))
            .get();
        let texture = Texture::from_file(
            &renderer.inner.device,
            &renderer.inner.queue,
            &file_path,
            &image.id,
            bind_group_layout,
        )
        .unwrap();

        self.textures.insert(image.file.clone(), texture);
    }

    pub fn get_bind_group_layout(
        &mut self,
        layout: LayoutKind,
        renderer: &Renderer,
    ) -> &BindGroupLayout {
        self.layouts
            .entry(layout)
            .or_insert_with_key(|layout| match layout {
                LayoutKind::Texture => create_texture_bind_group_layout(&renderer.inner.device),
                LayoutKind::Camera => create_camera_bind_group_layout(&renderer.inner.device),
            })
    }

    pub fn create_sprite_buffers(
        &self,
        image: &Image,
        position: &Position,
        renderer: &Renderer,
    ) -> (Buffer, Buffer) {
        let Texture { size, .. } = self.textures.get(&image.file).unwrap();
        let vertices = create_vertices(image, size, position, renderer.inner.size());

        let vertex_buffer = renderer
            .inner
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: BufferUsages::VERTEX,
            });
        let index_buffer = renderer
            .inner
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: BufferUsages::INDEX,
            });

        (vertex_buffer, index_buffer)
    }

    pub fn get_bind_group(&self, image_id: &str) -> Option<&BindGroup> {
        self.textures
            .get(image_id)
            .map(|texture| &texture.bind_group)
    }

    pub fn get_render_pipeline(
        &mut self,
        layout: LayoutKind,
        renderer: &Renderer,
    ) -> &RenderPipeline {
        let _ = self.get_bind_group_layout(LayoutKind::Texture, renderer);
        let _ = self.get_bind_group_layout(LayoutKind::Camera, renderer);

        let texture_bind_group_layout = self.layouts.get(&LayoutKind::Texture).unwrap();
        let camera_bind_group_layout = self.layouts.get(&LayoutKind::Camera).unwrap();
        self.pipelines.entry(layout).or_insert_with(|| {
            create_render_pipeline(
                &renderer.inner.device,
                &renderer.inner.config,
                texture_bind_group_layout,
                camera_bind_group_layout,
            )
        })
    }
}

const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];
fn create_vertices(
    image: &Image,
    size: &Extent3d,
    position: &Position,
    window_size: PhysicalSize<f32>,
) -> [texture::Vertex; 4] {
    let (tex_width, tex_height) = (size.width as f32, size.height as f32);
    let (x, y, w, h) = (
        image.x as f32,
        image.y as f32,
        image.width as f32,
        image.height as f32,
    );
    let (offset_x, offset_y) = (position.x as f32, position.y as f32);
    [
        // Top Left
        Vertex {
            position: [offset_x / window_size.width, offset_y / window_size.height],
            tex_coords: [x / tex_width, (y + h) / tex_height],
        },
        // Top Right
        Vertex {
            position: [
                (w + offset_x) / window_size.width,
                offset_y / window_size.height,
            ],
            tex_coords: [(x + w) / tex_width, (y + h) / tex_height],
        },
        // Bottom Left
        Vertex {
            position: [
                (0.0 + offset_x) / window_size.width,
                (h + offset_y) / window_size.height,
            ],
            tex_coords: [x / tex_width, y / tex_height],
        },
        // Bottom Right
        Vertex {
            position: [
                (w + offset_x) / window_size.width,
                (h + offset_y) / window_size.height,
            ],
            tex_coords: [(x + w) / tex_width, y / tex_height],
        },
    ]
}
