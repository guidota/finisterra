use std::time::{Duration, Instant};

use engine::engine::GameEngine;
use shared::protocol::{
    client::{Action, ClientPacket},
    movement::MoveRequest,
};

use crate::game::Context;

use super::{entity::Entity, WorldScreen};

impl WorldScreen {
    /// Client side prediction
    pub fn start_move<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) {
            let elapsed_since_last_move = Instant::now() - self.last_move;
            let latency = context.connection.ping();
            let was_idle_but_wants_to_move = !character.moving() && !self.input.is_empty();

            if (was_idle_but_wants_to_move || character.just_finished_moving())
                && elapsed_since_last_move >= Duration::from_millis(200 - latency as u64)
            {
                if let Some(direction) = self.input.front() {
                    tracing::info!(
                        "starting move {}, last move was {}ms ago",
                        self.movement_sequence,
                        elapsed_since_last_move.as_millis()
                    );
                    let map = context.maps.get(&character.position.map);
                    let position = map.next_position(&character.position, *direction);
                    character.change_direction(*direction);
                    character.move_to(position);

                    self.last_move = Instant::now();
                    self.predictions.push((self.movement_sequence, position));
                    context
                        .connection
                        .send(ClientPacket::UserAction(Action::Move(MoveRequest {
                            id: self.movement_sequence,
                            direction: *direction,
                        })));
                    self.movement_sequence += 1;
                }
            }
        }
    }
}
