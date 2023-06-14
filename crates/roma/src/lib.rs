pub use roma::*;
pub use settings::*;
pub use smol_str::SmolStr;
pub use wgpu::PresentMode;
pub use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
pub use winit_input_helper::WinitInputHelper;

mod camera;
mod renderer;
mod roma;
pub mod settings;
mod state;

pub type Position = [f32; 3];
pub type Color = [f32; 4];
pub type Rect = [f32; 4];

#[derive(Default, Debug, Clone)]
pub struct DrawImageParams {
    pub texture_id: usize,
    pub position: Position,
    pub source: Option<Rect>,
    pub color: Color,
    pub flip_y: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DrawTextParams {
    pub text: SmolStr,
    pub position: Position,
    pub color: Color,
}
