use crate::{heading::Heading, Range};

#[derive(Default, Clone, Debug, Eq, PartialEq)]
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

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct CommerceItem {
    pub id: usize,
    pub amount: usize,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Commerce {
    pub items: Vec<CommerceItem>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum Movement {
    #[default]
    Normal,
    Ground,
    Water,
    GroundAndWater,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
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
    pub movement: Movement,
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
