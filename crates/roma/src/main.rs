use std::{collections::HashMap, ops::Add, time::Duration};

use roma::{
    add_font, draw_image, draw_text, get_delta, get_input, set_camera_zoom, DrawImageParams,
    VirtualKeyCode,
};
use smol_str::SmolStr;

fn main() {
    let settings = roma::Settings {
        // textures_folder: "art/".to_string(),
        ..Default::default()
    };
    roma::run_game(
        settings,
        || {
            add_font(
                "tahoma",
                include_bytes!("ui/fonts/tahoma_bold.ttf").as_slice(),
            );

            let mut idle = HashMap::default();
            let mut walk = HashMap::default();

            let mut y_offset = 0;
            idle.insert(Direction::East, Animation::from_offset(y_offset));
            y_offset += 64;
            idle.insert(Direction::Weast, Animation::from_offset(y_offset));
            y_offset += 64;
            idle.insert(Direction::North, Animation::from_offset(y_offset));
            y_offset += 64;
            idle.insert(Direction::South, Animation::from_offset(y_offset));

            y_offset += 64;
            walk.insert(Direction::South, Animation::from_offset(y_offset));
            y_offset += 64;
            walk.insert(Direction::North, Animation::from_offset(y_offset));
            y_offset += 64;
            walk.insert(Direction::East, Animation::from_offset(y_offset));
            y_offset += 64;
            walk.insert(Direction::Weast, Animation::from_offset(y_offset));

            State {
                idle,
                walk,
                direction: Direction::South,
                walking: false,
                time: Duration::from_millis(0),
                transition: None,
                position: [0., 0.],
            }
        },
        |state| {
            // set_camera_position(400., 300.);
            set_camera_zoom(roma::Zoom::Double);

            if get_input().key_released(VirtualKeyCode::Space) {
                state.walking = !state.walking;
            }
            let transition = Duration::from_millis(150);
            if get_input().key_pressed(VirtualKeyCode::Left) {
                state.transition = Some((
                    state.direction,
                    Direction::Weast,
                    Duration::from_millis(0),
                    transition,
                ));
            } else if get_input().key_pressed(VirtualKeyCode::Right) {
                state.transition = Some((
                    state.direction,
                    Direction::East,
                    Duration::from_millis(0),
                    transition,
                ));
            } else if get_input().key_pressed(VirtualKeyCode::Down) {
                state.transition = Some((
                    state.direction,
                    Direction::South,
                    Duration::from_millis(0),
                    transition,
                ));
            } else if get_input().key_pressed(VirtualKeyCode::Up) {
                state.transition = Some((
                    state.direction,
                    Direction::North,
                    Duration::from_millis(0),
                    transition,
                ));
            }

            let delta = get_delta();

            if state.walking {
                let moving_to = match state.transition {
                    Some((_, to, _, _)) => to,
                    _ => state.direction,
                };
                let distance = 32. / 250. * delta.as_nanos() as f32 / 1000000.;
                match moving_to {
                    Direction::East => {
                        state.position[0] += distance;
                    }
                    Direction::Weast => {
                        state.position[0] -= distance;
                    }
                    Direction::North => {
                        state.position[1] += distance;
                    }
                    Direction::South => {
                        state.position[1] -= distance;
                    }
                }
            }

            let change = if let Some((_, _, ref mut timer, duration)) = state.transition.as_mut() {
                *timer = timer.add(delta);
                timer.ge(&duration)
            } else {
                false
            };

            if change {
                state.direction = state.transition.unwrap().1;
                state.transition = None;
                state.time = Duration::from_millis(0);
            }

            state.direction = match state.transition {
                Some((Direction::North, Direction::South, _, _)) => Direction::Weast,
                Some((Direction::South, Direction::North, _, _)) => Direction::East,
                Some((Direction::East, Direction::Weast, _, _)) => Direction::North,
                Some((Direction::Weast, Direction::East, _, _)) => Direction::South,
                Some((_, to, _, _)) => to,
                _ => state.direction,
            };

            let animation = if state.walking {
                &state.walk[&state.direction]
            } else {
                &state.idle[&state.direction]
            };

            state.time = state.time.add(delta);

            if state.time > animation.duration {
                state.time = Duration::from_millis(0);
            }
            // draw_animation(2, animation, state.time, [0., 0., 0.5]);
            // draw_animation(6, animation, state.time, [32., 0., 0.5]);
            // draw_animation(3, animation, state.time, [64., 0., 0.5]);
            // draw_animation(4, animation, state.time, [96., 0., 0.5]);
            draw_animation(
                5,
                animation,
                state.time,
                [state.position[0], state.position[1], 0.5],
            );
            let mut another_animation = animation.clone();
            another_animation.frame_width = 64;
            let weapon_z = match state.direction {
                Direction::East => 0.6,
                Direction::Weast => 0.4,
                Direction::North => 0.4,
                Direction::South => 0.6,
            };
            draw_animation(
                7,
                &another_animation,
                state.time,
                [state.position[0], state.position[1], weapon_z],
            );

            draw_image(
                1,
                DrawImageParams {
                    ..Default::default()
                },
            );

            draw_text(roma::DrawTextParams {
                text: SmolStr::new("Pandora"),
                color: [255, 0, 0, 255],
                position: [state.position[0], state.position[1], 0.6],
            });
        },
        |_, _| {},
    );
}

fn draw_animation(texture_id: u64, animation: &Animation, time: Duration, mut position: [f32; 3]) {
    let frame_duration = animation.duration.as_millis() / animation.frames as u128;
    let animation_time = time.as_millis();

    let index = (animation_time / frame_duration) as usize % animation.frames;

    let x = index * animation.frame_width;
    let y = animation.y_offset;

    position[0] -= animation.frame_width as f32 / 2.;

    draw_image(
        texture_id,
        DrawImageParams::new(
            &position,
            [255, 255, 255, 255],
            [x as u16, y as u16, animation.frame_width as u16, 64],
        ),
    );
}

#[derive(Default, Clone)]
struct Animation {
    frames: usize,
    frame_width: usize,
    y_offset: usize,
    duration: Duration,
}

impl Animation {
    fn from_offset(y: usize) -> Animation {
        Animation {
            frames: 8,
            frame_width: 32,
            y_offset: y,
            duration: Duration::from_millis(250),
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    East = 0,
    Weast = 1,
    North = 2,
    South = 3,
}

struct State {
    idle: HashMap<Direction, Animation>,
    walk: HashMap<Direction, Animation>,

    transition: Option<(Direction, Direction, Duration, Duration)>,

    direction: Direction,
    walking: bool,
    time: Duration,

    position: [f32; 2],
}
