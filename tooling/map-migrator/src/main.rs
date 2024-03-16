use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use argentum::ao_20;
use argentum::error::Error;
use argentum::image::Image;

pub const CONFIG: bincode::config::Configuration = bincode::config::standard();

fn main() {
    let result = migrate_ao20_maps(
        "assets/ao_20/maps/",
        "assets/ao_20/graphics/",
        "assets/ao_20/init/graficos.ind",
    );
    match result {
        Ok(_) => println!("migration done!"),
        Err(error) => println!("couldn't migrate ao20 maps: {error:?}"),
    }
}

/// Read AO 20 map and write new format. Also move used images to an output folder.
/// Write images/animations list metadata file.
fn migrate_ao20_maps(
    map_folder: &str,
    graphics_folder: &str,
    graphics_metadata_file: &str,
) -> Result<(), Error> {
    let maps = ao_20::maps::load_maps(map_folder)?;
    let graphics = ao_20::load_graphics(graphics_metadata_file)?;

    let _ = std::fs::create_dir("output/");
    let _ = std::fs::create_dir("output/maps/");
    let _ = std::fs::create_dir("output/images/");
    let _ = std::fs::create_dir("output/init/");

    let mut images = HashMap::new();

    for (id, map) in maps {
        let path = format!("output/maps/map_{id}");

        println!("writing map {id} to {path}");
        let mut file = File::create(path).map_err(|_| Error::FileNotFound)?;
        bincode::encode_into_std_write(&map, &mut file, CONFIG).map_err(|_| Error::Parse)?;

        for row in map.tiles {
            for tile in row {
                for graphic in tile.graphics {
                    if let Some(image) = graphics.images.get(&(graphic as u32)) {
                        images.insert(image.id, image.as_ref().clone());
                        find_and_move_image(image, graphics_folder, "output/images/");
                    } else if let Some(animation) = graphics.animations.get(&graphic) {
                        for frame in &animation.frames {
                            if let Some(image) = graphics.images.get(frame) {
                                images.insert(image.id, image.as_ref().clone());
                                find_and_move_image(image, graphics_folder, "output/images/");
                            }
                        }
                    }
                }
            }
        }
    }

    let file = File::create("output/init/images.ron").map_err(|_| Error::FileNotFound)?;
    let config = ron::ser::PrettyConfig::default();
    let _ = ron::ser::to_writer_pretty(file, &images, config);

    Ok(())
}

fn find_and_move_image(image: &Image, graphics_folder: &str, output: &str) {
    let file_num = image.file;

    let to = format!("{output}{file_num}.png");

    if Path::new(&to).exists() {
        return;
    }

    println!("copying image: {file_num}...");

    let from = format!("{graphics_folder}{file_num}.png");

    if std::fs::copy(from, to).is_err() {
        println!("copy failed!");
    }
}
