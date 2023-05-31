use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn set_projection(&mut self, projection: cgmath::Matrix4<f32>) {
        self.view_proj = projection.into();
    }
}

#[derive(Debug)]
pub struct Camera {
    pub(crate) uniform: CameraUniform,
    pub(crate) buffer: Buffer,
    pub(crate) bind_group: BindGroup,
    pub(crate) bind_group_layout: BindGroupLayout,
}

pub(crate) trait DeviceCameraExt {
    fn create_camera(&self) -> Camera;
}

impl DeviceCameraExt for Device {
    fn create_camera(&self) -> Camera {
        let uniform = CameraUniform::new();
        let bind_group_layout = self.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let buffer = self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = self.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Camera {
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}
