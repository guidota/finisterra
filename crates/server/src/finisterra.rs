use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use database::{model::CreateCharacter, Database};
use shared::protocol::{
    client::{self, ClientPacket},
    server::{self, ServerPacket},
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::{
    accounts::{AccountEvent, Accounts},
    server::Server,
    world::World,
};

pub struct Finisterra {
    server: Server,
    world: World,
    accounts: Accounts,

    /// connected users
    users: HashMap<u32, User>,

    outcoming_messages_sender: UnboundedSender<(u32, ServerPacket)>,

    last_tick: Instant,
}

pub enum User {
    Connected,
    InAccount {
        account_name: String,
        character_names: Vec<String>,
    },
    InWorld {
        // character: String,
        entity_id: u32,
    },
}

impl Finisterra {
    pub async fn initialize() -> Result<Self> {
        let database = Arc::new(Database::initialize().await?);

        // world will produce ServerPackets to be send to the users and the server will consume
        // and send them to the corresponding users
        let (sender, receiver) = unbounded_channel();

        let world = World::initialize(sender.clone());
        let server = Server::initialize(receiver).await?;
        let accounts = Accounts::initialize(database);

        let users = HashMap::default();

        Ok(Finisterra {
            accounts,
            server,
            world,
            users,
            outcoming_messages_sender: sender,

            last_tick: Instant::now(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let now = Instant::now();
            let delta = now - self.last_tick;
            if delta >= Duration::from_millis(16) {
                self.update_connections().await;
                self.process_incoming_messages().await;
                self.update_world().await;
                self.send_outcoming_messages().await;
                self.last_tick = Instant::now();
            }
        }
    }

    async fn send(&mut self, connection_id: u32, packet: ServerPacket) {
        self.outcoming_messages_sender
            .send((connection_id, packet))
            .expect("poisoned")
    }

    async fn update_connections(&mut self) {
        let (connections, disconnections) = self.server.update_connections().await;
        for connection_id in connections {
            self.users.insert(connection_id, User::Connected);
        }
        for connection_id in disconnections {
            if let Some(User::InWorld { entity_id }) = self.users.get(&connection_id) {
                self.world.remove_character(entity_id).await;
            }
            self.users.remove(&connection_id);
        }

        let authentication_events = self.accounts.poll_account_events().await;
        for event in authentication_events {
            match event {
                AccountEvent::Created {
                    connection_id,
                    account_name,
                } => {
                    self.users.insert(
                        connection_id,
                        User::InAccount {
                            account_name: account_name.clone(),
                            character_names: vec![],
                        },
                    );
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::Created { account_name }),
                    )
                    .await;
                }
                AccountEvent::CreateFailed {
                    connection_id,
                    reason,
                } => {
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::CreateFailed { reason }),
                    )
                    .await;
                }
                AccountEvent::LoginAccountOk {
                    connection_id,
                    account_name,
                    characters,
                } => {
                    let character_names = characters
                        .iter()
                        .map(|character| character.name.to_string())
                        .collect();
                    self.users.insert(
                        connection_id,
                        User::InAccount {
                            account_name,
                            character_names,
                        },
                    );

                    let mut account_characters = vec![];
                    for character in characters {
                        account_characters.push(character.into());
                    }
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::LoginOk {
                            characters: account_characters,
                        }),
                    )
                    .await;
                }
                AccountEvent::LoginAccountFailed { connection_id } => {
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::LoginFailed),
                    )
                    .await;
                }
                AccountEvent::LoginCharacterOk {
                    connection_id,
                    character,
                } => {
                    let character = character.into();
                    let entity_id = self.world.create_character(&character);
                    self.users
                        .insert(connection_id, User::InWorld { entity_id });
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::LoginCharacterOk {
                            entity_id,
                            character: character.clone(),
                        }),
                    )
                    .await;
                    self.world.notify_new_character(entity_id, &character).await;
                }
                AccountEvent::LoginCharacterFailed { connection_id } => {
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::LoginCharacterFailed {
                            reason: "Invalid character".to_string(),
                        }),
                    )
                    .await;
                }
                AccountEvent::CreateCharacterOk {
                    connection_id,
                    character,
                } => {
                    let character = character.into();
                    let entity_id = self.world.create_character(&character);
                    self.users
                        .insert(connection_id, User::InWorld { entity_id });
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::CreateCharacterOk {
                            entity_id,
                            character: character.clone(),
                        }),
                    )
                    .await;
                    self.world.notify_new_character(entity_id, &character).await;
                }
                AccountEvent::CreateCharacterFailed {
                    connection_id,
                    reason,
                } => {
                    self.send(
                        connection_id,
                        ServerPacket::Account(server::Account::CreateCharacterFailed { reason }),
                    )
                    .await
                }
            }
        }
    }

    async fn process_incoming_messages(&mut self) {
        let incoming_messages = self.server.read_incoming_messages().await;

        for (connection_id, message) in incoming_messages {
            if let ClientPacket::Account(message) = message {
                self.process_account_event(connection_id, message).await;
            } else if let Some(User::InWorld { entity_id }) = self.users.get(&connection_id) {
                self.world
                    .process_incoming_message(*entity_id, message)
                    .await;
            }
        }
    }

    async fn process_account_event(&mut self, connection_id: u32, message: client::Account) {
        match message {
            client::Account::CreateAccount {
                name,
                email,
                password,
                pin,
            } => {
                self.accounts
                    .create(connection_id, &name, &email, &password, pin)
                    .await
            }
            client::Account::LoginAccount { name, password } => {
                self.accounts.login(connection_id, &name, &password).await
            }
            client::Account::LoginCharacter { character } => {
                if let Some(User::InAccount {
                    character_names, ..
                }) = self.users.get(&connection_id)
                {
                    if character_names.contains(&character) {
                        self.accounts.enter(connection_id, &character).await
                    }
                }
            }
            client::Account::CreateCharacter {
                name,
                class,
                race,
                gender,
            } => {
                if let Some(User::InAccount { account_name, .. }) = self.users.get(&connection_id) {
                    let create_character = CreateCharacter {
                        name,
                        class_id: class.id() as i32,
                        race_id: race.id() as i32,
                        gender_id: gender.id() as i32,
                        // TODO: hardcoded
                        map: 1,
                        x: 50,
                        y: 50,
                        attributes: database::model::Attributes {
                            strength: 18,
                            agility: 18,
                            intelligence: 18,
                            charisma: 18,
                            constitution: 18,
                        },
                        statistics: database::model::Statistics {
                            health: 20,
                            mana: 100,
                            stamina: 100,
                            max_health: 20,
                            max_mana: 100,
                            max_stamina: 100,
                        },
                        look: database::model::Look::default(),
                        equipment: database::model::Equipment::default(),
                    };
                    self.accounts
                        .create_character(connection_id, account_name, create_character)
                        .await
                }
            }
            client::Account::DeleteCharacter { .. } => todo!(),
        }
    }

    async fn update_world(&mut self) {
        self.world.tick().await;
    }

    async fn send_outcoming_messages(&mut self) {
        self.server.send_outcoming_messages().await;
    }
}
