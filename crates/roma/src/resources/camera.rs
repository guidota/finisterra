use cgmath::ortho;

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
