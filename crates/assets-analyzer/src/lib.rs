// use std::collections::{BTreeMap, HashSet};
// use std::{fs, io};
//
// use definitions::animation::Animation;
// use definitions::image::Image;
// use serde::{Deserialize, Serialize};
//
// #[derive(Serialize, Deserialize, Default)]
// pub struct MapImages {
//     pub layers: [HashSet<String>; 4],
// }
//
// type Graphics = (BTreeMap<String, Image>, BTreeMap<String, Animation>);
//
// pub fn images_used_by_map(graphics: Graphics, map_number: usize) -> io::Result<MapImages> {
//     let mut map_images = MapImages::default();
//     let path = format!("./assets/maps/mapa{map_number}.csm");
//     let map_file_path = path.as_str();
//     print!("Analyzing map: {map_file_path}...");
//     let map = parse_map_full_path(map_file_path).expect("can] parse map");
//     for tiles in map.tiles {
//         for tile in tiles {
//             for i in 0..4 {
//                 if tile.graphics[i] > 0 {
//                     if let Some(file) = graphics.get_image_file(&tile.graphics[i].to_string()) {
//                         map_images.layers[i].insert(file);
//                     }
//                 }
//             }
//         }
//     }
//     println!("Done");
//
//     Ok(map_images)
// }
//
// pub fn images_used_by_maps(graphics: Graphics) -> io::Result<MapImages> {
//     let mut map_images = MapImages::default();
//     for map_file in fs::read_dir("./assets/maps/")? {
//         let path = &map_file?.path();
//         let map_file_path = path.to_str().unwrap();
//         print!("Analyzing map: {map_file_path}...");
//         let Ok(map) = parse_map_full_path(map_file_path) else {
//             println!("Skip");
//             continue;
//         };
//         for tiles in map.tiles {
//             for tile in tiles {
//                 for i in 0..4 {
//                     if tile.graphics[i] > 0 {
//                         if let Some(file) = graphics.get_image_file(&tile.graphics[i].to_string()) {
//                             map_images.layers[i].insert(file);
//                         }
//                     }
//                 }
//             }
//         }
//         println!("Done");
//     }
//     Ok(map_images)
// }
//
// trait GetImageFile {
//     fn get_image_file(&self, id: &str) -> Option<String>;
// }
//
// impl GetImageFile for Graphics {
//     fn get_image_file(&self, id: &str) -> Option<String> {
//         self.0
//             .get(id)
//             .map(|image| format!("{}.png", image.file_num))
//             .or_else(|| {
//                 self.1.get(id).and_then(|animation| {
//                     let frame_0 = &animation.frames[0];
//                     self.0
//                         .get(frame_0.as_str())
//                         .map(|image| format!("{}.png", image.file_num))
//                 })
//             })
//     }
// }
