use std::{collections::VecDeque, time::Duration};

use bincode::{Decode, Encode};

use crate::world::{Direction, WorldPosition};

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Prediction {
    pub key_id: u8,
    pub counter: u8,
    pub result: PredictionResult,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct KeyPressed {
    pub key_id: u8,
    pub direction: Direction,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct KeyReleased {
    pub direction: Direction,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum PredictionResult {
    Move(Direction),
    Stay,
}

pub struct Movement {
    pub input: VecDeque<KeyPressed>,
    pub predictions: Vec<Prediction>,

    pub velocity: f64,
    pub moving: Option<(u8, u8, Direction)>,

    pub position: WorldPosition,
    pub moving_position: (f64, f64),
}

impl Movement {
    pub fn key_pressed(&mut self, key_pressed: KeyPressed) {
        if self
            .input
            .iter()
            .any(|key| key.key_id == key_pressed.key_id || key.direction == key_pressed.direction)
        {
            // should not happen
        } else {
            self.input.push_front(key_pressed);
        }
    }

    pub fn key_released(&mut self, key_released: KeyReleased) {
        self.input
            .retain(|key| key.direction != key_released.direction);
    }

    fn update_moving(&mut self) {
        if self.moving.is_none() {
            if let Some(key) = self.input.front() {
                self.moving = Some((key.key_id, 0, key.direction.clone()));
            }
        }
    }

    /// TODO: consider map state (blocks, other entities, etc)
    pub fn update(&mut self, delta: Duration) -> bool {
        self.update_moving();

        if let Some((key_id, ref mut prediction_counter, direction)) = self.moving.as_mut() {
            let distance = delta.as_secs_f64() * self.velocity;
            match direction {
                Direction::South => self.moving_position.1 -= distance,
                Direction::North => self.moving_position.1 += distance,
                Direction::East => self.moving_position.0 += distance,
                Direction::West => self.moving_position.0 -= distance,
            }
            if arrived(self.moving_position, direction) {
                match direction {
                    Direction::South => self.moving_position.1 += 1.,
                    Direction::North => self.moving_position.1 -= 1.,
                    Direction::East => self.moving_position.0 -= 1.,
                    Direction::West => self.moving_position.0 += 1.,
                }
                let prediction = Prediction {
                    key_id: *key_id,
                    counter: *prediction_counter,
                    result: PredictionResult::Move(direction.clone()),
                };
                println!(
                    "prediction {prediction:?} - moving position {:?}",
                    self.moving_position
                );
                *prediction_counter += 1;
                self.predictions.push(prediction);
                if let Some(key) = self.input.front() {
                    let prediction_counter = if key_id == &key.key_id {
                        *prediction_counter
                    } else {
                        0
                    };
                    self.moving = Some((key.key_id, prediction_counter, key.direction.clone()));
                } else {
                    self.moving = None;
                    self.moving_position = (0.0, 0.0);
                }
                return true;
            }
        }
        false
    }

    pub fn validate(&mut self, server_prediction: Prediction) {
        for i in 0..self.predictions.len() {
            let client_prediction = &mut self.predictions[i];

            if client_prediction.key_id == server_prediction.key_id
                && client_prediction.counter == server_prediction.counter
            {
                match (&client_prediction.result, &server_prediction.result) {
                    (PredictionResult::Move(direction), PredictionResult::Stay) => {
                        match direction {
                            Direction::North => {
                                self.position.y -= 1;
                            }
                            Direction::East => {
                                self.position.x -= 1;
                            }
                            Direction::South => {
                                self.position.y += 1;
                            }
                            Direction::West => {
                                self.position.x += 1;
                            }
                        }
                    }
                    (PredictionResult::Stay, PredictionResult::Move(direction)) => {
                        match direction {
                            Direction::North => {
                                self.position.y += 1;
                            }
                            Direction::East => {
                                self.position.x += 1;
                            }
                            Direction::South => {
                                self.position.y -= 1;
                            }
                            Direction::West => {
                                self.position.x -= 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
            if client_prediction.key_id == server_prediction.key_id
                && client_prediction.counter <= server_prediction.counter
            {
                self.predictions.remove(i);
            }
        }
    }

    pub fn moving_direction(&self) -> Option<Direction> {
        self.moving.as_ref().map(|i| i.2.clone())
    }

    pub fn tile_position(&self) -> WorldPosition {
        let mut x = self.position.x;
        let mut y = self.position.y;
        for prediction in &self.predictions {
            if let PredictionResult::Move(direction) = &prediction.result {
                match direction {
                    Direction::North => y += 1,
                    Direction::East => x += 1,
                    Direction::South => y -= 1,
                    Direction::West => x -= 1,
                }
            }
        }

        WorldPosition {
            map: self.position.map,
            x,
            y,
        }
    }

    pub fn world_position(&self) -> (f64, f64) {
        let mut x = self.position.x as f64;
        let mut y = self.position.y as f64;
        for prediction in &self.predictions {
            if let PredictionResult::Move(direction) = &prediction.result {
                match direction {
                    Direction::North => y += 1.,
                    Direction::East => x += 1.,
                    Direction::South => y -= 1.,
                    Direction::West => x -= 1.,
                }
            }
        }

        (x + self.moving_position.0, y + self.moving_position.1)
    }
}

fn arrived(moving_position: (f64, f64), direction: &Direction) -> bool {
    let x = moving_position.0;
    let y = moving_position.1;

    match direction {
        Direction::South => y <= -1.0,
        Direction::North => y >= 1.0,
        Direction::East => x >= 1.0,
        Direction::West => x <= -1.0,
    }
}

/// TODO: consider map state (blocks, other entities, etc)
fn next_position(position: &WorldPosition, direction: &Direction) -> WorldPosition {
    match direction {
        Direction::South => WorldPosition {
            map: position.map,
            x: position.x,
            y: position.y - 1,
        },
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
        Direction::West => WorldPosition {
            map: position.map,
            x: position.x - 1,
            y: position.y,
        },
    }
}
