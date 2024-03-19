pub mod city;
pub mod class;
pub mod heading;
pub mod npc;
pub mod object;
pub mod race;
pub mod spell;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Range {
    pub min: usize,
    pub max: usize,
}
