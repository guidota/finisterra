fn main() {
    // maps_to_layer_atlases();
    // map_to_atlas(1);
    // map_layer_0_batches_required(1);
}
//
// fn map_layer_0_batches_required(map_number: usize) {
//     let graphics = ao::ao_20::graphics::parse::parse_graphics("./assets/init/graficos.ind")
//         .expect("can parse graphics");
//     let path = format!("./assets/maps/mapa{map_number}.csm");
//     let map_file_path = path.as_str();
//     let map = parse_map_full_path(map_file_path).expect("can] parse map");
// }
//
// fn map_to_atlas(map_number: usize) {
//     let graphics = ao::ao_20::graphics::parse::parse_graphics("./assets/init/graficos.ind")
//         .expect("can parse graphics");
//     let map_images = images_used_by_map(graphics, 1).unwrap();
//
//     let folder_path = format!("./assets/map_{map_number}_atlas");
//     _ = std::fs::create_dir(&folder_path);
//
//     for i in 0..4 {
//         for file in &map_images.layers[i] {
//             let src = format!("./assets/graphics/{file}");
//             let dst = format!("{folder_path}/{file}");
//             println!("Copying {} to {}", &src, &dst);
//
//             _ = std::fs::copy(&src, &dst);
//         }
//     }
// }
//
// fn maps_to_layer_atlases() {
//     let graphics = ao::ao_20::graphics::parse::parse_graphics("./assets/init/graficos.ind")
//         .expect("can parse graphics");
//
//     let map_images = images_used_by_maps(graphics).unwrap();
//     println!("Amount of files used by layer:");
//     println!(" > layer 0: {}", map_images.layers[0].len());
//     println!(" > layer 1: {}", map_images.layers[1].len());
//     println!(" > layer 2: {}", map_images.layers[2].len());
//     println!(" > layer 3: {}", map_images.layers[3].len());
//     // println!("{}", toml::to_string_pretty(&map_images).unwrap());
//
//     // move images to separated folders
//     for i in 0..4 {
//         let folder_path = format!("./assets/map_graphics_{}", i);
//         _ = std::fs::create_dir(&folder_path);
//
//         for file in &map_images.layers[i] {
//             let src = format!("./assets/graphics/{file}");
//             let dst = format!("{folder_path}/{file}");
//             println!("Copying {} to {}", &src, &dst);
//
//             _ = std::fs::copy(&src, &dst);
//         }
//     }
// }
