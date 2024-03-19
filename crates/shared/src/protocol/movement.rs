use bincode::{Decode, Encode};

use crate::world::{Direction, WorldPosition};

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
pub struct MoveRequest {
    pub id: u8,
    pub direction: Direction,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
pub struct MoveResponse {
    pub request_id: u8,
    pub position: WorldPosition,
}

pub fn next_position(position: &WorldPosition, direction: Direction) -> WorldPosition {
    match direction {
        Direction::North => WorldPosition {
            map: position.map,
            x: position.x,
            y: position.y + 1,
        },
        Direction::East => WorldPosition {
            map: position.map,
            x: position.x + 1,
            y: position.y,
        },
        Direction::South => WorldPosition {
            map: position.map,
            x: position.x,
            y: position.y - 1,
        },
        Direction::West => WorldPosition {
            map: position.map,
            x: position.x - 1,
            y: position.y,
        },
    }
}
// pub struct Movement {
//     pub input: VecDeque<Move>,
//     pub predictions: Vec<Output>,
//
//     pub velocity: f64,
//     pub moving: Option<(u8, u8, Direction)>,
//
//     pub position: WorldPosition,
//     pub moving_position: (f64, f64),
// }

// impl Movement {
//     pub fn new(velocity: f64, position: WorldPosition) -> Self {
//         Self {
//             input: VecDeque::new(),
//             predictions: vec![],
//             velocity,
//             moving: None,
//             position,
//             moving_position: (0.0, 0.0),
//         }
//     }
//     pub fn start_move(&mut self, start: Move) {
//         if self
//             .input
//             .iter()
//             .any(|input| input.move_id == start.move_id || input.direction == start.direction)
//         {
//             // should not happen
//         } else {
//             self.input.push_front(start);
//         }
//     }
//
//     pub fn stop_move(&mut self, stop: Stop) {
//         self.input.retain(|input| input.direction != stop.direction);
//     }
//
//     /// TODO: consider map state (blocks, other entities, etc)
//     /// Prediction
//     pub fn update(&mut self, delta: Duration) -> bool {
//         if self.moving.is_none() {
//             if let Some(key) = self.input.front() {
//                 self.moving = Some((key.move_id, 0, key.direction));
//             }
//         }
//
//         if let Some((key_id, ref mut prediction_counter, direction)) = self.moving.as_mut() {
//             let distance = delta.as_secs_f64() * self.velocity;
//             match direction {
//                 Direction::South => self.moving_position.1 -= distance,
//                 Direction::North => self.moving_position.1 += distance,
//                 Direction::East => self.moving_position.0 += distance,
//                 Direction::West => self.moving_position.0 -= distance,
//             }
//             if arrived(self.moving_position, direction) {
//                 match direction {
//                     Direction::South => self.moving_position.1 += 1.,
//                     Direction::North => self.moving_position.1 -= 1.,
//                     Direction::East => self.moving_position.0 -= 1.,
//                     Direction::West => self.moving_position.0 += 1.,
//                 }
//                 let prediction = Output {
//                     move_id: *key_id,
//                     sequence_number: *prediction_counter,
//                     movement: Some(*direction),
//                 };
//                 *prediction_counter += 1;
//                 self.predictions.push(prediction);
//                 if let Some(key) = self.input.front() {
//                     let prediction_counter = if key_id == &key.move_id {
//                         *prediction_counter
//                     } else {
//                         0
//                     };
//                     self.moving = Some((key.move_id, prediction_counter, key.direction));
//                 } else {
//                     self.moving = None;
//                     self.moving_position = (0.0, 0.0);
//                 }
//                 return true;
//             }
//         }
//         false
//     }
//
//     /// Reconciliation
//     pub fn validate(&mut self, server_output: &Output) {
//         // if self.predictions.is_empty() {
//         //     if let Some(direction) = server_output.movement {
//         //         match direction {
//         //             Direction::North => {
//         //                 self.position.y += 1;
//         //             }
//         //             Direction::East => {
//         //                 self.position.x += 1;
//         //             }
//         //             Direction::South => {
//         //                 self.position.y -= 1;
//         //             }
//         //             Direction::West => {
//         //                 self.position.x -= 1;
//         //             }
//         //         }
//         //     }
//         //
//         //     return;
//         // }
//
//         let mut to_remove = vec![];
//         for i in 0..self.predictions.len() {
//             let client_prediction = &mut self.predictions[i];
//
//             if client_prediction.move_id == server_output.move_id
//                 && client_prediction.sequence_number == server_output.sequence_number
//             {
//                 match (&client_prediction.movement, &server_output.movement) {
//                     (Some(direction), None) => match direction {
//                         Direction::North => {
//                             self.position.y -= 1;
//                         }
//                         Direction::East => {
//                             self.position.x -= 1;
//                         }
//                         Direction::South => {
//                             self.position.y += 1;
//                         }
//                         Direction::West => {
//                             self.position.x += 1;
//                         }
//                     },
//                     (_, Some(direction)) => match direction {
//                         Direction::North => {
//                             self.position.y += 1;
//                         }
//                         Direction::East => {
//                             self.position.x += 1;
//                         }
//                         Direction::South => {
//                             self.position.y -= 1;
//                         }
//                         Direction::West => {
//                             self.position.x -= 1;
//                         }
//                     },
//                     _ => {}
//                 }
//             } else if client_prediction.move_id == server_output.move_id
//                 && client_prediction.sequence_number < server_output.sequence_number
//             {
//                 if let Some(direction) = client_prediction.movement {
//                     match direction {
//                         Direction::North => {
//                             self.position.y += 1;
//                         }
//                         Direction::East => {
//                             self.position.x += 1;
//                         }
//                         Direction::South => {
//                             self.position.y -= 1;
//                         }
//                         Direction::West => {
//                             self.position.x -= 1;
//                         }
//                     }
//                 }
//             }
//             if client_prediction.move_id == server_output.move_id
//                 && client_prediction.sequence_number <= server_output.sequence_number
//             {
//                 to_remove.push(i);
//             }
//         }
//
//         for i in to_remove.iter().rev() {
//             self.predictions.remove(*i);
//         }
//     }
//
//     pub fn moving_direction(&self) -> Option<Direction> {
//         self.moving.as_ref().map(|i| i.2)
//     }
//
//     pub fn tile_position(&self) -> WorldPosition {
//         let mut x = self.position.x;
//         let mut y = self.position.y;
//         for prediction in &self.predictions {
//             if let Some(direction) = &prediction.movement {
//                 match direction {
//                     Direction::North => y += 1,
//                     Direction::East => x += 1,
//                     Direction::South => y -= 1,
//                     Direction::West => x -= 1,
//                 }
//             }
//         }
//
//         WorldPosition {
//             map: self.position.map,
//             x,
//             y,
//         }
//     }
//
//     pub fn world_position(&self) -> (f64, f64) {
//         let WorldPosition { x, y, .. } = self.tile_position();
//
//         (
//             x as f64 + self.moving_position.0,
//             y as f64 + self.moving_position.1,
//         )
//     }
// }
//
// fn arrived(moving_position: (f64, f64), direction: &Direction) -> bool {
//     let x = moving_position.0;
//     let y = moving_position.1;
//
//     match direction {
//         Direction::South => y <= -1.0,
//         Direction::North => y >= 1.0,
//         Direction::East => x >= 1.0,
//         Direction::West => x <= -1.0,
//     }
// }
