use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ClientPacket {
    Account(Account),
    UserAction(Action),
    Bank(Bank),
    Commerce(Commerce),
    Pet(Pet),
    Request(Request),
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Account {
    Create,
    Login,
    CharacterLogin,
    CharacterDelete,
    CharacterCreate,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Action {
    Talk,
    Move,
    Attack,
    LeftClick,
    DoubleClick,
    Meditate,

    UseItem,
    MoveItem,
    EquipItem,
    PickUpItem,
    DropItem,

    MoveSpell,
    CastSpell,

    LevelUpSkill,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Commerce {
    CommerceBuy,
    CommerceSell,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Bank {
    Show,
    Deposit,
    Extract,
    DepositItem,
    ExtractItem,
}
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Pet {
    Stand,
    Follow,
    Leave,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Request {
    SpellInfo,
    Attributes,
    Skills,
    Stats,
    Help,
    Online,
    Quit,
}
