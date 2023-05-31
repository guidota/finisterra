#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum Target {
    #[default]
    User = 1,
    NPC = 2,
    UserAndNPC = 3,
    Terrain = 4,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum SpellKind {
    #[default]
    None,
    Stats(StatEffect),
    State(StateEffect),
    Invoke {
        npc: usize,
    },
    Materialize {
        item: usize,
        amount: usize,
    },
    Metamorphosis,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum Stat {
    #[default]
    HP,
    Mana,
    Stamina,
    Hungry,
    Thirst,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum StatEffect {
    #[default]
    Damage,
    Heal {
        stat: Stat,
        min: usize,
        max: usize,
    },
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum StateEffect {
    #[default]
    None,
    Paralisis(bool),
    Invisibility(bool),
    Poison(bool),
    Resurrection,
    Malediction,
    Blessing,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Spell {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub magic_words: String,

    pub message_source: String,
    pub message_target: String,
    pub message_self: String,

    pub sound: usize,
    pub fx: usize,
    pub loops: usize,

    pub required_mana: usize,
    pub required_skill: usize,

    pub kind: SpellKind,
    pub target: Target,
}
