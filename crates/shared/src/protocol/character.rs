use crate::{
    bincode::CONFIG,
    character::{Inventory, Skills},
    protocol::ProtocolMessage,
};

impl ProtocolMessage for Inventory {
    fn decode(bytes: &[u8]) -> Option<Self> {
        bincode::decode_from_slice(bytes, CONFIG)
            .ok()
            .map(|(result, _)| result)
    }

    fn encode(self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(self, CONFIG).ok()
    }
}

impl ProtocolMessage for Skills {
    fn decode(bytes: &[u8]) -> Option<Self> {
        bincode::decode_from_slice(bytes, CONFIG)
            .ok()
            .map(|(result, _)| result)
    }

    fn encode(self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(self, CONFIG).ok()
    }
}
