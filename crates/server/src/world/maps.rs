use nohash_hasher::IntMap;
use shared::world::Map;

pub fn load_maps(folder: &str) -> IntMap<u16, Map> {
    let mut maps = IntMap::default();
    let maps_folder = std::fs::read_dir(folder).expect("maps folder not present");
    for map in maps_folder {
        let map = map.expect("should be an entry");
        let path = map.path();
        let map_path = path.to_str();
        if let Some(map_path) = map_path {
            let (_, number) = map_path.split_once('_').expect("map number");
            let number: u16 = number.parse().expect("is a number");
            if let Some(map) = Map::from_path(map_path) {
                maps.insert(number, map);
            }
        }
    }
    maps
}
