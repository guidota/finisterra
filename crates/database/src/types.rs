use protocol::{
    character::{Class, Inventory, Race, Skills, Stat, Stats},
    world::WorldPosition,
    ProtocolMessage,
};

use crate::model::{self};

impl From<model::Character> for protocol::character::Character {
    fn from(character: model::Character) -> Self {
        Self {
            name: character.name,
            description: character.description,
            level: character.level as u16,
            exp: Stat {
                current: character.exp as u64,
                max: character.exp as u64,
            },
            gold: character.gold as u64,
            position: WorldPosition {
                map: character.map as u16,
                x: character.x as u16,
                y: character.y as u16,
            },
            class: Class::from(character.class_id as usize).unwrap(),
            race: Race::from(character.race_id as usize).unwrap(),
            look: character.look.into(),
            equipment: character.equipment.into(),
            attributes: character.attributes.into(),
            skills: Skills::decode(&character.skills).unwrap_or_default(),
            stats: Stats {
                health: Stat::<u16> {
                    current: character.stats.health as u16,
                    max: character.stats.max_health as u16,
                },
                mana: Stat::<u16> {
                    current: character.stats.mana as u16,
                    max: character.stats.max_mana as u16,
                },
                stamina: Stat::<u16> {
                    current: character.stats.stamina as u16,
                    max: character.stats.max_stamina as u16,
                },
            },
            inventory: Inventory::decode(&character.inventory).unwrap_or_default(),
        }
    }
}

impl From<model::Attributes> for protocol::character::Attributes {
    fn from(value: model::Attributes) -> Self {
        Self {
            strength: value.strength as u16,
            agility: value.agility as u16,
            intelligence: value.intelligence as u16,
            charisma: value.charisma as u16,
            constitution: value.constitution as u16,
        }
    }
}

impl From<model::Equipment> for protocol::character::Equipment {
    fn from(value: model::Equipment) -> Self {
        Self {
            weapon: value.weapon.map(|value| value as u8),
            shield: value.shield.map(|value| value as u8),
            headgear: value.headgear.map(|value| value as u8),
            clothing: value.clothing.map(|value| value as u8),
        }
    }
}

impl From<model::Look> for protocol::character::Look {
    fn from(value: model::Look) -> Self {
        Self {
            body: value.body as u8,
            skin: value.skin as u8,
            face: value.face as u8,
            hair: value.hair as u8,
        }
    }
}

impl From<model::CharacterPreview> for protocol::character::CharacterPreview {
    fn from(character: model::CharacterPreview) -> Self {
        Self {
            name: character.name,
            level: character.level as u16,
            exp: Stat {
                current: character.exp as u64,
                max: character.exp as u64,
            },
            gold: character.gold as u64,
            position: WorldPosition {
                map: character.map as u16,
                x: character.x as u16,
                y: character.y as u16,
            },
            class: Class::from(character.class_id as usize).unwrap(),
            race: Race::from(character.race_id as usize).unwrap(),
            look: character.look.into(),
            equipment: character.equipment.into(),
        }
    }
}
