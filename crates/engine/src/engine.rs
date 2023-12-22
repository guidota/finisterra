pub trait GameEngine {
    /// Initialize engine using the window provided by `winit`
    fn initialize(window: winit::window::Window, settings: &crate::settings::Settings) -> Self;

    /// Handle window events
    fn handle_event(&mut self, event: &winit::event::Event<()>);

    /// Check if a key was pressed
    fn key_pressed(&self, key: crate::input::keyboard::KeyCode) -> bool;

    /// Check if a key was released
    fn key_released(&self, key: crate::input::keyboard::KeyCode) -> bool;

    /// Check if a key is held
    fn key_held(&self, key: crate::input::keyboard::KeyCode) -> bool;

    /// Get mouse position in screen
    fn get_mouse_position(&self) -> crate::input::mouse::Position;

    /// Check mouse primary click
    fn mouse_clicked(&self) -> bool;

    /// Check mouse secondary click
    fn mouse_secondary_clicked(&self) -> bool;

    /// Adds a texture and receives an integer to be used later on draw_image
    fn add_texture(&mut self, path: &str) -> u64;

    /// Adds a texture that be used later on draw_image by using the id
    fn set_texture(&mut self, path: &str, id: u64);

    /// Draws an image in the screen
    /// `id` should be the identifier of the texture previously added or set
    /// `parameters` includes the instructions for rendering a texture or a portion of it
    /// it should be possible to render images from atlases
    fn draw_image(&mut self, id: u64, parameters: crate::draw::image::DrawImage);

    /// Adds a font and receives an integer to be used later on draw_text
    fn add_font(&mut self, id: u64, path: &str, texture_id: u64);

    /// Draws text in the screen
    /// `id` should be the identifier of the font previously added or set
    /// `parameters` includes the instructions for rendering the text
    fn draw_text(&mut self, id: u64, parameters: crate::draw::text::DrawText);

    /// Adds a sound and receives an integer to be used later on play_sound or play_music
    fn add_sound(&mut self, path: &str) -> u64;

    /// Adds a sound that can be used later on play_sound or play_music by using the id
    fn set_sound(&mut self, path: &str, id: u64);

    /// Plays a sound
    fn play_sound(&mut self, id: u64, parameters: crate::sound::PlaySound);

    /// Plays the music until stop_music or play_music is called
    fn play_music(&mut self, id: u64, parameters: crate::sound::PlayMusic);

    /// Stops music
    fn stop_music(&mut self);

    /// Get camera viewport
    fn get_world_camera_viewport(&self) -> crate::camera::Viewport;

    /// Camera viewport can be used to indicate where to render the game
    /// Use this method if you want to only render to a portion of the screen
    fn set_world_camera_viewport(&mut self, viewport: crate::camera::Viewport);

    /// Get camera zoom
    fn get_world_camera_zoom(&self) -> crate::camera::Zoom;

    /// Set camera zoom
    /// Using `None` means no scaling
    /// Using `Double` means scaling 2x
    fn set_world_camera_zoom(&mut self, zoom: crate::camera::Zoom);

    /// Get camera world position
    fn get_world_camera_position(&self) -> crate::camera::Position;

    /// Set camera position in the world
    fn set_world_camera_position(&mut self, position: crate::camera::Position);

    /// Get camera viewport
    fn get_ui_camera_viewport(&self) -> crate::camera::Viewport;

    /// Camera viewport can be used to indicate where to render the game
    /// Use this method if you want to only render to a portion of the screen
    fn set_ui_camera_viewport(&mut self, viewport: crate::camera::Viewport);

    /// Set the delta time since the last frame
    fn set_delta(&mut self, delta: std::time::Duration);

    /// Get the delta time since the last frame
    fn get_delta(&self) -> std::time::Duration;

    /// Get window size
    /// Useful when drawing a UI based on the window size
    fn get_window_size(&self) -> crate::window::Size;

    /// Set window size, automatically called by the engine, not meant to be called by final user
    fn set_window_size(&mut self, size: crate::window::Size);

    /// Call every frame after game updates
    fn render(&mut self);

    /// Call every frame at the end
    fn finish(&self);
}
