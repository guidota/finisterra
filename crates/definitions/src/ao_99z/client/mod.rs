use std::collections::BTreeMap;

use byteorder::ReadBytesExt;

use crate::{
    animation::Animation,
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
}

pub fn load_bodies(path: &str) -> Result<BTreeMap<usize, Body>, Error> {
    let mut bodies = BTreeMap::new();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        bodies.insert(
            id.into(),
            Body {
                animations: [
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                ],
                head_offset: Offset {
                    x: reader.read_integer().into(),
                    y: reader.read_integer().into(),
                },
                ..Default::default()
            },
        );
    }
    Ok(bodies)
}

pub fn load_head(path: &str) -> Result<BTreeMap<usize, Head>, Error> {
    let mut heads = BTreeMap::new();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        heads.insert(
            id.into(),
            Head {
                images: [
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                ],
            },
        );
    }
    Ok(heads)
}

pub fn load_headgears(path: &str) -> Result<BTreeMap<usize, HeadGear>, Error> {
    let mut headgears = BTreeMap::new();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        headgears.insert(
            id.into(),
            HeadGear {
                images: [
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                ],
            },
        );
    }
    Ok(headgears)
}

/// this one is from .dat file (?)
pub fn load_weapons(path: &str) -> Result<BTreeMap<usize, Weapon>, Error> {
    let mut weapons = BTreeMap::new();
    let reader = get_ini_reader(path)?;

    for number in 1..=reader.get_count("NumArmas") {
        let section = reader
            .section(Some(&format!("Arma{number}")))
            .ok_or(Error::Parse)?;
        weapons.insert(
            number,
            Weapon {
                animations: [
                    section.get_number("Dir1"),
                    section.get_number("Dir2"),
                    section.get_number("Dir3"),
                    section.get_number("Dir4"),
                ],
            },
        );
    }

    Ok(weapons)
}

pub fn load_shields(path: &str) -> Result<BTreeMap<usize, Shield>, Error> {
    let mut shields = BTreeMap::new();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        shields.insert(
            id.into(),
            Shield {
                animations: [
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                    reader.read_integer().into(),
                ],
            },
        );
    }
    Ok(shields)
}

pub fn load_fxs(path: &str) -> Result<BTreeMap<usize, FX>, Error> {
    let mut fxs = BTreeMap::new();
    let mut reader = get_binary_reader(path)?;

    reader.skip_header();

    for id in 0..reader.read_integer() {
        fxs.insert(
            id.into(),
            FX {
                animation: reader.read_integer().into(),
                offset: Offset {
                    x: reader.read_integer().into(),
                    y: reader.read_integer().into(),
                },
            },
        );
    }
    Ok(fxs)
}

pub fn load_maps(path: &str) -> Result<BTreeMap<usize, Map>, Error> {
    let mut maps = BTreeMap::new();

    let dir = std::fs::read_dir(path).map_err(|_| Error::FileNotFound)?;
    for entry in dir {
        let file = entry.map_err(|_| Error::FileNotFound)?;
        let path = file.path();
        let path = path.to_str().ok_or(Error::Parse)?;

        let map_file_num = path.to_string().replace(".map", "").replace("Mapa", "");
        let Ok(id) = map_file_num.parse::<usize>() else {
            continue;
        };
        let mut reader = get_binary_reader(path)?;

        // Header, version and other trash
        reader.read_integer();
        reader.skip_header();
        reader.read_integer();
        reader.read_integer();
        reader.read_integer();
        reader.read_integer();

        let mut map = Map::default();
        for y in 1..=100 {
            map.tiles.push(Vec::with_capacity(100));
            for x in 1..=100 {
                map.tiles[x - 1][y - 1] = Tile {
                    blocked: reader.read_byte(),
                    graphics: [
                        reader.read_integer().into(),
                        reader.read_integer().into(),
                        reader.read_integer().into(),
                        reader.read_integer().into(),
                    ],
                    trigger: reader.read_integer(),
                    ..Default::default()
                };
                reader.read_integer();
            }
        }
        maps.insert(id, map);
    }

    Ok(maps)
}

pub struct Graphics {
    pub images: BTreeMap<usize, Image>,
    pub animations: BTreeMap<usize, Animation>,
}

pub fn load_graphics(path: &str) -> Result<Graphics, Error> {
    let mut images = BTreeMap::new();
    let mut animations = BTreeMap::new();

    let mut reader = get_binary_reader(path)?;

    reader.skip_header();
    reader.read_integer();
    reader.read_integer();
    reader.read_integer();
    reader.read_integer();
    reader.read_integer();

    let mut grh = reader.read_integer();
    while grh > 0 {
        let frames_len = reader.read_integer();

        match frames_len {
            0 => return Err(Error::Parse),
            1 => {
                images.insert(
                    grh.into(),
                    Image {
                        id: grh.into(),
                        file_num: reader.read_integer().into(),
                        x: reader.read_integer(),
                        y: reader.read_integer(),
                        width: reader.read_integer(),
                        height: reader.read_integer(),
                    },
                );
            }
            _ => {
                animations.insert(
                    grh.into(),
                    Animation {
                        id: grh.into(),
                        frames: (0..frames_len)
                            .map(|_| reader.read_integer().into())
                            .collect(),
                        speed: reader.read_integer().into(),
                    },
                );
            }
        }
        grh = match reader.read_u16::<Endian>() {
            Ok(val) => val,
            Err(_) => break,
        };
    }

    Ok(Graphics { images, animations })
}

pub fn load_client_resources(paths: ClientResourcesPaths) -> Result<ClientResources, Error> {
    let bodies = load_bodies(paths.bodies)?;
    let heads = load_head(paths.heads)?;
    let weapons = load_weapons(paths.weapons)?;
    let shields = load_shields(paths.shields)?;
    let headgears = load_headgears(paths.headgears)?;
    let fxs = load_fxs(paths.fxs)?;
    let maps = load_maps(paths.maps)?;
    let Graphics { images, animations } = load_graphics(paths.graphics)?;

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
