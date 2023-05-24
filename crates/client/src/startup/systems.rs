use bevy_ecs::system::{Commands, Res, ResMut};
use roma::render::renderer::LayoutKind;

use crate::{
    render::components::{Position, Sprite},
    RenderData, Renderer,
};

pub fn spawn_sprite(mut commands: Commands) {
    println!("Spawning grass 1!");
    commands.spawn((
        Sprite {
            id: "grass-1".to_string(),
        },
        Position { x: 0, y: 0 },
    ));
    commands.spawn((
        Sprite {
            id: "grass-2".to_string(),
        },
        Position { x: 64, y: 0 },
    ));
    commands.spawn((
        Sprite {
            id: "grass-3".to_string(),
        },
        Position { x: 0, y: 64 },
    ));
    commands.spawn((
        Sprite {
            id: "grass-4".to_string(),
        },
        Position { x: 64, y: 64 },
    ));
}

pub fn prepare_pipelines(mut render_data: ResMut<RenderData>, renderer: Res<Renderer>) {
    let _ = render_data.get_bind_group_layout(LayoutKind::Texture, &renderer);
    let _ = render_data.get_render_pipeline(LayoutKind::Texture, &renderer);
}
