use cgmath::ortho;

use crate::graphics::vec2::{vec2, Vec2};

#[derive(Debug)]
pub struct Camera2D {
    data: Camera2DOrtho,
    size: Vec2,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct Camera2DOrtho {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Camera2D {
    pub fn new(width: f32, height: f32) -> Camera2D {
        let data = Camera2DOrtho {
            left: 0.0,
            right: width,
            bottom: 0.0,
            top: height,
            near: -1.0,
            far: 1.0,
        };
        Camera2D {
            size: vec2(width, height),
            position: vec2(0., 0.),
            data,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let Camera2DOrtho {
            left,
            right,
            bottom,
            top,
            near,
            far,
        } = self.data;

        ortho(left, right, bottom, top, near, far)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;

        let Vec2 {
            x: camera_width,
            y: camera_height,
        } = self.size;
        let Vec2 { x, y } = position;
        self.data.left = x - camera_width / 2.;
        self.data.right = x + camera_width / 2.;
        self.data.bottom = y - camera_width / 2.;
        self.data.top = y + camera_height / 2.;
    }
}
