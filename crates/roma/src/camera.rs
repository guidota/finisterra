use cgmath::ortho;
use wgpu::{util::DeviceExt, *};

use crate::roma::{get_device, get_state};

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
    pub fn new() -> Self {
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
pub struct Camera2D {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Camera2D {
    pub fn new(width: usize, height: usize) -> Camera2D {
        Camera2D {
            width,
            height,
            x: 0,
            y: 0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let left = self.x as f32 - self.width as f32 / 2.;
        let right = self.x as f32 + self.width as f32 / 2.;
        let bottom = self.y as f32 - self.height as f32 / 2.;
        let top = self.y as f32 + self.height as f32 / 2.;

        ortho(left, right, bottom, top, -1., 0.)
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
pub(crate) struct Camera {
    uniform: CameraUniform,
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn init() -> Self {
        let device = get_device();
        let uniform = CameraUniform::new();
        let bind_group_layout = device.create_camera_bind_group_layout();
        let buffer = device.create_camera_buffer();
        let bind_group = device.create_camera_bind_group(&bind_group_layout, &buffer);

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
    pub fn update_projection(&mut self, projection: cgmath::Matrix4<f32>) {
        let state = get_state();
        self.uniform.set_projection(projection);
        state
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

pub(crate) trait DeviceCameraExt {
    fn create_camera_buffer(&self) -> Buffer;
    fn create_camera_bind_group_layout(&self) -> BindGroupLayout;
    fn create_camera_bind_group(&self, layout: &BindGroupLayout, buffer: &Buffer) -> BindGroup;
}

impl DeviceCameraExt for Device {
    fn create_camera_buffer(&self) -> Buffer {
        self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[CameraUniform::new()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn create_camera_bind_group_layout(&self) -> BindGroupLayout {
        self.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
        })
    }

    fn create_camera_bind_group(&self, layout: &BindGroupLayout, buffer: &Buffer) -> BindGroup {
        self.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        })
    }
}
