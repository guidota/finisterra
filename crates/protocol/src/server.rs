use bincode::{Decode, Encode};

use crate::{bincode::CONFIG, client::ClientPacket};

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ServerPacket {
    Intervals,
    Connection(Connection),
    Account(AccountCharacter),
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
pub enum AccountCharacter {
    List,
    Created,
    Logged,
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

pub struct ClientConnection {}

impl ClientConnection {
    pub fn send(&self, packet: ServerPacket) -> Result<(), bincode::error::EncodeError> {
        let _bytes = bincode::encode_to_vec(packet, CONFIG)?;
        Ok(())
    }

    pub fn receive(&self) -> Result<ClientPacket, bincode::error::DecodeError> {
        let payload: Vec<u8> = vec![];
        let (packet, _) = bincode::decode_from_slice(&payload, CONFIG)?;

        Ok(packet)
    }
}
