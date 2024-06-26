use std::time::Duration;

use engine::{engine::GameEngine, input::keyboard::KeyCode};
use shared::world::Direction;

use crate::{
    game::Context,
    ui::{
        colors::{transparent, CYAN, YELLOW},
        fonts::TAHOMA_BOLD_8_SHADOW_ID,
    },
};

use super::{
    entity::{Character, Entity},
    WorldScreen,
};

impl WorldScreen {
    pub fn process_input<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let mut push = |direction| {
            if !self.input.contains(&direction) {
                self.input.push_front(direction);
            }
        };

        if context.engine.key_pressed(KeyCode::ArrowUp) {
            push(Direction::North);
        }
        if context.engine.key_pressed(KeyCode::ArrowDown) {
            push(Direction::South);
        }
        if context.engine.key_pressed(KeyCode::ArrowRight) {
            push(Direction::East);
        }
        if context.engine.key_pressed(KeyCode::ArrowLeft) {
            push(Direction::West);
        }
        if context.engine.key_released(KeyCode::ArrowUp) {
            self.input.retain(|dir| dir != &Direction::North);
        }
        if context.engine.key_released(KeyCode::ArrowDown) {
            self.input.retain(|dir| dir != &Direction::South);
        }
        if context.engine.key_released(KeyCode::ArrowRight) {
            self.input.retain(|dir| dir != &Direction::East);
        }
        if context.engine.key_released(KeyCode::ArrowLeft) {
            self.input.retain(|dir| dir != &Direction::West);
        }

        // TODO: remove
        if context.engine.key_pressed(KeyCode::KeyH) {
            if let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) {
                character.add_dialog(
                    context.engine,
                    "Rahma Nañarak O'al",
                    TAHOMA_BOLD_8_SHADOW_ID,
                    CYAN,
                );
            }
        }

        if context.engine.key_pressed(KeyCode::KeyI) {
            if let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) {
                if character.is_invisible() {
                    character.remove_invisible();
                } else {
                    character.set_invisible(
                        context.engine,
                        Duration::from_secs(15),
                        transparent(YELLOW, 128),
                    );
                }
            }
        }
        if context.engine.key_pressed(KeyCode::KeyR) {
            if let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) {
                character.animation = Character::random(context.resources);
            }
        }
    }
}
