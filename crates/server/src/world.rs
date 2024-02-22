use protocol::server::ServerPacket;
use tokio::sync::mpsc::Sender;

pub struct World {
    // outcoming_messages_sender: Sender<(u32, ServerPacket)>,
}

impl World {
    pub fn initialize(_outcoming_messages_sender: Sender<(u32, ServerPacket)>) -> Self {
        Self {
            // outcoming_messages_sender,
        }
    }

    pub fn tick(&mut self) {}
}
