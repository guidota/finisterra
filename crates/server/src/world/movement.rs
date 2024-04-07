use std::time::{Duration, Instant};

use shared::{
    protocol::server::{CharacterUpdate, ServerPacket},
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

            let Some(map) = self.maps.get(&character.position.map) else {
                return;
            };
            // we are ready to process a move request
            let move_request = pending_moves.pop_front().unwrap();
            *last_move = now;

            let old_position = character.position;
            let next_position = map.next_position(&character.position, move_request.direction);
            let result = if next_position == old_position {
                MoveOutput::Heading {
                    direction: move_request.direction,
                }
            } else if let Some(position) = map.tile(next_position.x, next_position.y).exit {
                MoveOutput::Translate { position }
            } else {
                MoveOutput::Move {
                    position: next_position,
                }
            };
            if let Some(map) = self.maps.get_mut(&old_position.map) {
                map.tile_mut(old_position.x, old_position.y).user = None;
            }
            if let Some(map) = self.maps.get_mut(&next_position.map) {
                map.tile_mut(next_position.x, next_position.y).user = Some(entity_id);
            }

            match result {
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
                MoveOutput::Translate { position } => {
                    character.position = position;
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::Translate {
                            entity_id,
                            position,
                        }),
                        Target::User { entity_id },
                    );
                    self.send(
                        ServerPacket::CharacterUpdate(CharacterUpdate::Remove { entity_id }),
                        Target::Area {
                            _position: old_position,
                        },
                    );
                }
            };
        }
    }
}

enum MoveOutput {
    Heading { direction: Direction },
    Move { position: WorldPosition },
    Translate { position: WorldPosition },
}
