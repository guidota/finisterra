use std::num::NonZeroUsize;

use lru::LruCache;
use shared::world::Map;

pub struct Maps {
    folder: String,
    cache: LruCache<u16, Map>,
}

impl Maps {
    pub fn initialize(folder: &str) -> Self {
        Self {
            folder: folder.to_string(),
            cache: LruCache::new(NonZeroUsize::new(20).unwrap()),
        }
    }

    pub fn get(&mut self, map_number: &u16) -> &mut Map {
        self.cache.get_or_insert_mut(*map_number, || {
            let path = format!("{}map_{map_number}", self.folder);
            Map::from_path(&path).expect("Map not found or invalid")
        })
    }
}
