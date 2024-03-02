use crate::{networking::bincode::CONFIG, ProtocolMessage};
use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
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

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Connection {
    Connected,
    Disconnect,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Account {
    Created { id: i64 },
    CreateFailed { reason: String },

    LoginOk { characters: Vec<String> },
    LoginFailed,

    CreateCharacterOk { character: Character },
    CreateCharacterFailed { reason: String },

    LoginCharacterOk { character: Character },
    LoginCharacterFailed { reason: String },
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Character {
    pub name: String,
}

/// Users and NPCs
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum CharacterUpdate {
    Create,
    Remove,
    Move,
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

#[derive(Encode, Decode, PartialEq, Debug)]
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

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Event {
    LevelUp,
    Attack,
    ShieldBlock,
    Hit,
    Kill,
    FX,
    MapChanged,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Object {
    ObjectCreate,
    ObjectDelete,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Response {
    Attributes,
    Skills,
    Stats,
    Help,
    Online,
}

#[derive(Encode, Decode, PartialEq, Debug)]
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
