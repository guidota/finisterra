use std::{
    collections::HashMap,
    env,
    fs::File,
    path::{self, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub id: String,
    pub frames: Vec<String>,
    pub velocity: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub file: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum LayersKey {
    Ground = 0,
    GroundDecoration = 1,
    Elements = 2,
    Roofs = 3,
}
type Layers = HashMap<LayersKey, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub name: String,
    pub tiles: HashMap<Pos, Layers>,
}
#[derive(Debug, Default)]
pub struct Assets {
    pub images: HashMap<String, Image>,
    pub animations: HashMap<String, Animation>,
    pub maps: HashMap<String, Map>,
}

/// This will give you the path to the executable (when in build mode) or to the root of the current project.
pub fn app_base_path() -> PathBuilder {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuilder {
            path_buff: path::PathBuf::from(manifest_dir),
        };
    }

    match env::current_exe() {
        Ok(path) => PathBuilder { path_buff: path },
        Err(e) => {
            log::error!(
                "Error while creating the app base_path {:?}, will use default.",
                e
            );
            PathBuilder {
                path_buff: Default::default(),
            }
        }
    }
}

/// Utils to help to build path and get them as String
pub struct PathBuilder {
    path_buff: PathBuf,
}

impl PathBuilder {
    pub fn join(mut self, path: &str) -> PathBuilder {
        self.path_buff = self.path_buff.join(path);
        self
    }

    pub fn get(self) -> String {
        self.path_buff
            .as_path()
            .to_str()
            .expect("Unable to extract the path from the path builder")
            .to_string()
    }
}
impl Assets {
    pub fn load() -> Assets {
        let path = app_base_path().join("assets/assets.yml").get();
        println!("Path: {path:?}");
        let file = File::open(path);

        match file {
            Ok(file) => {
                println!("assets file found");
                let mut images = HashMap::new();
                let mut animations = HashMap::new();
                let mut maps = HashMap::new();

                match serde_yaml::from_reader::<File, Vec<String>>(file) {
                    Ok(assets) => {
                        for asset in assets {
                            let images_path = app_base_path()
                                .join(&format!("assets/{asset}/images.yml"))
                                .get();
                            if let Ok(images_file) = File::open(images_path) {
                                println!("images file found");
                                let asset_images: Vec<Image> =
                                    serde_yaml::from_reader(images_file).unwrap_or(vec![]);
                                for image in asset_images {
                                    images.insert(image.id.clone(), image);
                                }
                            } else {
                                println!("images file not found: {asset}");
                            }
                            let animations_path = app_base_path()
                                .join(&format!("assets/{asset}/animations.yml"))
                                .get();
                            if let Ok(animations_file) = File::open(animations_path) {
                                println!("animations file found");
                                let asset_animations: Vec<Animation> =
                                    serde_yaml::from_reader(animations_file).unwrap_or(vec![]);
                                for animation in asset_animations {
                                    animations.insert(animation.id.clone(), animation);
                                }
                            }
                            let maps_path = app_base_path().join("assets/maps/").get();
                            let read_dir = std::fs::read_dir(maps_path);
                            if let Ok(read_dir) = read_dir {
                                read_dir.for_each(|file| {
                                    if let Ok(file) = file {
                                        // try to read map
                                        if let Ok(file) = File::open(file.path()) {
                                            if let Ok(map) =
                                                serde_yaml::from_reader::<File, Map>(file)
                                            {
                                                maps.insert(map.name.clone(), map);
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                    Err(_) => {
                        println!("Assets should be a list of strings!");
                    }
                };

                Assets {
                    images,
                    animations,
                    maps,
                }
            }
            Err(_) => {
                println!("Nothing to load!");
                Assets::default()
            }
        }
    }
}
