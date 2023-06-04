use bumpalo::Bump;

use crate::draw::DrawStrictParams;

#[repr(C)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub fn draw_params_to_vertex(params: &DrawStrictParams) -> Vec<Vertex> {
    let texture_width = params.texture_width;
    let texture_height = params.texture_height;
    let flip_y = params.flip_y;
    let x = params.x;
    let y = params.y;
    let z = params.z;
    let sx = params.sx;
    let sy = params.sy;
    let sw = params.sw;
    let sh = params.sh;

    let p = [
        [x, y, z],
        [x + sw, y, z],
        [x + sw, y + sh, z],
        [x, y + sh, z],
    ];

    let mut tex_coords = [
        [sx / texture_width, sy / texture_height],
        [(sx + sw) / texture_width, sy / texture_height],
        [(sx + sw) / texture_width, (sy + sh) / texture_height],
        [sx / texture_width, (sy + sh) / texture_height],
    ];

    if flip_y {
        tex_coords.swap(0, 3);
        tex_coords.swap(1, 2);
    }

    let mut vertices = Vec::with_capacity(4);
    for i in 0..4 {
        let vertex = Vertex {
            position: p[i],
            tex_coords: tex_coords[i],
        };
        vertices.push(vertex);
    }
    vertices
}
