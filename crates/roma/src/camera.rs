use cgmath::ortho;

#[derive(Debug)]
pub enum Zoom {
    None,
    Double,
}

#[derive(Debug)]
pub struct Camera2D {
    pub width: f32,
    pub height: f32,
    x: f32,
    y: f32,

    pub zoom: Zoom,
}

impl Camera2D {
    pub fn new(width: f32, height: f32) -> Camera2D {
        Camera2D {
            width,
            height,
            x: 0.,
            y: 0.,
            zoom: Zoom::None,
        }
    }

    pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let zoom = match self.zoom {
            Zoom::None => 1.,
            Zoom::Double => 2.,
        };
        let left = self.x - self.width / zoom / 2.;
        let right = self.x + self.width / zoom / 2.;
        let bottom = self.y - self.height / zoom / 2.;
        let top = self.y + self.height / zoom / 2.;

        ortho(left, right, bottom, top, -1., 0.).into()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_zoom(&mut self, zoom: Zoom) {
        self.zoom = zoom;
    }
}
