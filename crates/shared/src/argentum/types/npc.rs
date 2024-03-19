use crate::argentum::{heading::Heading, Range};

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum NpcKind {
    #[default]
    Common,
    Priest,
    Guard,
    Trainer,
    Banker,
    Nobel,
    Dragon,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CommerceItem {
    pub id: usize,
    pub amount: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Commerce {
    pub items: Vec<CommerceItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum MovementKind {
    #[default]
    Normal,
    Ground,
    Water,
    GroundAndWater,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NPC {
    pub id: usize,
    pub name: String,
    pub kind: NpcKind,
    pub description: String,

    // Rewards
    pub gold: usize,
    pub exp: usize,

    // Appeareance
    pub body: usize,
    pub head: usize,
    pub heading: Heading,

    // Behaviour
    pub movement: MovementKind,
    pub respawns: bool,
    pub hostile: bool,
    pub attackable: bool,
    pub spells: Vec<usize>,
    pub commerce: Option<Commerce>,

    // Attributes
    pub health: Range,
    pub hit: Range,
    pub defense: Range,
    pub attack_power: usize,
    pub evasion_power: usize,

    // Sounds
    pub hit_sound: usize,
    pub die_sound: usize,
    pub attack_sound: usize,
}
