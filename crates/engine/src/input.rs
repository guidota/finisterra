pub mod keyboard {
    pub use winit::keyboard::*;
}

pub mod mouse {
    pub struct Position {
        pub x: f32,
        pub y: f32,
    }
}
