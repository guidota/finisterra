use bincode::{Decode, Encode};

use crate::parse::{Byte, Integer, Long};

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct MapHeader {
    pub blocks: Long,
    pub layers: [Long; 4],
    pub triggers: Long,
    pub lights: Long,
    pub particles: Long,
    pub npcs: Long,
    pub objs: Long,
    pub exits: Long,
}

pub type Blocking = Byte;

pub type GrhIndex = Long;

pub type Trigger = Integer;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct RGBA {
    pub b: Byte,
    pub g: Byte,
    pub r: Byte,
    pub a: Byte,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Light {
    pub color: RGBA,
    pub range: Byte,
}

pub type Particle = Long;

pub type NpcIndex = Integer;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Obj {
    pub index: Integer,
    pub amount: Integer,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct WorldPosition {
    pub map: Integer,
    pub x: Integer,
    pub y: Integer,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Point {
    pub min: Integer,
    pub max: Integer,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct MapSize {
    pub x: Point,
    pub y: Point,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct MapInfo {
    pub map_name: String,
    pub backup_mode: bool,
    pub restrict_mode: String,
    pub music_number_high: Long,
    pub music_number_low: Long,
    pub secure: bool,
    pub zone: String,
    pub terrain: String,
    pub ambient: String,
    pub base_light: Long,
    pub letter_grh: Long,
    pub rain: bool,
    pub snow: bool,
    pub fog: bool,

    pub extra1: Long,
    pub extra2: Long,
    pub extra3: String,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Tile {
    pub graphics: [usize; 4],
    pub light: Option<Light>,
    pub particle: Option<Particle>,
    pub exit: Option<WorldPosition>,
    pub blocked: Blocking,
    pub trigger: Trigger,

    // these fields could be only on server side
    pub obj: Option<Obj>,
    pub npc: Option<NpcIndex>,
    pub user: Option<usize>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn tile(&mut self, (x, y): (Integer, Integer)) -> &mut Tile {
        &mut self.tiles[(x - 1) as usize][(y - 1) as usize]
    }

    const CONFIG: bincode::config::Configuration = bincode::config::standard();
    pub fn from_path(path: &str) -> Option<Self> {
        let file = std::fs::File::open(path).ok()?;
        let reader = std::io::BufReader::new(file);
        bincode::decode_from_reader(reader, Self::CONFIG).ok()
    }
}
