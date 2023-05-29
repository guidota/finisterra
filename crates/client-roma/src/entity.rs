use ao::ao_20::init::Body;
use rand::{seq::IteratorRandom, Rng};
use roma::graphics::vec2::{vec2, Vec2};

use crate::resources::Resources;

pub struct Entity {
    pub body: usize,
    pub head: usize,
    pub name: String,
    pub position: Vec2,
}

impl Entity {
    pub fn random(resources: &Resources) -> Self {
        let mut rng = rand::thread_rng();
        let random_body = loop {
            let (id, body) = resources.bodies.iter().choose(&mut rng).unwrap();

            match body {
                Body::AnimatedWithTemplate {
                    template_id,
                    file_num,
                    head_offset,
                } => {
                    if let Some(template) = resources.body_templates.get(template_id) {
                        if template.width == 0 {
                            continue;
                        }
                        if file_num != &0 && head_offset.1 != 0 {
                            break *id;
                        }
                    }
                }
                Body::Animated { walks, head_offset } => {
                    let first_animation = walks.0;
                    if head_offset.1 == 0 {
                        continue;
                    }
                    if let Some(animation) = resources.animations.get(&first_animation.to_string())
                    {
                        let first_image = &animation.frames[0];
                        if resources.images.contains_key(first_image) {
                            break *id;
                        }
                    }
                }
            }
        };
        let random_head = loop {
            let (id, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.0 != 0 {
                break *id;
            }
        };
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        Self {
            body: random_body,
            head: random_head,
            position: vec2(x as f32, y as f32),
            name: "Pandora".to_string(),
        }
    }
}
