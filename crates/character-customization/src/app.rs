use std::{
    fmt::Display,
    fs::{self, File},
    sync::mpsc::Sender,
    time::Duration,
};

use lorenzo::{
    animations::{Animation, ImageFrameMetadata},
    character::{
        animation::{CharacterAnimation, CharacterAnimations},
        animator::Animator,
        direction::Direction,
        Armor, Body, Character, Eyes, Face, Hair, Head, Helmet, Shield, Skin, Weapon,
    },
    image::Image,
};
use rand::seq::SliceRandom;
use roma::{
    add_ui_texture, draw_image, draw_text, get_input, register_texture, set_camera_position,
    set_camera_size, set_camera_zoom,
    ui::{self, ManagedTextureId},
    DrawImageParams, DrawTextParams, SmolStr,
};

#[derive(Default, Debug)]
pub struct App {
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

    pub character: Character,
}

impl App {
    pub fn load(sender: Sender<LoadingStage>) {
        let mut app = App::default();
        // let resources = "crates/character-customization/resources";
        let bytes = include_bytes!("../../../art/dv.png");
        let filter = ui::paint::TextureFilter::Nearest;
        let background = add_ui_texture(bytes, filter);
        let _ = sender.send(LoadingStage::Background(background));

        let _ = sender.send(LoadingStage::LoadingBodies);
        app.load_body("art/human/ao-human/");
        let _ = sender.send(LoadingStage::LoadingHeads);
        app.load_head("art/human/ao-human/");
        let _ = sender.send(LoadingStage::LoadingHelmets);
        app.load_shields("art/shields/ao-shields/");
        let _ = sender.send(LoadingStage::LoadingShields);
        app.load_helmets("art/helmets/ao-helmets/");
        let _ = sender.send(LoadingStage::LoadingWeapons);
        app.load_weapons("art/weapons/ao-weapons/");
        // let _ = sender.send(LoadingStage::LoadingArmors);
        let _ = sender.send(LoadingStage::PreparingCharacter);

        app.randomize_character();

        app.character.animator = Animator {
            duration: Duration::from_millis(400),
            ..Default::default()
        };

        let app = Box::new(app);
        let _ = sender.send(LoadingStage::Finish(app));
    }

    fn load_body(&mut self, folder: &str) {
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

            let file_num = register_texture(skin_path);
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

    fn load_head(&mut self, folder: &str) {
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

            let file_num = register_texture(file_path);
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

            let file_num = register_texture(file_path);
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

            let file_num = register_texture(file_path);
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

    fn load_shields(&mut self, folder: &str) {
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

            let file_num = register_texture(file_path);
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

    fn load_weapons(&mut self, folder: &str) {
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

            let file_num = register_texture(file_path);
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

    fn load_helmets(&mut self, folder: &str) {
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

            let file_num = register_texture(file_path);
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

    pub fn update(&mut self, dt: Duration) {
        if get_input().key_released(roma::VirtualKeyCode::Space) {
            self.randomize_character();
        }
        self.character.update_animation(dt);
    }

    pub fn draw(&self, size: (usize, usize)) {
        let x = (size.0 / 2) as u16;
        let y = (size.1 / 2) as u16;
        set_camera_size(size.0 as f32, size.1 as f32);
        set_camera_position(x as f32, y as f32);
        set_camera_zoom(roma::Zoom::Double);

        let body = self.character.get_body_frame();
        draw_text(DrawTextParams {
            text: SmolStr::new("guidota"),
            position: [x as f32, y as f32 - 14., 0.5],
            color: [255, 0, 0, 255],
        });

        draw_text(DrawTextParams {
            text: SmolStr::new("Rahma Nanarak O'al"),
            position: [x as f32, y as f32 + 56., 0.5],
            color: [0, 255, 255, 255],
        });

        let skin = self.character.get_skin_frame();
        let image = &self.images[skin.image as usize];

        let x = x - body.base.x as u16;
        let y = y - body.base.y as u16;

        let color = [255, 255, 255, 255];

        draw_image(
            image.file,
            DrawImageParams {
                x,
                y,
                source: [image.x, image.y, image.width, image.height],
                color,
                z: 0.5 + (skin.priority as f32 * 0.0001),
            },
        );

        if let Some(metadata) = self.character.get_face_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.head.x as u16 - metadata.offset.x as u16,
                    y: y + body.head.y as u16 - metadata.offset.y as u16,
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }

        if let Some(metadata) = self.character.get_eyes_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.head.x as u16 - metadata.offset.x as u16,
                    y: y + body.head.y as u16 - metadata.offset.y as u16,
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }

        if let Some(metadata) = self.character.get_hair_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.head.x as u16 - metadata.offset.x as u16,
                    y: y + body.head.y as u16 - metadata.offset.y as u16,

                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }

        if let Some(metadata) = self.character.get_helmet_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.head.x as u16 - metadata.offset.x as u16,
                    y: y + body.head.y as u16 - metadata.offset.y as u16,

                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }

        if let Some(metadata) = self.character.get_weapon_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.right_hand.x as u16 - metadata.offset.x as u16,
                    y: y + body.right_hand.y as u16 - metadata.offset.y as u16,

                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }

        if let Some(metadata) = self.character.get_shield_frame() {
            let image = &self.images[metadata.image as usize];

            draw_image(
                image.file,
                DrawImageParams {
                    x: x + body.left_hand.x as u16 - metadata.offset.x as u16,
                    y: y + body.left_hand.y as u16 - metadata.offset.y as u16,
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    z: 0.5 + (metadata.priority as f32 * 0.0001),
                },
            );
        }
    }

    fn randomize_character(&mut self) {
        let rng = &mut rand::thread_rng();
        self.character.body = self.bodies.choose(rng).unwrap().clone();
        self.character.skin = self.skins.choose(rng).unwrap().clone();
        self.character.face = Some(self.faces.choose(rng).unwrap().clone());
        self.character.eyes = Some(self.eyes.choose(rng).unwrap().clone());
        self.character.hair = Some(self.hairs.choose(rng).unwrap().clone());
        self.character.shield = Some(self.shields.choose(rng).unwrap().clone());
        self.character.helmet = Some(self.helmets.choose(rng).unwrap().clone());
        self.character.weapon = Some(self.weapons.choose(rng).unwrap().clone());
    }
}

#[derive(Debug)]
pub enum LoadingStage {
    Init,
    Background(ManagedTextureId),
    LoadingBodies,
    LoadingHeads,
    LoadingHelmets,
    LoadingShields,
    LoadingWeapons,
    // LoadingArmors,
    PreparingCharacter,
    Finish(Box<App>),
}

impl Display for LoadingStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LoadingStage::Init => "Initializing...",
            LoadingStage::Background(_) => "Background loaded",
            LoadingStage::LoadingBodies => "Loading bodies...",
            LoadingStage::LoadingHeads => "Loading heads...",
            LoadingStage::LoadingHelmets => "Loading helmets...",
            LoadingStage::LoadingShields => "Loading shields...",
            LoadingStage::LoadingWeapons => "Loading weapons...",
            // LoadingStage::LoadingArmors => "Loading armors...",
            LoadingStage::PreparingCharacter => "Preparing charcater...",
            LoadingStage::Finish(_) => "App loaded!",
        })
    }
}

enum Layout {
    Rows,
    Columns,
}

fn from_images(
    images: &mut Vec<Image>,
    file_num: u64,
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
