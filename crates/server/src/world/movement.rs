use std::time::{Duration, Instant};

use shared::{
    protocol::{
        movement::next_position,
        server::{CharacterUpdate, ServerPacket},
    },
    world::{Direction, WorldPosition},
};

use super::{networking::Target, Entity, World};

impl World {
    pub fn process_pending_moves(&mut self) {
        let entity_ids = self.entities.keys().cloned().collect::<Vec<_>>();
        for entity_id in entity_ids {
            self.process_move(entity_id);
        }
    }

    fn process_move(&mut self, entity_id: u32) {
        if let Some(Entity::Character {
            character,
            ref mut last_move,
            ref mut pending_moves,
            ..
        }) = self.entities.get_mut(&entity_id)
        {
            if pending_moves.is_empty() {
                return;
            }
            let now = Instant::now();
            if now < *last_move + Duration::from_millis(200) {
                return;
            }
            // we are ready to process a move request
            let move_request = pending_moves.pop_front().unwrap();
            *last_move = now;
            let next_position = next_position(&character.position, move_request.direction);
            let result = MoveOutput::Move {
                position: next_position,
            };

            match result {
                MoveOutput::TooSoon => {}
                MoveOutput::Heading { direction } => {
                    let position = character.position;
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::MoveResponse {
                            request_id: move_request.id,
                            position,
                        }),
                        Target::User { entity_id },
                    );
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::Heading {
                            entity_id,
                            direction,
                        }),
                        Target::AreaButUser { entity_id },
                    )
                }
                MoveOutput::Move { position } => {
                    character.position = position;
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::MoveResponse {
                            request_id: move_request.id,
                            position,
                        }),
                        Target::User { entity_id },
                    );
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::Move {
                            entity_id,
                            position,
                        }),
                        Target::AreaButUser { entity_id },
                    );
                }
            };
        }
    }
}

enum MoveOutput {
    TooSoon,
    Heading { direction: Direction },
    Move { position: WorldPosition },
}
