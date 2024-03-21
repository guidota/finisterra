use std::cmp::Ordering;

use shared::world::WorldPosition;

use super::{entity::Entity, WorldScreen};

impl WorldScreen {
    pub fn reconciliation(&mut self, request_id: u8, position: WorldPosition) {
        let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) else {
            return;
        };
        let mut to_remove = vec![];
        for (i, (prediction_request_id, predicted_position)) in self.predictions.iter().enumerate()
        {
            match prediction_request_id.cmp(&request_id) {
                Ordering::Less => {
                    to_remove.push(i);
                }
                Ordering::Equal => {
                    to_remove.push(i);
                    if predicted_position != &position {
                        tracing::error!("move {request_id} prediction was wrong");

                        if let Some(move_to) = character.position_buffer.first() {
                            if move_to == predicted_position {
                                character.position_buffer[0] = position;
                                continue;
                            }
                        }

                        correct_position(predicted_position, position, character);
                    }
                }
                Ordering::Greater => {}
            }
        }
        for i in to_remove.iter().rev() {
            self.predictions.remove(*i);
        }
    }
}

fn correct_position(
    predicted_position: &WorldPosition,
    position: WorldPosition,
    character: &mut super::entity::Character,
) {
    if predicted_position.x < position.x {
        character.position.x += 1;
        character.render_position.0 += 1.;
    } else if predicted_position.x > position.x {
        character.position.x -= 1;
        character.render_position.0 -= 1.;
    } else if predicted_position.y < position.y {
        character.position.y += 1;
        character.render_position.1 += 1.;
    } else if predicted_position.y > position.y {
        character.position.y -= 1;
        character.render_position.1 -= 1.;
    }
}
