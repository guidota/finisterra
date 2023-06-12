use std::io::Read;

use crate::{
    map::{GrhIndex, Light, MapHeader, MapInfo, MapSize, Obj, Particle, Point, Trigger, RGBA},
    parse::{ArgentumReadExt, Integer, Long},
};

// use crate::error::Error;
// use crate::map::{
//     GrhIndex, Light, Map, MapHeader, MapInfo, MapSize, Obj, Particle, Point, Trigger, RGBA,
// };
// use crate::parse::{get_binary_reader, ArgentumReadExt, Integer, Long};
// use std::io::{BufReader, Read};
//
pub trait MapsReadExt: std::io::Read {
    fn read_map_header(&mut self) -> MapHeader {
        MapHeader {
            blocks: self.read_long(),
            layers: (0..4)
                .map(|_| self.read_long())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            triggers: self.read_long(),
            lights: self.read_long(),
            particles: self.read_long(),
            npcs: self.read_long(),
            objs: self.read_long(),
            exits: self.read_long(),
        }
    }

    fn read_map_size(&mut self) -> MapSize {
        MapSize {
            x: Point {
                max: self.read_integer(),
                min: self.read_integer(),
            },
            y: Point {
                max: self.read_integer(),
                min: self.read_integer(),
            },
        }
    }

    fn read_map_info(&mut self) -> MapInfo {
        MapInfo {
            map_name: self.read_string(),
            backup_mode: self.read_bool(),
            restrict_mode: self.read_string(),
            music_number_high: self.read_long(),
            music_number_low: self.read_long(),
            secure: self.read_bool(),
            zone: self.read_string(),
            terrain: self.read_string(),
            ambient: self.read_string(),
            base_light: self.read_long(),
            letter_grh: self.read_long(),
            extra1: self.read_long(),
            extra2: self.read_long(),
            extra3: self.read_string(),
            rain: self.read_bool(),
            snow: self.read_bool(),
            fog: self.read_bool(),
        }
    }

    fn read_pos(&mut self) -> (Integer, Integer) {
        (self.read_integer(), self.read_integer())
    }

    fn read_block(&mut self) -> u8 {
        self.read_byte()
    }

    fn read_grh(&mut self) -> GrhIndex {
        self.read_long()
    }

    fn read_trigger(&mut self) -> Trigger {
        self.read_integer()
    }

    fn read_particle(&mut self) -> Particle {
        self.read_long()
    }

    fn read_color(&mut self) -> RGBA {
        RGBA {
            b: self.read_byte(),
            g: self.read_byte(),
            r: self.read_byte(),
            a: self.read_byte(),
        }
    }

    fn read_light(&mut self) -> Light {
        Light {
            color: self.read_color(),
            range: self.read_byte(),
        }
    }

    fn read_obj(&mut self) -> Obj {
        Obj {
            index: self.read_integer(),
            amount: self.read_integer(),
        }
    }
    //
    // fn iter<'a, T, F>(&'a mut self, iterations: Long, then_read: F) -> ParseIterator<'a, Self, F>
    // where
    //     Self: std::marker::Sized,
    //     F: Fn(&'a mut Self) -> T,
    // {
    //     ParseIterator {
    //         reader: self,
    //         iterations,
    //         then_read,
    //     }
    // }
}
impl<R: Read + ?Sized> MapsReadExt for R {}
//
// pub struct ParseIterator<'a, R, F> {
//     reader: &'a mut R,
//     iterations: Long,
//     then_read: F,
// }
//
// impl<'a, R: Read, T, F> Iterator for ParseIterator<'a, R, F>
// where
//     T: Sized,
//     F: Fn(&mut R) -> T,
// {
//     type Item = (Integer, Integer, T);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.iterations > 0 {
//             self.iterations -= 1;
//             let x = self.reader.read_integer();
//             let y = self.reader.read_integer();
//             let data = (self.then_read)(self.reader);
//             Some((x, y, data))
//         } else {
//             None
//         }
//     }
// }
//
// fn parse_from_reader<R: Read>(reader: &mut BufReader<R>) -> Result<Map, Error> {
//     let mut map = Map::default();
//
//     let map_header = reader.read_map_header();
//     let _ = reader.read_map_size();
//     let _ = reader.read_map_info();
//
//     for _ in 0..map_header.blocks {
//         let pos = reader.read_pos();
//         map.get(pos[0].into(), pos[1].into()).blocked = reader.read_block();
//     }
//
//     for layer in 0..4 {
//         for _ in 0..map_header.layers[layer] {
//             let pos = reader.read_pos();
//             map.tile(pos).graphics[layer] = reader.read_grh();
//             match layer {
//                 0 => {
//                     // TODO! check water and lava
//                 }
//                 1 => {
//                     // TODO! check coast
//                 }
//                 2 => {
//                     // TODO! check tree
//                 }
//                 _ => {}
//             }
//         }
//     }
//
//     for _ in 0..map_header.triggers {
//         let pos = reader.read_pos();
//         map.tile(pos).trigger = reader.read_trigger();
//     }
//
//     for _ in 0..map_header.particles {
//         let pos = reader.read_pos();
//         let particle = reader.read_particle();
//         if particle != 0 {
//             map.tile(pos).particle = Some(particle);
//         }
//     }
//
//     for _ in 0..map_header.lights {
//         let pos = reader.read_pos();
//         map.tile(pos).light = Some(reader.read_light());
//     }
//
//     for _ in 0..map_header.objs {
//         let pos = reader.read_pos();
//         let obj = reader.read_obj();
//         if obj.index != 0 {
//             map.tile(pos).obj = Some(obj);
//         }
//     }
//
//     // TODO! map_header.exits
//
//     Ok(map)
// }
//
// pub fn parse_map_from_bytes(bytes: &[u8]) -> Result<Map, Error> {
//     let mut reader = BufReader::new(bytes);
//     parse_from_reader(&mut reader)
// }
//
// pub fn parse_map(base_path: &str, map_number: usize) -> Result<Map, Error> {
//     let mut reader = get_binary_reader(&format!("{}/mapa{}.csm", base_path, map_number))?;
//     parse_from_reader(&mut reader)
// }
//
// pub fn parse_map_full_path(path: &str) -> Result<Map, Error> {
//     let mut reader = get_binary_reader(path)?;
//     parse_from_reader(&mut reader)
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_maps() {
//         use crate::ao_20::maps::parse::*;
//         use std::time::*;
//
//         let start = Instant::now();
//         for i in 1..602 {
//             let result = parse_map("../client/assets/ao_20/maps", i);
//             assert!(result.is_ok());
//         }
//         println!("parse map in {} ms", start.elapsed().as_millis());
//     }
// }
