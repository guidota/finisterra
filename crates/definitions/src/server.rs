use std::collections::BTreeMap;

use crate::{
    city::City,
    map::Map,
    npc::NPC,
    object::{CarpentryItem, Object, SmithyItem},
    spell::Spell,
};

#[derive(Default, Clone, Debug)]
pub struct ServerResources {
    pub maps: BTreeMap<usize, Map>,
    pub npcs: BTreeMap<usize, NPC>,
    pub objects: BTreeMap<usize, Object>,
    pub cities: BTreeMap<String, City>,
    pub spells: BTreeMap<usize, Spell>,
    pub smithy: Smithy,
    pub carpentry: Carpentry,
}

#[derive(Default, Clone, Debug)]
pub struct Carpentry {
    pub objects: BTreeMap<usize, CarpentryItem>,
}

#[derive(Default, Clone, Debug)]
pub struct Smithy {
    pub armors: BTreeMap<usize, SmithyItem>,
    pub weapons: BTreeMap<usize, SmithyItem>,
}
