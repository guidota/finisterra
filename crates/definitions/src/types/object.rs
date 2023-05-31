use crate::{race::Race, Range};

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub id: usize,
    pub name: String,
    pub info: String,
    pub grh: usize,
    pub value: usize,
    pub data: ObjectData,
    pub smithy: Option<Smithy>,
    pub carpentry: Option<Carpentry>,
    pub not_allowed: Vec<Race>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Smithy {
    pub gold_ingots: usize,
    pub silver_ingots: usize,
    pub bronze_ingots: usize,
    pub skills: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Carpentry {
    pub wood: usize,
    pub skills: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum DoorState {
    #[default]
    Open,
    Closed,
    ClosedWithKey {
        password: usize,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum PotionKind {
    #[default]
    Mana,
    Health,
    Agility,
    Strength,
    Poison,
    Death,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ObjectData {
    #[default]
    Empty,
    Food {
        amount: usize,
    },
    Weapon {
        hit: Range,
        animation: usize,
    },
    Armor {
        defense: Range,
        animation: usize,
    },
    Tree,
    Gold,
    Door {
        state: DoorState,
        index_open: usize,
        index_closed: usize,
        index_key_closed: usize,
    },
    Container,
    Poster {
        text: String,
    },
    Key {
        password: usize,
    },
    Forum {
        id: String,
    },
    Potion {
        kind: PotionKind,
        amount: Range,
    },
    Book {
        text: String,
    },
    Beverage {
        amount: usize,
        stamina: usize,
    },
    Wood,
    Bonfire,
    Shield {
        animation: usize,
        defense: Range,
    },
    Helmet {
        animation: usize,
        defense: Range,
    },
    Tool,
    Teleport,
    Furniture,
    Jewel,
    MineralDeposit {
        index: usize,
    },
    Metals {
        skills: usize,
        ingot_index: usize,
    },
    Parchment {
        spell: usize,
    },
    Aura,
    MusicInstrument {
        sound_1: usize,
        sound_2: usize,
        sound_3: usize,
    },
    Anvil,
    Forge,
    Gem,
    Flower,
    Boat {
        animation: usize,
        skills: usize,
        defense: Range,
        hit: Range,
    },
    Arrow,
    EmptyBottle,
    Bottle,
    Stain,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CarpentryItem {
    id: usize,
    item_index: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SmithyItem {
    id: usize,
    item_index: usize,
}
