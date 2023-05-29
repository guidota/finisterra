use macroquad::prelude::*;

pub const ENTITY_SIZE: &(u32, u32) = &(100, 100);
pub fn create_entity_camera() -> Camera2D {
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.0,
        0.0,
        ENTITY_SIZE.0 as f32,
        ENTITY_SIZE.1 as f32,
    ));
    let render_target = render_target(ENTITY_SIZE.0, ENTITY_SIZE.1);
    render_target.texture.set_filter(FilterMode::Nearest);
    camera.render_target = Some(render_target);
    // camera.zoom.y = -camera.zoom.y;
    camera
}

pub fn create_ui_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800., 600.0))
}

pub fn create_world_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800., 600.))
}

pub fn create_map_static_camera() -> Camera2D {
    let (map_width, map_height) = (100 * 32, 100 * 32);
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, map_width as f32, map_height as f32));
    let render_target = render_target(map_width, map_height);
    render_target.texture.set_filter(FilterMode::Nearest);
    camera.render_target = Some(render_target);
    camera.zoom.y = -camera.zoom.y;
    camera
}
