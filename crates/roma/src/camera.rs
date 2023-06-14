use cgmath::ortho;

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
            // width: width / 2,
            // height: height / 2,
            width,
            height,
            x: 0,
            y: 0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let left = self.x as f32 - self.width as f32 / 2.;
        let right = self.x as f32 + self.width as f32 / 2.;
        let bottom = self.y as f32 - self.height as f32 / 2.;
        let top = self.y as f32 + self.height as f32 / 2.;

        ortho(left, right, bottom, top, -1., 0.).into()
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
