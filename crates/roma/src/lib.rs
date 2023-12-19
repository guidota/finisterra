pub use camera::*;
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

pub mod ui {
    pub use yakui::*;
}

mod camera;
mod renderer;
mod roma;
pub mod settings;
mod state;

pub type Position = [f32; 3];
pub type Color = [u8; 4];
pub type Source = [f32; 2];

/// Note: there is some packing and unpacking happening here:
/// u16 pairs will be unpacked from u32.
///
/// This affects to `x`, `y` and `source`
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct DrawImageParams {
    pub x: u16,
    pub y: u16,
    pub z: f32,
    pub color: [u8; 4],
    pub source: [u16; 4],
}

impl DrawImageParams {
    pub fn new(position: &[f32; 3], color: Color, source: [u16; 4]) -> Self {
        let [x, y, z] = position;

        Self {
            x: *x as u16,
            y: *y as u16,
            z: *z,
            color,
            source,
        }
    }
}

impl Default for DrawImageParams {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0.,
            color: [255, 255, 255, 255],
            source: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DrawTextParams {
    pub text: SmolStr,
    pub position: Position,
    pub color: Color,
}
