use assets::Assets;
pub use bytemuck;
use render::renderer::Renderer;
pub use wgpu::*;
pub use winit::event::Event;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod assets;
pub mod render;

pub struct Resources {
    pub assets: Assets,
    pub renderer: Renderer,
}

pub trait Game {
    fn new(resources: Resources) -> Self;
    fn input<T>(&mut self, _event: Event<T>) {}
    fn update(&mut self) {}
    fn render(&mut self) {}
    fn post_update(&mut self) {}
    fn resize(&mut self, _new_size: PhysicalSize<u32>, _scale_factor: f64) {}
    fn on_close(&mut self) {}
}

pub async fn run<G: Game + 'static>() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let renderer = Renderer::new(&window).await;
    let resources = Resources {
        renderer,
        assets: Assets::load(),
    };
    println!("Loaded assets {:?}", resources.assets);
    let mut game = G::new(resources);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                window.set_inner_size(*physical_size);
                game.resize(*physical_size, window.scale_factor());
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                window.set_inner_size(**new_inner_size);
                game.resize(**new_inner_size, window.scale_factor());
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            game.update();
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            game.render();
        }
        Event::RedrawEventsCleared => {
            game.post_update();
            window.request_redraw();
        }
        _ => (),
    });
}
