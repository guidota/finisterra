mod admin;
mod bincode;
pub mod character;
pub mod client;
pub mod movement;
pub mod server;
pub mod world;

pub const CONFIG: Configuration = standard();

use ::bincode::config::standard;
use ::bincode::config::Configuration;

pub trait ProtocolMessage {
    fn encode(self) -> Option<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}
