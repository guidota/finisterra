use rustc_hash::FxHashMap;

use crate::{
    error::Error,
    map::{Map, Tile},
    parse::get_binary_reader,
};

use self::parse::MapsReadExt;

pub mod parse;

/// maps in csm format
pub fn load_maps(path: &str) -> Result<FxHashMap<usize, Map>, Error> {
    let mut maps = FxHashMap::default();

    let dir = std::fs::read_dir(path).map_err(|_| Error::FileNotFound)?;
    for entry in dir {
        let file = entry.map_err(|_| Error::FileNotFound)?;
        let path = file.path();
        let path = path.to_str().ok_or(Error::Parse)?;

        let path_str = path.to_string();
        let file_name = path_str.split('/').last().unwrap();
        let map_file_num = file_name.replace("mapa", "").replace(".csm", "");
        let Ok(id) = map_file_num.parse::<usize>() else {
            continue;
        };
        let mut reader = get_binary_reader(path)?;

        let mut map = Map {
            tiles: vec![vec![Tile::default(); 100]; 100],
        };
        // Load map
        let map_header = reader.read_map_header();
        let _ = reader.read_map_size();
        let _ = reader.read_map_info();

        for _ in 0..map_header.blocks {
            let pos = reader.read_pos();
            map.tiles[pos.0 as usize - 1][pos.1 as usize - 1].blocked = reader.read_block();
        }

        for layer in 0..4 {
            for _ in 0..map_header.layers[layer] {
                let pos = reader.read_pos();
                map.tile(pos).graphics[layer] = reader.read_grh() as usize;
                match layer {
                    0 => {
                        // TODO! check water and lava
                    }
                    1 => {
                        // TODO! check coast
                    }
                    2 => {
                        // TODO! check tree
                    }
                    _ => {}
                }
            }
        }

        for _ in 0..map_header.triggers {
            let pos = reader.read_pos();
            map.tile(pos).trigger = reader.read_trigger();
        }

        for _ in 0..map_header.particles {
            let pos = reader.read_pos();
            let particle = reader.read_particle();
            if particle != 0 {
                map.tile(pos).particle = Some(particle);
            }
        }

        for _ in 0..map_header.lights {
            let pos = reader.read_pos();
            map.tile(pos).light = Some(reader.read_light());
        }

        for _ in 0..map_header.objs {
            let pos = reader.read_pos();
            let obj = reader.read_obj();
            if obj.index != 0 {
                map.tile(pos).obj = Some(obj);
            }
        }

        // TODO! map_header.exits
        for tiles in map.tiles.iter_mut() {
            tiles.reverse();
        }

        maps.insert(id, map);
    }

    Ok(maps)
}
