#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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
