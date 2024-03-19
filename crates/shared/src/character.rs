use std::fmt::{self, Display, Formatter};

use bincode::{Decode, Encode};

use crate::world::WorldPosition;

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct CharacterPreview {
    pub name: String,
    pub level: u16,
    pub exp: Stat<u64>,
    pub gold: u64,
    pub position: WorldPosition,

    pub class: Class,
    pub race: Race,
    pub look: Look,
    pub equipment: Equipment,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub level: u16,
    pub exp: Stat<u64>,
    pub gold: u64,
    pub position: WorldPosition,

    pub class: Class,
    pub race: Race,
    pub look: Look,
    pub equipment: Equipment,

    pub attributes: Attributes,
    pub skills: Skills,
    pub stats: Stats,
    pub inventory: Inventory,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Look {
    pub body: u8,
    pub skin: u8,
    pub face: u8,
    pub hair: u8,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Equipment {
    pub weapon: Option<u8>,
    pub shield: Option<u8>,
    pub headgear: Option<u8>,
    pub clothing: Option<u8>,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub enum Race {
    #[default]
    Human,
    Elf,
    Drow,
    Gnome,
    Dwarf,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub enum Class {
    #[default]
    Mage,
    Druid,
    Thief,
    Bard,
    Pirate,
    Cleric,
    Assesin,
    Paladin,

    // Workers
    Tailor,
    Fisher,
    Miner,
    Woodcutter,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Skills {
    // combat
    pub weapons: u8,
    pub projectiles: u8,
    pub tactics: u8,
    pub defense: u8,
    pub stab: u8,
    pub wrestling: u8,
    // magic
    pub magic: u8,
    pub resistence: u8,
    // job
    pub woodcutting: u8,
    pub smithy: u8,
    pub carpentry: u8,
    pub alchemy: u8,
    pub tailor: u8,
    // misc
    pub steal: u8,
    pub meditate: u8,
    pub hide: u8,
    pub survival: u8,
    pub trading: u8,
    pub leadership: u8,
    pub tame: u8,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Attributes {
    pub strength: u16,
    pub agility: u16,
    pub intelligence: u16,
    pub charisma: u16,
    pub constitution: u16,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Stats {
    pub health: Stat<u16>,
    pub mana: Stat<u16>,
    pub stamina: Stat<u16>,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Stat<T> {
    pub current: T,
    pub max: T,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Vault {
    items: Vec<Item>,
    gold: u64,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Inventory {
    items: Vec<Item>,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct Item {
    item_id: u32,
    amount: u32,
}

impl Class {
    pub const VALUES: [Self; 12] = [
        Self::Mage,
        Self::Druid,
        Self::Bard,
        Self::Cleric,
        Self::Assesin,
        Self::Paladin,
        Self::Pirate,
        Self::Thief,
        Self::Woodcutter,
        Self::Fisher,
        Self::Miner,
        Self::Tailor,
    ];
    pub fn id(&self) -> usize {
        let mut result = 0;
        for item in Self::VALUES.iter() {
            if item == self {
                return result;
            }
            result += 1;
        }
        result
    }
    pub fn from(id: usize) -> Option<Self> {
        for (i, item) in Self::VALUES.iter().enumerate() {
            if i == id {
                return Some(item.clone());
            }
        }
        None
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Class::Mage => "Mago",
            Class::Druid => "Druida",
            Class::Thief => "Ladron",
            Class::Bard => "Bardo",
            Class::Pirate => "Pirata",
            Class::Cleric => "Clerigo",
            Class::Assesin => "Asesino",
            Class::Paladin => "Paladin",
            Class::Tailor => "Sastre",
            Class::Fisher => "Pescador",
            Class::Miner => "Minero",
            Class::Woodcutter => "Talador",
        })
    }
}

impl Race {
    pub const VALUES: [Self; 5] = [Self::Human, Self::Elf, Self::Drow, Self::Dwarf, Self::Gnome];
    pub fn id(&self) -> usize {
        let mut result = 0;
        for item in Self::VALUES.iter() {
            if item == self {
                return result;
            }
            result += 1;
        }
        result
    }
    pub fn from(id: usize) -> Option<Self> {
        for (i, item) in Self::VALUES.iter().enumerate() {
            if i == id {
                return Some(item.clone());
            }
        }
        None
    }
}
impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Race::Human => "Humano",
            Race::Elf => "Elfo",
            Race::Dwarf => "Enano",
            Race::Gnome => "Gnomo",
            Race::Drow => "Elfo Oscuro",
        })
    }
}

impl Gender {
    pub const VALUES: [Self; 2] = [Self::Male, Self::Female];
    pub fn id(&self) -> usize {
        let mut result = 0;
        for item in Self::VALUES.iter() {
            if item == self {
                return result;
            }
            result += 1;
        }
        result
    }
    pub fn from(id: usize) -> Option<Self> {
        for (i, item) in Self::VALUES.iter().enumerate() {
            if i == id {
                return Some(item.clone());
            }
        }
        None
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Gender::Male => "Hombre",
            Gender::Female => "Mujer",
        })
    }
}
