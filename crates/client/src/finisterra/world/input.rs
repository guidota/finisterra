// use std::time::Duration;
//
// use definitions::heading::Heading;
// use engine::{camera::Zoom, engine::GameEngine, input::keyboard::KeyCode};
//
// use crate::finisterra::Finisterra;
//
// use super::entity;
//
// impl Finisterra {
//     pub fn process_input<E: GameEngine>(&mut self, engine: &mut E) {
//         let delta = engine.get_delta();
//         let mut move_position = (0., 0.);
//         if engine.key_held(KeyCode::ArrowRight) {
//             self.entities[0].direction = Heading::East;
//             move_position.0 = 1.;
//         }
//
//         if engine.key_held(KeyCode::ArrowLeft) {
//             self.entities[0].direction = Heading::West;
//             move_position.0 = -1.;
//         }
//
//         if engine.key_held(KeyCode::ArrowDown) {
//             self.entities[0].direction = Heading::South;
//             move_position.1 = -1.;
//         }
//
//         if engine.key_held(KeyCode::ArrowUp) {
//             self.entities[0].direction = Heading::North;
//             move_position.1 = 1.;
//         }
//
//         if move_position.0 != 0. || move_position.1 != 0. {
//             let (x, y) = (self.position.0.floor(), self.position.1.floor());
//
//             let distance = 5. * delta.as_secs_f32();
//             self.position.0 += move_position.0 * distance;
//             self.position.1 += move_position.1 * distance;
//             self.entities[0].set_position(self.position.0, self.position.1);
//
//             let (new_x, new_y) = (self.position.0.floor(), self.position.1.floor());
//             if new_x != x || new_y != y {
//                 self.current_map.tiles[x as usize][y as usize].user = None;
//                 self.current_map.tiles[new_x as usize][new_y as usize].user =
//                     Some(self.entities[0].id);
//             }
//         }
//
//         if engine.key_pressed(KeyCode::KeyR) {
//             self.entities[0].movement = match self.entities[0].movement {
//                 Movement::Idle => entity::Movement::Walking {
//                     animation_time: Duration::from_millis(500),
//                     current_time: Duration::from_millis(0),
//                 },
//                 _ => Movement::Idle,
//             };
//         }
//
//         if engine.key_released(KeyCode::KeyZ) {
//             match engine.get_world_camera_zoom() {
//                 Zoom::None => engine.set_world_camera_zoom(Zoom::Double),
//                 Zoom::Double => engine.set_world_camera_zoom(Zoom::None),
//             }
//         }
//
//         if engine.key_released(KeyCode::KeyN) {
//             self.draw_names = !self.draw_names;
//         }
//
//         if engine.key_released(KeyCode::KeyM) {
//             self.draw_map = !self.draw_map;
//         }
//
//         if engine.key_released(KeyCode::KeyE) {
//             self.draw_entities = !self.draw_entities;
//         }
//
//         if engine.key_released(KeyCode::KeyI) {
//             self.entities[0].invisible = !self.entities[0].invisible;
//         }
//
//         if engine.key_held(KeyCode::Space) {
//             let entities_len = self.entities.len() - 1;
//             for i in 1..=2 {
//                 let id = entities_len + i;
//                 let entity = Entity::random(engine, id, &self.resources);
//
//                 let [x, y] = entity.position;
//                 self.current_map.tiles[x as usize][y as usize].user = Some(id);
//                 self.entities.push(entity);
//             }
//         }
//     }
// }
