use definitions::client::ClientResources;
use rand::{seq::IteratorRandom, Rng};
use roma::SmolStr;

use crate::TILE_SIZE;

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub body: usize,
    pub head: usize,
    pub name: SmolStr,
    pub position: [usize; 2],
    pub world_position: [usize; 2],
}

impl Entity {
    pub fn random(id: usize, resources: &ClientResources) -> Self {
        let mut rng = rand::thread_rng();
        let (random_body, _) = resources.bodies.iter().choose(&mut rng).unwrap();
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
            name: SmolStr::new("Pandora"),
        }
    }
}
