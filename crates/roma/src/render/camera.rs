use wgpu::{util::DeviceExt, Device, Queue};
use winit::dpi::PhysicalSize;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

pub struct CameraState {
    pub camera: Camera,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
}

impl CameraState {
    pub fn new(device: &Device) -> Self {
        let camera = Camera::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.x, camera.y]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = create_camera_bind_group_layout(device);

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            camera,
            camera_buffer,
            camera_bind_group,
        }
    }

    pub fn update_projection(&self, queue: &Queue, window_size: PhysicalSize<f32>) {
        let uniform = [
            self.camera.x / window_size.width,
            self.camera.y / window_size.height,
        ];
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}

pub fn create_camera_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

impl Camera {
    pub fn new() -> Self {
        Self {
            x: -200. / 800.,
            y: -200. / 600.,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
