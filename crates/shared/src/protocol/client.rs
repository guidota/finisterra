use bincode::{Decode, Encode};

use crate::world::WorldPosition;
use crate::{bincode::CONFIG, protocol::ProtocolMessage};

use crate::character::{Class, Gender, Race};

use super::movement::MoveRequest;

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
    CreateAccount {
        name: String,
        email: String,
        password: String,
        pin: usize,
    },
    LoginAccount {
        name: String,
        password: String,
    },
    LoginCharacter {
        character: String,
    },
    DeleteCharacter {
        character: String,
    },
    CreateCharacter {
        name: String,
        class: Class,
        race: Race,
        gender: Gender,
        // TODO! character customization
    },
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Action {
    Talk {
        text: String,
    },
    Move(MoveRequest),
    Attack,
    LeftClick {
        position: WorldPosition,
    },
    DoubleClick {
        position: WorldPosition,
    },
    Meditate,

    UseItem {
        slot: u8,
    },
    MoveItem {
        from: u8,
        to: u8,
    },
    EquipItem {
        slot: u8,
    },
    PickUpItem,
    DropItem {
        position: WorldPosition,
        slot: u8,
        amount: u16,
    },

    MoveSpell {
        from: u8,
        to: u8,
    },
    CastSpell {
        slot: u8,
        position: WorldPosition,
    },

    LevelUpSkill {
        skill_id: u8,
    },
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Commerce {
    Buy,
    Sell,
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

impl ProtocolMessage for ClientPacket {
    fn decode(bytes: &[u8]) -> Option<Self> {
        bincode::decode_from_slice(bytes, CONFIG)
            .ok()
            .map(|(result, _)| result)
    }

    fn encode(self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(self, CONFIG).ok()
    }
}
