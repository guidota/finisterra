use wgpu::{util::DeviceExt, *};

use crate::resources::camera::CameraUniform;

pub(crate) struct Camera {
    uniform: CameraUniform,
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn init(device: &wgpu::Device) -> Self {
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
    pub fn update_projection(&mut self, queue: &wgpu::Queue, projection: cgmath::Matrix4<f32>) {
        self.uniform.set_projection(projection);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
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
