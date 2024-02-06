use std::rc::Rc;

use byteorder::ReadBytesExt;
use rustc_hash::FxHashMap;

use crate::{
    animation::Animation,
    atlas::AtlasResource,
    body::Body,
    client::ClientResources,
    error::Error,
    fx::FX,
    gear::HeadGear,
    head::Head,
    image::Image,
    map::{Map, Tile},
    parse::{
        get_binary_reader, get_ini_reader, ArgentumIniPropertyReadExt, ArgentumIniReadExt,
        ArgentumReadExt, Endian,
    },
    shield::Shield,
    weapon::Weapon,
    Offset,
};

mod template;

use self::template::{parse_templates, Template};

use super::maps::parse::MapsReadExt;

#[derive(Default)]
pub struct ClientResourcesPaths<'a> {
    pub bodies: &'a str,
    pub heads: &'a str,
    pub weapons: &'a str,
    pub shields: &'a str,
    pub headgears: &'a str,
    pub fxs: &'a str,
    pub maps: &'a str,
    pub graphics: &'a str,
    pub templates: &'a str,
    pub atlas: Option<AtlasResource<'a>>,
}

/// moldes.ini
pub fn load_templates(path: &str) -> Result<FxHashMap<usize, Template>, Error> {
    let templates = parse_templates(path);

    Ok(templates)
}

/// cuerpos.dat, depends on moldes.ini
pub fn load_bodies(
    path: &str,
    templates: &FxHashMap<usize, Template>,
    images: &mut FxHashMap<u32, Rc<Image>>,
    animations: &mut FxHashMap<usize, Animation>,
) -> Result<FxHashMap<usize, Body>, Error> {
    let mut bodies = FxHashMap::default();

    let mut latest_image_id = *images.keys().max().unwrap();
    let mut latest_animation_id = *animations.keys().max().unwrap();
    let ini = get_ini_reader(path).expect("File doesn't exist");
    for body_number in 1..=ini.get_count("NumBodies") {
        let Some(body_section) = ini.section(Some(&format!("BODY{body_number}"))) else {
            continue;
        };

        let head_offset = Offset {
            x: body_section
                .get("HeadOffsetX")
                .unwrap_or("0")
                .parse::<isize>()
                .expect("Bad head offset"),
            y: body_section
                .get("HeadOffsetY")
                .unwrap_or("0")
                .parse::<isize>()
                .expect("Bad head offset"),
        };

        let body = {
            let file_num = body_section
                .get("FileNum")
                .map(|s| s.parse::<u32>().unwrap());
            if let Some(file_num) = file_num {
                let std = body_section.get_number("Std");
                let template = templates.get(&std).expect("Bad template");
                // create images for each animation
                let mut body_animations = vec![];
                for _ in 0..4 {
                    latest_animation_id += 1;
                    body_animations.push(Animation {
                        id: latest_animation_id,
                        speed: 250, // TODO
                        frames: vec![],
                    });
                }

                for (animation_number, rect) in template.clone() {
                    latest_image_id += 1;
                    let image = Image {
                        id: latest_image_id,
                        file_num: file_num as u64,
                        x: rect.min.0 as u16,
                        y: rect.min.1 as u16,
                        width: (rect.max.0 - rect.min.0) as u16,
                        height: (rect.max.1 - rect.min.1) as u16,
                    };
                    images.insert(latest_image_id, Rc::new(image));
                    body_animations[animation_number]
                        .frames
                        .push(latest_image_id);
                }

                // create body with recent added animations
                let body = Body {
                    animations: [
                        body_animations[0].id,
                        body_animations[1].id,
                        body_animations[2].id,
                        body_animations[3].id,
                    ],
                    head_offset,
                };
                for animation in body_animations {
                    animations.insert(animation.id, animation);
                }
                body
            } else {
                Body {
                    animations: [
                        body_section.get_number("Walk3"),
                        body_section.get_number("Walk1"),
                        body_section.get_number("Walk4"),
                        body_section.get_number("Walk2"),
                    ],
                    head_offset,
                }
            }
        };

        bodies.insert(body_number, body);
    }

    Ok(bodies)
}

/// cabezas.ini or cabezas.ind
pub fn load_head(path: &str) -> Result<FxHashMap<usize, Head>, Error> {
    let mut heads = FxHashMap::default();

    let reader = get_ini_reader(path).expect("File doesn't exist");
    for head_number in 1..=reader.get_count("NumHeads") {
        let head_section = reader
            .section(Some(&format!("HEAD{head_number}")))
            .expect("Head {head_number} doesn't exist");
        let head = Head {
            images: [
                head_section.get_number("Head3"),
                head_section.get_number("Head1"),
                head_section.get_number("Head4"),
                head_section.get_number("Head2"),
            ],
        };
        heads.insert(head_number, head);
    }

    Ok(heads)
}

/// cascos.ini or cascos.ind?
pub fn load_headgears(path: &str) -> Result<FxHashMap<usize, HeadGear>, Error> {
    let mut headgears = FxHashMap::default();

    let reader = get_ini_reader(path).expect("File doesn't exist");
    for headgear_number in 1..=reader.get_count("NumHeads") {
        let head_section = reader
            .section(Some(&format!("HEAD{headgear_number}")))
            .expect("Head {head_number} doesn't exist");
        let headgear = HeadGear {
            images: [
                head_section.get_number("Head3"),
                head_section.get_number("Head1"),
                head_section.get_number("Head4"),
                head_section.get_number("Head2"),
            ],
        };
        println!("Helmet: {headgear:#?}");
        headgears.insert(headgear_number, headgear);
    }

    Ok(headgears)
}

/// armas.dat:
pub fn load_weapons(path: &str) -> Result<FxHashMap<usize, Weapon>, Error> {
    let mut weapons = FxHashMap::default();
    let reader = get_ini_reader(path)?;

    let count = reader.get_count("NumArmas");
    for number in 1..=count {
        let Some(section) = reader.section(Some(&format!("Arma{number}"))) else {
            continue;
        };

        weapons.insert(
            number,
            Weapon {
                animations: [
                    section.get_number("Dir3"),
                    section.get_number("Dir1"),
                    section.get_number("Dir4"),
                    section.get_number("Dir2"),
                ],
            },
        );
    }

    Ok(weapons)
}

/// escudos.dat
pub fn load_shields(
    path: &str,
    templates: &FxHashMap<usize, Template>,
    images: &mut FxHashMap<u32, Rc<Image>>,
    animations: &mut FxHashMap<usize, Animation>,
) -> Result<FxHashMap<usize, Shield>, Error> {
    let mut shields = FxHashMap::default();
    let mut latest_image_id = *images.keys().max().unwrap();
    let mut latest_animation_id = *animations.keys().max().unwrap();
    let reader = get_ini_reader(path)?;

    let count = reader.get_count("NumEscudos");
    for number in 1..=count {
        let Some(section) = reader.section(Some(&format!("ESC{number}"))) else {
            continue;
        };

        let file_num = section.get("FileNum").map(|s| s.parse::<u32>().unwrap());
        let shield = if let Some(file_num) = file_num {
            let std = section.get_number("Std");
            let template = templates.get(&std).expect("Bad template");
            // create images for each animation
            let mut shield_animations = vec![];
            for _ in 0..4 {
                latest_animation_id += 1;
                shield_animations.push(Animation {
                    id: latest_animation_id,
                    speed: 250, // TODO
                    frames: vec![],
                });
            }

            for (animation_number, rect) in template.clone() {
                latest_image_id += 1;
                let image = Image {
                    id: latest_image_id,
                    file_num: file_num as u64,
                    x: rect.min.0 as u16,
                    y: rect.min.1 as u16,
                    width: (rect.max.0 - rect.min.0) as u16,
                    height: (rect.max.1 - rect.min.1) as u16,
                };
                images.insert(latest_image_id, Rc::new(image));
                shield_animations[animation_number]
                    .frames
                    .push(latest_image_id);
            }

            // create body with recent added animations
            let shield = Shield {
                animations: [
                    shield_animations[0].id,
                    shield_animations[1].id,
                    shield_animations[2].id,
                    shield_animations[3].id,
                ],
            };
            for animation in shield_animations {
                animations.insert(animation.id, animation);
            }
            shield
        } else {
            Shield {
                animations: [
                    section.get_number("Dir3"),
                    section.get_number("Dir1"),
                    section.get_number("Dir4"),
                    section.get_number("Dir2"),
                ],
            }
        };
        shields.insert(number, shield);
    }

    Ok(shields)
}

/// FXs.ini or fxs.ind
pub fn load_fxs(path: &str) -> Result<FxHashMap<usize, FX>, Error> {
    let mut fxs = FxHashMap::default();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        fxs.insert(
            id.into(),
            FX {
                animation: reader.read_integer().into(),
                offset: Offset {
                    x: reader.read_signed_integer().into(),
                    y: reader.read_signed_integer().into(),
                },
            },
        );
    }
    Ok(fxs)
}

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

pub struct Graphics {
    pub images: FxHashMap<u32, Rc<Image>>,
    pub animations: FxHashMap<usize, Animation>,
}

/// graficos.ini or graficos.ind
pub fn load_graphics(path: &str) -> Result<Graphics, Error> {
    let mut reader = get_binary_reader(path)?;
    let mut images = FxHashMap::<u32, Rc<Image>>::default();
    let mut animations = FxHashMap::<usize, Animation>::default();

    reader.read_long();
    reader.read_long();

    while let Ok(grh) = reader.read_u32::<Endian>() {
        if grh == 0 {
            break;
        }
        match reader.read_integer() {
            0 => {
                return Err(Error::Parse);
            }
            1 => {
                let image = Image {
                    file_num: reader.read_long() as u64,
                    x: reader.read_integer(),
                    y: reader.read_integer(),
                    width: reader.read_integer(),
                    height: reader.read_integer(),
                    id: grh,
                };
                images.insert(grh, Rc::new(image));
            }
            frames_len => {
                let animation = Animation {
                    frames: (0..frames_len)
                        .map(|_| reader.read_long() as u32)
                        .collect::<Vec<_>>(),
                    speed: reader.read_long() as usize,
                    id: grh as usize,
                };
                animations.insert(grh as usize, animation);
            }
        }
    }
    Ok(Graphics { images, animations })
}

pub fn load_client_resources(paths: ClientResourcesPaths) -> Result<ClientResources, Error> {
    let Graphics {
        images: mut images_map,
        animations: mut animations_map,
    } = load_graphics(paths.graphics)?;
    println!(
        "Loaded {} images, {} animations",
        images_map.len(),
        animations_map.len()
    );
    let templates = load_templates(paths.templates)?;
    println!("Loaded {} templates", templates.len());

    let heads = load_head(paths.heads)?;
    println!("Loaded {} heads", heads.len());
    let weapons = load_weapons(paths.weapons)?;
    println!("Loaded {} weapons", weapons.len());
    let shields = load_shields(
        paths.shields,
        &templates,
        &mut images_map,
        &mut animations_map,
    )?;
    println!("Loaded {} shields", shields.len());
    let headgears = load_headgears(paths.headgears)?;
    println!("Loaded {} headgears", headgears.len());
    let fxs = load_fxs(paths.fxs)?;
    println!("Loaded {} fxs", fxs.len());
    let maps = load_maps(paths.maps)?;
    println!("Loaded {} maps", maps.len());

    let bodies = load_bodies(
        paths.bodies,
        &templates,
        &mut images_map,
        &mut animations_map,
    )?;
    println!("Loaded {} bodies", bodies.len());

    let max = images_map.iter().max_by_key(|item| item.0).unwrap();

    let mut images = vec![None; (*max.0 + 1) as usize];
    for (image_id, image) in images_map {
        images[image_id as usize] = Some(image);
    }

    let max = animations_map.iter().max_by_key(|item| item.0).unwrap();
    let mut animations = vec![None; max.0 + 1];
    for (id, animation) in animations_map {
        animations[id] = Some(Rc::new(animation));
    }

    Ok(ClientResources {
        bodies,
        heads,
        weapons,
        shields,
        headgears,
        fxs,
        maps,
        animations,
        images,
    })
}
