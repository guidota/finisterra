pub mod character;
pub mod client;
pub mod movement;
pub mod server;

pub trait ProtocolMessage {
    fn encode(self) -> Option<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}
