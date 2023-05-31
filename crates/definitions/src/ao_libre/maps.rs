use byteorder::ReadBytesExt;
use std::io::BufRead;

use crate::{
    map::{Map, MapInfo, Obj, Tile, WorldPosition},
    parse::*,
};

impl<R: std::io::Read + ?Sized> ArgentumReadExt for R {}
// #[derive(Debug, Clone, Copy)]
// pub struct Pos {
//     pub map: u16,
//     pub x: u16,
//     pub y: u16,
// }
//
// #[derive(Debug, Clone, Copy)]
// pub struct ObjInfo {
//     pub obj_index: u16,
//     pub amount: u16,
// }
//
// #[derive(Default, Debug, Clone, Copy)]
// pub struct Tile {
//     pub grh: [u16; 4],
//     pub char_index: Option<u16>,
//     pub npc_index: Option<u16>,
//     pub obj_info: Option<ObjInfo>,
//     pub blocked: bool,
//     pub trigger: u16,
//     pub tile_exit: Option<Pos>,
// }
//
// pub type Tiles = [[Tile; 100]; 100];
//
// #[derive(Debug, Clone)]
// pub struct Map {
//     pub width: u16,
//     pub height: u16,
//     pub tiles: Tiles,
// }
//
// #[derive(Debug, Clone)]
// pub struct MapData {
//     pub music_num: (u16, u16),
//     pub start_pos: Pos,
//     pub name: String,
//     pub pk: bool,
//     pub terrain: String,
//     pub zone: String,
//     pub restrict: bool,
//     pub backup: bool,
// }
//
// impl MapData {
//     fn new() -> Self {
//         MapData {
//             music_num: (0, 0),
//             start_pos: Pos { map: 0, x: 0, y: 0 },
//             name: "".to_string(),
//             pk: false,
//             terrain: "".to_string(),
//             zone: "".to_string(),
//             restrict: false,
//             backup: false,
//         }
//     }
// }
//
pub trait MapsReadExt: std::io::Read {
    fn read_obj_info(&mut self) -> Obj {
        Obj {
            index: self.read_integer(),
            amount: self.read_integer(),
        }
    }
    fn read_pos(&mut self) -> WorldPosition {
        WorldPosition {
            map: self.read_integer(),
            x: self.read_integer(),
            y: self.read_integer(),
        }
    }

    fn read_tile(&mut self) -> Tile {
        let mut tile = Tile {
            blocked: self.read_u8().unwrap(), // warn: this means blocking entire tile, but
            // blocking has direcitons
            ..Default::default()
        };
        tile.graphics = [
            self.read_integer().into(),
            self.read_integer().into(),
            self.read_integer().into(),
            self.read_integer().into(),
        ];
        tile.trigger = self.read_integer();
        self.read_integer();
        tile
    }

    fn fill_tile_inf(&mut self, tile: &mut Tile) {
        let exit_pos = self.read_pos();
        if exit_pos.map != 0 {
            tile.exit = Some(exit_pos);
        }

        let npc = self.read_integer();
        if npc != 0 {
            tile.npc = Some(npc);
        }
        let obj_info = self.read_obj_info();
        if obj_info.index != 0 {
            tile.obj = Some(obj_info);
        }
        self.read_integer();
        self.read_integer();
    }
}

impl<R: std::io::Read + ?Sized> MapsReadExt for R {}

fn get_reader(file: String) -> Option<BufReader<File>> {
    let file = File::open(file);
    match file {
        Ok(file) => {
            let buffer = BufReader::new(file);
            Some(buffer)
        }
        Err(_) => None,
    }
}

pub fn parse_map_dat(base_path: &str, map_number: usize) -> Result<MapInfo, String> {
    let reader = get_reader(format!("{}/Mapa{}.dat", base_path, map_number))
        .ok_or_else(|| format!("Could not open map{}.dat", map_number))?;
    let mut map_data = MapInfo::default();
    for line in reader.lines() {
        let line = line.expect("asdf");
        let parts: Vec<&str> = line.split('=').collect();
        let key = parts[0];
        match key {
            "MusicNum" => {
                let value = parts[1].to_string();
                let numbers: Vec<&str> = value.split('-').collect();
                map_data.music_number_low = numbers[0].parse::<u16>().unwrap().into();
                map_data.music_number_high = numbers[1].parse::<u16>().unwrap().into();
            }
            "StartPos" => {
                // let value = parts[1].to_string();
                // let numbers: Vec<&str> = value.split('-').collect();
                // map_data.start_pos = Pos {
                //     map: numbers[0].parse::<u16>().unwrap(),
                //     x: numbers[1].parse::<u16>().unwrap(),
                //     y: numbers[2].parse::<u16>().unwrap(),
                // };
            }
            "Name" => {
                map_data.map_name = parts[1].to_string().clone();
            }
            "PK" => {
                map_data.secure = parts[1] == "1";
            }
            "Terreno" => {
                map_data.terrain = parts[1].to_string().clone();
            }
            "Zona" => {
                map_data.zone = parts[1].to_string().clone();
            }
            "Restringir" => {
                map_data.restrict_mode = parts[1].to_string();
            }
            "BackUp" => {
                map_data.backup_mode = parts[1] == "1";
            }
            _ => {}
        }
    }

    Ok(map_data)
}

pub fn parse_map(base_path: &str, map_number: usize) -> Result<Map, String> {
    let mut reader = get_reader(format!("{}/Mapa{}.map", base_path, map_number))
        .ok_or_else(|| format!("Mapa{}.map not found", map_number))?;
    let mut inf_reader = get_reader(format!("{}/Mapa{}.inf", base_path, map_number));

    let mut map = Map::default();

    // map header
    dbg!("map version {}", reader.read_integer());
    reader.skip_header();
    reader.skip_temp_ints(4);

    // inf header
    if let Some(ref mut r) = inf_reader {
        r.skip_temp_ints(5);
    }

    for x in 1..=100 {
        for y in 1..=100 {
            let mut tile = reader.read_tile();
            if let Some(ref mut r) = inf_reader {
                r.fill_tile_inf(&mut tile);
            }
            map.tiles[(x - 1) as usize][(y - 1) as usize] = tile;
        }
    }

    Ok(map)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_maps() {
//         use crate::ao_libre::maps::*;
//         use std::time::*;

//         let start = Instant::now();
//         let result = parse_map("assets/maps", 1);
//         assert!(result.is_ok());
//         println!("parse map in {} ms", start.elapsed().as_millis());
//     }
// }
