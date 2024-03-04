pub trait Game {
    fn initialize<E: crate::engine::GameEngine>(engine: &mut E) -> Self;
    fn tick<E: crate::engine::GameEngine>(&mut self, engine: &mut E);
}

pub async fn run_game<G: Game, E: crate::engine::GameEngine>(settings: crate::settings::Settings) {
    let event_loop =
        winit::event_loop::EventLoop::new().expect("[run_game] couldn't initialize event loop");

    let window = winit::window::WindowBuilder::new()
        .with_title(settings.title.clone())
        .with_inner_size(winit::dpi::PhysicalSize::new(
            settings.width as u32,
            settings.height as u32,
        ))
        .with_decorations(false)
        .with_resizable(false)
        .build(&event_loop)
        .expect("[run_game] couldn't create window");

    let mut engine = E::initialize(window, &settings);
    let mut game = G::initialize(&mut engine);

    let mut last_tick = std::time::Instant::now();
    let event_loop_end = event_loop.run(move |event, window_target| {
        engine.handle_event(&event);

        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            logical_key:
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            ..
                        },
                    ..
                } => window_target.exit(),
                winit::event::WindowEvent::Resized(size) => {
                    engine.set_window_size(crate::window::Size {
                        width: size.width as u16,
                        height: size.height as u16,
                    });
                }
                winit::event::WindowEvent::RedrawRequested => {
                    let now = std::time::Instant::now();
                    let delta = now - last_tick;
                    last_tick = now;

                    engine.set_delta(delta);
                    game.tick(&mut engine);
                    engine.render();
                    engine.finish();
                }

                _ => {}
            }
        }
    });

    match event_loop_end {
        Err(e) => log::error!("[run_game] error while running event loop: {:#?}", e),
        Ok(ok) => log::info!("[run_game] event loop ended correctly: {:#?}", ok),
    }
}
