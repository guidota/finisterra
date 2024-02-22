use cgmath::ortho;
use engine::camera::{Position, Viewport, Zoom};

pub struct Camera {
    pub viewport: Viewport,
    pub position: Position,
    pub zoom: Zoom,
}

impl Camera {
    pub fn initialize(size: engine::window::Size) -> Self {
        Self {
            viewport: Viewport {
                x: 0.,
                y: 0.,
                width: size.width as f32,
                height: size.height as f32,
            },
            zoom: Zoom::None,
            position: Position { x: 0., y: 0. },
        }
    }

    // pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
    //     let zoom = match self.zoom {
    //         Zoom::None => 1.,
    //         Zoom::Double => 2.,
    //     };
    // let left = self.position.x - self.viewport.width / zoom / 2.;
    // let right = self.position.x + self.viewport.width / zoom / 2.;
    // let bottom = self.position.y - self.viewport.height / zoom / 2.;
    // let top = self.position.y + self.viewport.height / zoom / 2.;
    //
    // ortho(left, right, bottom, top, -1., 0.).into()
    // }

    pub fn build_ui_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let left = self.position.x + self.viewport.x;
        let right = self.position.x + self.viewport.x + self.viewport.width;
        let bottom = self.position.y + self.viewport.y;
        let top = self.position.y + self.viewport.y + self.viewport.height;

        ortho(left, right, bottom, top, -1., 0.).into()
    }

    pub fn build_view_projection_matrix(&self, centered: bool) -> [[f32; 4]; 4] {
        let zoom = match self.zoom {
            Zoom::None => 1.,
            Zoom::Double => 2.,
        };
        if centered {
            let left = self.position.x - self.viewport.width / zoom / 2.;
            let right = self.position.x + self.viewport.width / zoom / 2.;
            let bottom = self.position.y - self.viewport.height / zoom / 2.;
            let top = self.position.y + self.viewport.height / zoom / 2.;

            ortho(left, right, bottom, top, -1., 0.).into()
        } else {
            let left = self.position.x + self.viewport.x;
            // let right = self.position.x + self.viewport.x + self.viewport.width / zoom;
            let bottom = self.position.y + self.viewport.y;
            // let top = self.position.y + self.viewport.y + self.viewport.height / zoom;
            // let left = self.position.x - self.viewport.width / zoom;
            let right = self.position.x + self.viewport.width / zoom;
            // let bottom = self.position.y - self.viewport.height / zoom;
            let top = self.position.y + self.viewport.height / zoom;
            ortho(left, right, bottom, top, -1., 0.).into()
        }
    }
}
