use engine::{
    draw::{image::DrawImage, Dimensions, Position, Target},
    engine::{GameEngine, TextureID},
};
use itertools::iproduct;

use crate::{
    game::Context,
    screens::world::{
        depth::Z, entity::Entity, get_range, WorldScreen, HORIZONTAL_TILES, TILE_SIZE,
        VERTICAL_TILES, WHITE, WORLD_RENDER_HEIGHT, WORLD_RENDER_WIDTH,
    },
    texture::TextureState,
};

#[derive(Default)]
pub struct WorldMap {
    ground: (TextureID, TextureState),
    roof: (TextureID, TextureState),
}

impl WorldMap {
    pub fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let ground_texture_id = context.engine.create_texture(Dimensions {
            width: TILE_SIZE * 100,
            height: TILE_SIZE * 100,
        });
        let roof_texture_id = context.engine.create_texture(Dimensions {
            width: TILE_SIZE * 100,
            height: TILE_SIZE * 100,
        });

        Self {
            ground: (ground_texture_id, TextureState::Dirty),
            roof: (roof_texture_id, TextureState::Dirty),
        }
    }
}

impl WorldScreen {
    pub fn draw_world_2<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if matches!(self.map.ground.1, TextureState::Dirty) {
            let target = Target::Texture {
                id: self.map.ground.0,
            };
            self.draw_map_layer_to(context, 0, target);
            self.map.ground.1 = TextureState::JustDraw;
        }

        if matches!(self.map.roof.1, TextureState::Dirty) {
            let target = Target::Texture {
                id: self.map.roof.0,
            };
            self.draw_map_layer_to(context, 3, target);
            self.map.roof.1 = TextureState::JustDraw;
        }

        if matches!(self.map.ground.1, TextureState::JustDraw) {
            self.map.ground.1 = TextureState::Ready;
        } else {
            self.draw_world_prerendered_layer_character_vision(context, self.map.ground.0, 0.);
        }
        self.draw_world_character_vision(context, 1);
        self.draw_world_character_vision(context, 2);
        if matches!(self.map.roof.1, TextureState::JustDraw) {
            self.map.roof.1 = TextureState::Ready;
        } else {
            self.draw_world_prerendered_layer_character_vision(context, self.map.roof.0, 0.99);
        }
    }

    fn draw_map_layer_to<E: GameEngine>(
        &mut self,
        context: &mut Context<E>,
        layer: usize,
        target: Target,
    ) {
        let Some(Entity::Character(character)) = self.entities.get(&self.entity_id) else {
            return;
        };
        let map = context.maps.get(&character.position.map);

        for (y, x) in iproduct!(0..100, 0..100) {
            let tile = &map.tiles[x][y];

            let world_x = (x as u16 * TILE_SIZE) + TILE_SIZE;
            let world_y = (y as u16 * TILE_SIZE) + TILE_SIZE;

            if tile.graphics[layer] != 0 {
                let z = Z[layer][x][y];
                let image = &context.resources.images[tile.graphics[layer]];
                let position = Position::new(world_x + 16 - image.width / 2, world_y, z);

                context.engine.draw_image(
                    DrawImage {
                        position,
                        color: WHITE,
                        source: [image.x, image.y, image.width, image.height],
                        index: image.file,
                    },
                    target,
                );
            };
        }
    }

    fn draw_world_prerendered_layer_character_vision<E: GameEngine>(
        &mut self,
        context: &mut Context<E>,
        texture_id: TextureID,
        z: f32,
    ) {
        let Some(Entity::Character(character)) = self.entities.get(&self.entity_id) else {
            return;
        };
        let (x, y) = character.render_position();
        let x = x as u16 - (WORLD_RENDER_WIDTH + 1) / 2;
        let y_start = y as u16 - (WORLD_RENDER_HEIGHT + 1) / 2;
        let source_y = 3200 - y as u16 - (VERTICAL_TILES / 2 * TILE_SIZE) - TILE_SIZE;

        context.engine.draw_image(
            DrawImage {
                position: Position::new(x, y_start, z),
                color: WHITE,
                source: [
                    x,
                    source_y,
                    WORLD_RENDER_WIDTH,
                    WORLD_RENDER_HEIGHT + TILE_SIZE,
                ],
                index: texture_id,
            },
            Target::World,
        );
    }

    fn draw_world_character_vision<E: GameEngine>(
        &mut self,
        context: &mut Context<E>,
        layer: usize,
    ) {
        let Some(Entity::Character(character)) = self.entities.get(&self.entity_id) else {
            return;
        };
        let position = &character.position;
        let map = context.maps.get(&position.map);
        const EXTRA_TILES: u16 = 5;
        const HORIZONTAL_EXTRA_TILES: u16 = ((HORIZONTAL_TILES + 1) / 2) + EXTRA_TILES;
        const VERTICAL_EXTRA_TILES: u16 = (VERTICAL_TILES / 2) + EXTRA_TILES;
        let (x_start, x_end, y_start, y_end) =
            get_range(position, HORIZONTAL_EXTRA_TILES, VERTICAL_EXTRA_TILES);

        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &map.tiles[x][y];

            let world_x = (x as u16 * TILE_SIZE) + TILE_SIZE;
            let world_y = (y as u16 * TILE_SIZE) + TILE_SIZE;

            if tile.graphics[layer] != 0 {
                let z = Z[layer][x][y];
                let image = &context.resources.images[tile.graphics[layer]];
                let position = Position::new(world_x + 16 - image.width / 2, world_y, z);
                let color = WHITE;

                context.engine.draw_image(
                    DrawImage {
                        position,
                        color,
                        source: [image.x, image.y, image.width, image.height],
                        index: image.file,
                    },
                    Target::World,
                );
            };

            if layer == 2 {
                if let Some(entity_id) = tile.user {
                    if let Some(entity) = self.entities.get_mut(&entity_id) {
                        entity.draw(context.engine, context.resources);
                    }
                }
            }
        }
    }

    pub fn map_changed(&mut self) {
        self.map.ground.1 = TextureState::Dirty;
        self.map.roof.1 = TextureState::Dirty;
    }
}
