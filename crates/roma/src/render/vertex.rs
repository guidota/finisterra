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

impl From<&DrawStrictParams> for Vec<Vertex> {
    fn from(value: &DrawStrictParams) -> Self {
        let texture_width = value.texture_width;
        let texture_height = value.texture_height;
        let flip_y = value.flip_y;
        let x = value.x;
        let y = value.y;
        let z = value.z;
        let sx = value.sx;
        let sy = value.sy;
        let sw = value.sw;
        let sh = value.sh;

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

        let mut vertices = vec![];
        for i in 0..4 {
            vertices.push(Vertex {
                position: p[i],
                tex_coords: tex_coords[i],
            });
        }
        vertices
    }
}
