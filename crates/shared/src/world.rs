use bincode::{Decode, Encode};

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct WorldPosition {
    pub map: u16,
    pub x: u16,
    pub y: u16,
}

impl WorldPosition {
    pub fn get_direction(&self, other: &WorldPosition) -> Option<Direction> {
        if other.x > self.x {
            Some(Direction::East)
        } else if other.x < self.x {
            Some(Direction::West)
        } else if other.y > self.y {
            Some(Direction::North)
        } else if other.y < self.y {
            Some(Direction::South)
        } else {
            None
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone, Copy)]
pub enum Direction {
    North,
    East,
    #[default]
    South,
    West,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct MapHeader {
    pub blocks: u32,
    pub layers: [u32; 4],
    pub triggers: u32,
    pub lights: u32,
    pub particles: u32,
    pub npcs: u32,
    pub objs: u32,
    pub exits: u32,
}

pub type Blocking = u8;

pub type GrhIndex = u32;

pub type Trigger = u16;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct RGBA {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Light {
    pub color: RGBA,
    pub range: u8,
}

pub type Particle = u32;

pub type NpcIndex = u16;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Obj {
    pub index: u16,
    pub amount: u16,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Point {
    pub min: u16,
    pub max: u16,
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
    pub music_number_high: u32,
    pub music_number_low: u32,
    pub secure: bool,
    pub zone: String,
    pub terrain: String,
    pub ambient: String,
    pub base_light: u32,
    pub letter_grh: u32,
    pub rain: bool,
    pub snow: bool,
    pub fog: bool,

    pub extra1: u32,
    pub extra2: u32,
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

    // tile state
    pub obj: Option<Obj>,
    pub npc: Option<NpcIndex>,
    pub user: Option<u32>,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Encode, Decode,
)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn tile(&self, x: u16, y: u16) -> &Tile {
        &self.tiles[(x - 1) as usize][(y - 1) as usize]
    }
    pub fn tile_mut(&mut self, x: u16, y: u16) -> &mut Tile {
        &mut self.tiles[(x - 1) as usize][(y - 1) as usize]
    }

    const CONFIG: bincode::config::Configuration = bincode::config::standard();
    pub fn from_path(path: &str) -> Option<Self> {
        let file = std::fs::File::open(path).ok()?;
        let reader = std::io::BufReader::new(file);
        bincode::decode_from_reader(reader, Self::CONFIG).ok()
    }

    pub fn next_position(&self, position: &WorldPosition, direction: Direction) -> WorldPosition {
        if position.x <= 1 || position.x >= 99 || position.y <= 1 || position.y >= 99 {
            return *position;
        }
        let target = match direction {
            Direction::North => WorldPosition {
                map: position.map,
                x: position.x,
                y: position.y + 1,
            },
            Direction::East => WorldPosition {
                map: position.map,
                x: position.x + 1,
                y: position.y,
            },
            Direction::South => WorldPosition {
                map: position.map,
                x: position.x,
                y: position.y - 1,
            },
            Direction::West => WorldPosition {
                map: position.map,
                x: position.x - 1,
                y: position.y,
            },
        };

        let tile = self.tile(target.x, target.y);
        if tile.blocked != 0 || tile.user.is_some() {
            *position
        } else {
            target
        }
    }
}
