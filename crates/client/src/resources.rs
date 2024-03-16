use std::{
    collections::HashMap,
    fs::{self, File},
};

use argentum::map::Map;
use argentum::{
    animations::{Animation, ImageFrameMetadata},
    character::{
        animation::{CharacterAnimation, CharacterAnimations},
        direction::Direction,
        Armor, Body, Eyes, Face, Hair, Head, Helmet, Shield, Skin, Weapon,
    },
    image::Image,
};
use engine::engine::GameEngine;
use nohash_hasher::IntMap;

use crate::ui::textures::Textures;

#[derive(Debug, Default)]
pub struct Resources {
    pub images: Vec<Image>,
    pub animations: Vec<Animation<ImageFrameMetadata>>,

    pub bodies: Vec<Body>,
    pub skins: Vec<Skin>,
    pub hairs: Vec<Hair>,
    pub eyes: Vec<Eyes>,
    pub faces: Vec<Face>,
    pub helmets: Vec<Helmet>,
    pub shields: Vec<Shield>,
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,

    pub maps: IntMap<usize, Map>,

    pub textures: Textures,
}

impl Resources {
    pub fn load<E: GameEngine>(engine: &mut E) -> Self {
        let mut resources = Resources::default();

        resources.load_images(engine, "assets/finisterra/init/images.ron");
        resources.load_maps("assets/finisterra/maps/");
        resources.load_body(engine, "assets/finisterra/human/ao-human/");
        resources.load_head(engine, "assets/finisterra/human/ao-human/");
        resources.load_shields(engine, "assets/finisterra/shields/ao-shields/");
        resources.load_helmets(engine, "assets/finisterra/helmets/ao-helmets/");
        resources.load_weapons(engine, "assets/finisterra/weapons/ao-weapons/");
        resources.load_textures(engine);

        resources
    }

    fn load_textures<E: GameEngine>(&mut self, engine: &mut E) {
        self.textures = Textures::load(engine);
    }

    fn load_images<E: GameEngine>(&mut self, engine: &mut E, path: &str) {
        let file = File::open(path).expect("file to exist");
        let reader = std::io::BufReader::new(file);

        let images: HashMap<u32, Image> =
            ron::de::from_reader(reader).expect("images to be correct");
        let max_id = images.keys().max().expect("to be a max");

        self.images
            .resize_with((*max_id + 1) as usize, Image::default);
        for (id, image) in images {
            let file_num = image.file;
            engine.set_texture(
                &format!("assets/finisterra/images/{file_num}.png"),
                file_num,
            );
            self.images[id as usize] = image;
        }
    }

    fn load_maps(&mut self, folder: &str) {
        let maps_folder = fs::read_dir(folder).expect("maps folder not present");
        for map in maps_folder {
            let map = map.expect("should be an entry");
            let path = map.path();
            let map_path = path.to_str();
            if let Some(map_path) = map_path {
                let (_, number) = map_path.split_once('_').expect("map number");
                let number: usize = number.parse().expect("is a number");
                if let Some(map) = Map::from_path(map_path) {
                    self.maps.insert(number, map);
                }
            }
        }
    }

    fn load_body<E: GameEngine>(&mut self, engine: &mut E, folder: &str) {
        let body_ron_path = format!("{folder}body.ron");
        let file = File::open(body_ron_path).expect("body.ron not found");
        let body: Body = ron::de::from_reader(file).expect("invalid body.ron");

        let skin_ron_path = format!("{folder}skin.ron");
        let file = File::open(skin_ron_path).expect("skin.ron not found");
        let base_skin: Skin = ron::de::from_reader(file).expect("invalid skin.ron");

        // traverse skins folder
        let skins = fs::read_dir(format!("{folder}skins/")).expect("skins folder not present");
        for skin in skins {
            let skin = skin.expect("should be an entry");
            let skin_path = skin.path();
            let img = image::open(skin_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(skin_path.to_str().expect("is a file"));
            let skin = from_images(
                &mut self.images,
                file_num,
                &base_skin,
                width,
                height,
                Layout::Rows,
            );

            self.skins.push(skin);
        }

        self.bodies.push(body);
    }

    fn load_head<E: GameEngine>(&mut self, engine: &mut E, folder: &str) {
        let head_ron_path = format!("{folder}head.ron");
        let file = File::open(head_ron_path).expect("head.ron not found");
        let head: Head = ron::de::from_reader(file).expect("invalid head.ron");

        // load faces
        let faces = fs::read_dir(format!("{folder}faces/")).expect("faces folder not present");
        for file in faces {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &head,
                width,
                height,
                Layout::Columns,
            );
            self.faces.push(metadata);
        }

        // load eyes
        let eyes = fs::read_dir(format!("{folder}eyes/")).expect("eyes folder not present");
        for file in eyes {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &head,
                width,
                height,
                Layout::Columns,
            );
            self.eyes.push(metadata);
        }
        // load hairs
        let hairs = fs::read_dir(format!("{folder}hairs/")).expect("hairs folder not present");
        for file in hairs {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &head,
                width,
                height,
                Layout::Columns,
            );
            self.hairs.push(metadata);
        }
    }

    fn load_shields<E: GameEngine>(&mut self, engine: &mut E, folder: &str) {
        let file_ron_path = format!("{folder}shield.ron");
        let file = File::open(file_ron_path).expect("shield.ron not found");
        let shield: Shield = ron::de::from_reader(file).expect("invalid shield.ron");

        let shields =
            fs::read_dir(format!("{folder}shields/")).expect("shields folder not present");
        for file in shields {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &shield,
                width,
                height,
                Layout::Rows,
            );
            self.shields.push(metadata);
        }
    }

    fn load_weapons<E: GameEngine>(&mut self, engine: &mut E, folder: &str) {
        let file_ron_path = format!("{folder}weapon.ron");
        let file = File::open(file_ron_path).expect("weapon.ron not found");
        let weapon: Weapon = ron::de::from_reader(file).expect("invalid weapon.ron");

        let weapons =
            fs::read_dir(format!("{folder}weapons/")).expect("weapons folder not present");
        for file in weapons {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &weapon,
                width,
                height,
                Layout::Rows,
            );
            self.weapons.push(metadata);
        }
    }

    fn load_helmets<E: GameEngine>(&mut self, engine: &mut E, folder: &str) {
        let file_ron_path = format!("{folder}helmet.ron");
        let file = File::open(file_ron_path).expect("helmet.ron not found");
        let helmet: Weapon = ron::de::from_reader(file).expect("invalid helmet.ron");

        let helmets =
            fs::read_dir(format!("{folder}helmets/")).expect("helmets folder not present");
        for file in helmets {
            let file = file.expect("should be an entry");
            let file_path = file.path();
            let img = image::open(file_path.clone()).expect("skin file is not an image");
            let width = img.width();
            let height = img.height();

            let file_num = engine.add_texture(file_path.to_str().expect("is a file"));
            let metadata = from_images(
                &mut self.images,
                file_num,
                &helmet,
                width,
                height,
                Layout::Columns,
            );
            self.helmets.push(metadata);
        }
    }
}

enum Layout {
    Rows,
    Columns,
}

fn from_images(
    images: &mut Vec<Image>,
    file_num: u32,
    metadata: &CharacterAnimations<ImageFrameMetadata>,
    width: u32,
    height: u32,
    layout: Layout,
) -> CharacterAnimations<ImageFrameMetadata> {
    let animations = [CharacterAnimation::Idle, CharacterAnimation::Walk];
    let directions = [
        Direction::South,
        Direction::North,
        Direction::East,
        Direction::West,
    ];

    // todo: assuming 8 animations (idle, walk in each direction)
    let max_frames = {
        let mut max = 1;

        for animation in animations {
            for direction in directions {
                let frames = metadata[animation][direction].frames.len();
                if max < frames {
                    max = frames;
                }
            }
        }

        max as u32
    };
    let (frame_height, frame_width) = match layout {
        Layout::Rows => (height / 8, width / max_frames),
        Layout::Columns => (height, width / 8),
    };

    let mut metadata = metadata.clone();
    let mut i = 0;
    for animation in animations.iter() {
        for direction in directions.iter() {
            let animation = &mut metadata[*animation][*direction];
            for (f, frame) in animation.frames.iter_mut().enumerate() {
                let (x, y) = match layout {
                    Layout::Rows => (f, i),
                    Layout::Columns => (i, 0),
                };

                // build image
                let image_id = images.len() as u32;
                let image = Image {
                    id: image_id,
                    file: file_num,
                    width: frame_width as u16,
                    height: frame_height as u16,
                    y: (y * frame_height as usize) as u16,
                    x: (x as u32 * frame_width) as u16,
                };
                // insert image
                images.push(image);

                // update skin
                frame.image = image_id;
            }
            i += 1;
        }
    }

    metadata
}
