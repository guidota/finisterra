use bevy_ecs::{
    prelude::Entity,
    query::{Added, Changed, Or, Without},
    system::{Commands, Query, Res, ResMut},
};
use roma::render::renderer::LayoutKind;

use crate::{Assets, RenderData, Renderer};

use super::components::{Position, Sprite, SpriteBuffers};

pub fn prepare(
    query: Query<&Sprite, Or<(Added<Sprite>, Changed<Sprite>)>>,
    assets: Res<Assets>,
    mut render_data: ResMut<RenderData>,
    renderer: Res<Renderer>,
) {
    for sprite in &query {
        println!("Going to prepare image texture {}", sprite.id);
        if let Some(image) = assets.inner.images.get(&sprite.id) {
            println!("Preparing image texture {}", sprite.id);
            render_data.prepare_texture(image, &renderer);
        }
    }
}

pub fn update_sprite_buffers(
    mut query: Query<
        (&mut SpriteBuffers, &Sprite, &Position),
        Or<(Changed<Position>, Changed<Sprite>)>,
    >,
    _render_data: Res<RenderData>,
) {
    for (mut _buffers, _sprite, _position) in query.iter_mut() {
        // queue write buffer
    }
}

pub fn insert_sprite_buffers(
    mut commands: Commands,
    query: Query<(Entity, &Sprite, &Position), Without<SpriteBuffers>>,
    render_data: Res<RenderData>,
    renderer: Res<Renderer>,
    assets: Res<Assets>,
) {
    for (entity, sprite, position) in &query {
        if let Some(image) = assets.inner.images.get(&sprite.id) {
            println!("Inserting sprite buffers");
            let (vertex_buffer, index_buffer) =
                render_data.create_sprite_buffers(image, position, &renderer);

            commands.entity(entity).insert(SpriteBuffers {
                vertex_buffer,
                index_buffer,
            });
        }
    }
}

pub fn render(
    render_data: Res<RenderData>,
    assets: Res<Assets>,
    mut renderer: ResMut<Renderer>,
    query: Query<(&Sprite, &SpriteBuffers)>,
) {
    renderer.inner.update();
    let mut draws = vec![];
    for (
        sprite,
        SpriteBuffers {
            vertex_buffer,
            index_buffer,
        },
    ) in &query
    {
        if let Some(image) = assets.inner.images.get(&sprite.id) {
            let bind_group = render_data.get_bind_group(&image.file);
            if let Some(bind_group) = bind_group {
                let pipeline = render_data.pipelines.get(&LayoutKind::Texture);
                if let Some(pipeline) = pipeline {
                    draws.push((pipeline, bind_group, (vertex_buffer, index_buffer)));
                }
            } else {
            }
        }
    }
    let _ = renderer.inner.render(&draws);
}
