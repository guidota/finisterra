use crate::bincode::CONFIG;
use crate::character::{Character, CharacterPreview};
use crate::protocol::ProtocolMessage;
use crate::world::{Direction, WorldPosition};

use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum ServerPacket {
    Intervals,
    Connection(Connection),
    Account(Account),
    CharacterUpdate(CharacterUpdate),
    UserUpdate(UserUpdate),
    Event(Event),
    Object(Object),
    Message(Message),
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Connection {
    Connected,
    Disconnect,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Account {
    Created {
        account_name: String,
    },
    CreateFailed {
        reason: String,
    },

    LoginOk {
        characters: Vec<CharacterPreview>,
    },
    LoginFailed,

    CreateCharacterOk {
        entity_id: u32,
        character: Character,
    },
    CreateCharacterFailed {
        reason: String,
    },

    LoginCharacterOk {
        entity_id: u32,
        character: Character,
    },
    LoginCharacterFailed {
        reason: String,
    },
}

/// Users and NPCs
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum CharacterUpdate {
    Create {
        entity_id: u32,
        character: Character,
    },
    Remove {
        entity_id: u32,
    },
    MoveResponse {
        request_id: u8,
        position: WorldPosition,
    },
    Move {
        entity_id: u32,
        position: WorldPosition,
    },
    Heading {
        entity_id: u32,
        direction: Direction,
    },
    Translate,
    Paralize,
    Info,
    Change,
    Meditate,
    Invisible,
    Attacked,
    DialogAdd,
    DialogRemove,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum UserUpdate {
    Sta,
    Mana,
    Health,
    Hunger,
    Thirst,
    Gold,
    Exp,
    Position,
    Stats,
    InventorySlot,
    SpellsSlot,
    BankSlot,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Event {
    LevelUp,
    Attack,
    ShieldBlock,
    Hit,
    Kill,
    FX,
    MapChanged,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Object {
    ObjectCreate,
    ObjectDelete,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Response {
    Attributes,
    Skills,
    Stats,
    Help,
    Online,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Message {
    CantUseWhileMeditating,
    ConsoleMessage,
}

impl ProtocolMessage for ServerPacket {
    fn decode(bytes: &[u8]) -> Option<Self> {
        bincode::decode_from_slice(bytes, CONFIG)
            .ok()
            .map(|(result, _)| result)
    }

    fn encode(self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(self, CONFIG).ok()
    }
}
