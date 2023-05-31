#[derive(Default, Clone, Debug, Eq, PartialEq)]
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
