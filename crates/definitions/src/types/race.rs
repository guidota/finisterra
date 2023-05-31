#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Race {
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
