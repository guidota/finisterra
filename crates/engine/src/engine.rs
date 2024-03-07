use winit::window::CursorIcon;

use crate::draw::{Dimensions, Target};

pub type TextureID = u32;
pub type FontID = u32;
pub type SoundID = u32;

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

    fn held_keys(&self) -> Vec<crate::input::keyboard::Key>;
    fn pressed_keys(&self) -> Vec<crate::input::keyboard::Key>;
    fn released_keys(&self) -> Vec<crate::input::keyboard::Key>;

    /// Get mouse position in screen
    fn mouse_position(&self) -> crate::input::mouse::Position;

    /// Check mouse primary click
    fn mouse_clicked(&self) -> bool;

    /// Check mouse primary held  
    fn mouse_held(&self) -> bool;

    /// Check mouse primary released
    fn mouse_released(&self) -> bool;

    /// Check mouse secondary click
    fn mouse_secondary_clicked(&self) -> bool;

    /// Sets current mouse cursor
    fn set_mouse_cursor(&mut self, cursor: CursorIcon);

    /// Adds a texture and receives an integer to be used later on draw_image
    fn add_texture(&mut self, path: &str) -> TextureID;

    /// Adds a texture that be used later on draw_image by using the id
    fn set_texture(&mut self, path: &str, id: TextureID);

    fn texture_dimensions(&mut self, texture_id: TextureID) -> Option<(u16, u16)>;

    /// Creates a texture that can be used to draw images or text
    /// The texture of this target can be used as any other texture
    fn create_texture(&mut self, dimensions: Dimensions) -> TextureID;

    /// Draws an image in the specific target
    /// `id` should be the identifier of the texture previously added or set
    /// `parameters` includes the instructions for rendering a texture or a portion of it
    /// 'target' can be the world, the ui, or an specific texture
    /// it should be possible to render images from atlases
    fn draw_image(&mut self, parameters: crate::draw::image::DrawImage, target: Target);

    /// Adds a font and receives an integer to be used later on draw_text
    fn add_font(&mut self, id: FontID, path: &str, texture_id: TextureID);

    /// Draws text in the screen
    /// `id` should be the identifier of the font previously added or set
    /// `parameters` includes the instructions for rendering the text
    /// 'target' can be the world, the ui, or an specific texture
    fn draw_text(&mut self, id: FontID, parameters: crate::draw::text::DrawText, target: Target);

    /// Parse text and get character positions and width
    fn parse_text(&mut self, id: FontID, text: &str) -> Option<crate::draw::text::ParsedText>;

    /// Adds a sound and receives an integer to be used later on play_sound or play_music
    fn add_sound(&mut self, path: &str) -> SoundID;

    /// Adds a sound that can be used later on play_sound or play_music by using the id
    fn set_sound(&mut self, path: &str, id: SoundID);

    /// Plays a sound
    fn play_sound(&mut self, id: SoundID, parameters: crate::sound::PlaySound);

    /// Plays the music until stop_music or play_music is called
    fn play_music(&mut self, id: SoundID, parameters: crate::sound::PlayMusic);

    /// Stops music
    fn stop_music(&mut self);

    /// Get camera viewport
    fn get_world_camera_viewport(&self) -> crate::camera::Viewport;

    /// Camera viewport can be used to indicate where to render the game
    /// Use this method if you want to only render to a portion of the screen
    fn set_world_camera_viewport(&mut self, viewport: crate::camera::Viewport);

    /// Get camera zoom
    fn get_camera_zoom(&self) -> crate::camera::Zoom;

    /// Set camera zoom
    /// Using `None` means no scaling
    /// Using `Double` means scaling 2x
    fn set_camera_zoom(&mut self, zoom: crate::camera::Zoom);

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
