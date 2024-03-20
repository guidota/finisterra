use shared::world::{Map, WorldPosition};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use nohash_hasher::IntMap;
use shared::{
    character::Character,
    protocol::{
        client::{self, ClientPacket},
        movement::MoveRequest,
        server::{CharacterUpdate, ServerPacket},
    },
};
use tokio::sync::mpsc::UnboundedSender;

use self::{maps::load_maps, networking::Target};

mod maps;
mod movement;

pub struct World {
    outcoming_messages_sender: UnboundedSender<(u32, ServerPacket)>,

    maps: IntMap<u16, Map>,
    entities: IntMap<u32, Entity>,
    next_entity_id: u32,
}

pub enum Entity {
    Character {
        character: Character,
        last_move: Instant,
        last_move_receive: Instant,
        pending_moves: VecDeque<MoveRequest>,
    },
    // NPC,
}

impl World {
    pub fn initialize(outcoming_messages_sender: UnboundedSender<(u32, ServerPacket)>) -> Self {
        let maps = load_maps("assets/finisterra/maps/");
        let entities = IntMap::default();
        Self {
            outcoming_messages_sender,
            entities,
            next_entity_id: 0,
            maps,
        }
    }

    pub async fn process_incoming_message(&mut self, entity_id: u32, message: ClientPacket) {
        match message {
            ClientPacket::UserAction(action) => match action {
                client::Action::Move(move_request) => {
                    if let Some(Entity::Character {
                        last_move_receive,
                        pending_moves,
                        character,
                        ..
                    }) = self.entities.get_mut(&entity_id)
                    {
                        let now = Instant::now();
                        let elapsed_since_last_move = now - *last_move_receive;
                        if elapsed_since_last_move > Duration::from_millis(100) {
                            *last_move_receive = now;
                            pending_moves.push_back(move_request);
                        } else {
                            tracing::error!(
                                "received a move request but last move was {}ms ago",
                                elapsed_since_last_move.as_millis()
                            );
                            let world_position = character.position;
                            self.send(
                                ServerPacket::CharacterUpdate(CharacterUpdate::MoveResponse {
                                    request_id: move_request.id,
                                    position: world_position,
                                }),
                                Target::User { entity_id },
                            );
                        }
                    }
                }
                client::Action::Talk { .. } => todo!(),
                _ => {}
            },
            ClientPacket::Bank(_) => todo!(),
            ClientPacket::Commerce(_) => todo!(),
            ClientPacket::Pet(_) => todo!(),
            ClientPacket::Request(_) => todo!(),
            ClientPacket::Account(_) => unreachable!(),
        }
    }

    pub fn create_character(&mut self, character: &Character) -> u32 {
        let entity = Entity::Character {
            character: character.clone(),
            last_move: Instant::now() - Duration::from_millis(200),
            last_move_receive: Instant::now() - Duration::from_millis(200),
            pending_moves: VecDeque::new(),
        };
        let id = self.next_entity_id;
        self.entities.insert(id, entity);
        self.next_entity_id += 1;

        let WorldPosition { map, x, y } = character.position;
        if let Some(map) = self.maps.get_mut(&map) {
            map.tile_mut(x, y).user = Some(id);
        }

        id
    }

    pub async fn notify_new_character(&mut self, id: u32, character: &Character) {
        // notify near entities about new character
        let character_create = ServerPacket::CharacterUpdate(CharacterUpdate::Create {
            entity_id: id,
            character: character.clone(),
        });
        self.send(character_create, Target::AreaButUser { entity_id: id });
        // notify user about near entities
        for (area_entity_id, entity) in &self.entities {
            if area_entity_id == &id {
                continue;
            }
            match entity {
                Entity::Character { character, .. } => {
                    let character_create = ServerPacket::CharacterUpdate(CharacterUpdate::Create {
                        entity_id: *area_entity_id,
                        character: character.clone(),
                    });
                    self.send(character_create, Target::User { entity_id: id });
                }
            }
        }
    }

    pub async fn remove_character(&mut self, entity_id: &u32) {
        if let Some(Entity::Character { character, .. }) = self.entities.get(entity_id) {
            let WorldPosition { map, x, y } = character.position;
            if let Some(map) = self.maps.get_mut(&map) {
                map.tile_mut(x, y).user = None;
            }
        }
        let character_remove = ServerPacket::CharacterUpdate(CharacterUpdate::Remove {
            entity_id: *entity_id,
        });
        self.send(
            character_remove,
            Target::AreaButUser {
                entity_id: *entity_id,
            },
        );
        self.entities.remove(entity_id);
    }

    pub async fn tick(&mut self) {
        self.process_pending_moves();
    }
}

mod networking {
    use shared::{protocol::server::ServerPacket, world::WorldPosition};

    use super::World;

    pub enum Target {
        User { entity_id: u32 },
        Area { position: WorldPosition },
        AreaButUser { entity_id: u32 },
        // TODO
    }

    impl World {
        // todo remove entity id and use target
        pub fn send(&self, packet: ServerPacket, target: Target) {
            match target {
                Target::User { entity_id } => {
                    self.outcoming_messages_sender
                        .send((entity_id, packet))
                        .expect("poisoned");
                }
                Target::Area { .. } => {
                    for entity_id in self.entities.keys() {
                        //todo send to near users in entity_id area
                        self.outcoming_messages_sender
                            .send((*entity_id, packet.clone()))
                            .expect("poisoned");
                    }
                }
                Target::AreaButUser { entity_id } => {
                    for area_entity_id in self.entities.keys() {
                        if area_entity_id == &entity_id {
                            continue;
                        }
                        //todo send to near users in entity_id area
                        self.outcoming_messages_sender
                            .send((*area_entity_id, packet.clone()))
                            .expect("poisoned");
                    }
                }
            }
        }
    }
}
