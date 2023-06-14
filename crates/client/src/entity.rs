use std::rc::Rc;

use definitions::{
    animation::Animation, body::Body, client::ClientResources, head::Head, image::Image,
};
use rand::{seq::IteratorRandom, Rng};
use roma::SmolStr;

use crate::TILE_SIZE;

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub body: Option<(Body, Animation)>,
    pub head: Option<(Head, Rc<Image>)>,
    pub name: SmolStr,
    pub position: [usize; 2],
    pub world_position: [usize; 2],
}

impl Entity {
    pub fn random(id: usize, resources: &ClientResources) -> Self {
        let mut rng = rand::thread_rng();
        let (random_body, animation) = loop {
            let (_, body) = resources.bodies.iter().choose(&mut rng).unwrap();
            let Some(animation) = resources.animations.get(&body.animations[0]) else { continue; };
            break (body.clone(), animation);
        };
        let (random_head, image) = loop {
            let (_, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.images[0] != 0 {
                let image = resources.images.get(&head.images[2]).unwrap();
                break (head.clone(), image);
            }
        };
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        Self {
            id,
            body: Some((random_body, animation.clone())),
            head: Some((random_head, image.clone())),
            position: [x, y],
            world_position: [x * TILE_SIZE, y * TILE_SIZE],
            name: SmolStr::new("Pandora"),
        }
    }
}
