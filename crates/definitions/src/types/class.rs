use std::fmt::{Display, Formatter, Result};

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

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Class::Mage => "Mago",
            Class::Druid => "Druida",
            Class::Thief => "Ladron",
            Class::Bard => "Bardo",
            Class::Pirate => "Pirata",
            Class::Cleric => "Clerigo",
            Class::Assesin => "Asesino",
            Class::Paladin => "Paladin",
            Class::Tailor => "Sastre", //?
            Class::Fisher => "Pescador",
            Class::Miner => "Minero",
            Class::Woodcutter => "Talador",
        })
    }
}
