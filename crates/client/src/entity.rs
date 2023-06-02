use definitions::client::ClientResources;
use rand::{seq::IteratorRandom, Rng};

use crate::TILE_SIZE;

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub body: usize,
    pub head: usize,
    pub name: String,
    pub position: [usize; 2],
    pub world_position: [usize; 2],
}

impl Entity {
    pub fn random(id: usize, resources: &ClientResources) -> Self {
        let mut rng = rand::thread_rng();
        let (random_body, _) = resources.bodies.iter().choose(&mut rng).unwrap();
        //
        // match body {
        //     Body::AnimatedWithTemplate {
        //         template_id,
        //         file_num,
        //         head_offset,
        //     } => {
        //         if let Some(template) = resources.body_templates.get(template_id) {
        //             if template.width == 0 {
        //                 continue;
        //             }
        //             if file_num != &0 && head_offset.1 != 0 {
        //                 break *id;
        //             }
        //         }
        //     }
        //     Body::Animated { walks, head_offset } => {
        //         let first_animation = walks.0;
        //         if head_offset.1 == 0 {
        //             continue;
        //         }
        //         if let Some(animation) = resources.animations.get(&first_animation.to_string())
        //         {
        //             let first_image = &animation.frames[0];
        //             if resources.images.contains_key(first_image) {
        //                 break *id;
        //             }
        //         }
        //     }
        // }
        let random_head = loop {
            let (id, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.images[0] != 0 {
                break *id;
            }
        };
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        Self {
            id,
            body: *random_body,
            head: random_head,
            position: [x, y],
            world_position: [x * TILE_SIZE, y * TILE_SIZE],
            name: "Pandora".to_string(),
        }
    }
}
