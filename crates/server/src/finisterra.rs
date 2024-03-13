use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use database::{model::CreateCharacter, Database};
use protocol::{
    client::{self, ClientPacket},
    server::{self, ServerPacket},
};
use tokio::sync::mpsc::{channel, Sender};

use crate::{
    accounts::{AccountEvent, Accounts},
    server::Server,
    world::World,
};

pub struct Finisterra {
    server: Server,
    world: World,
    accounts: Accounts,

    users: HashMap<u32, User>,

    outcoming_messages_sender: Sender<(u32, ServerPacket)>,
}

pub struct User {
    state: UserState,
}

pub enum UserState {
    Connected,
    InAccount {
        account_name: String,
        character_names: Vec<String>,
    },
    InWorld {
        character: String,
    },
}

impl Finisterra {
    pub async fn initialize() -> Result<Self> {
        let database = Arc::new(Database::initialize().await?);

        // world will produce ServerPackets to be send to the users and the server will consume
        // and send them to the corresponding users
        let (sender, receiver) = channel(3000);

        let server = Server::initialize(receiver).await?;
        let world = World::initialize(sender.clone());
        let accounts = Accounts::initialize(database);
        let users = HashMap::default();

        Ok(Finisterra {
            accounts,
            server,
            world,
            users,
            outcoming_messages_sender: sender,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.update_connections().await;
            self.process_incoming_messages().await;
            self.update_world().await;
            self.send_outcoming_messages().await;
        }
    }

    async fn update_connections(&mut self) {
        let (connections, disconnections) = self.server.update_connections().await;
        for connection_id in connections {
            self.users.insert(
                connection_id,
                User {
                    state: UserState::Connected,
                },
            );
        }
        for connection_id in disconnections {
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
                        User {
                            state: UserState::InAccount {
                                account_name: account_name.clone(),
                                character_names: vec![],
                            },
                        },
                    );
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::Created { account_name }),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::CreateFailed {
                    connection_id,
                    reason,
                } => {
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::CreateFailed { reason }),
                        ))
                        .await
                        .expect("poisoned");
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
                        User {
                            state: UserState::InAccount {
                                account_name,
                                character_names,
                            },
                        },
                    );

                    let mut account_characters = vec![];
                    for character in characters {
                        account_characters.push(character.into());
                    }
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::LoginOk {
                                characters: account_characters,
                            }),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::LoginAccountFailed { connection_id } => {
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::LoginFailed),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::LoginCharacterOk {
                    connection_id,
                    character,
                } => {
                    self.users.insert(
                        connection_id,
                        User {
                            state: UserState::InWorld {
                                character: character.name.to_string(),
                            },
                        },
                    );
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::LoginCharacterOk {
                                character: character.into(),
                            }),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::LoginCharacterFailed { connection_id } => {
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::LoginCharacterFailed {
                                reason: "Invalid character".to_string(),
                            }),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::CreateCharacterOk {
                    connection_id,
                    character,
                } => {
                    self.users.insert(
                        connection_id,
                        User {
                            state: UserState::InWorld {
                                character: character.name.to_string(),
                            },
                        },
                    );
                    self.outcoming_messages_sender
                        .send((
                            connection_id,
                            ServerPacket::Account(server::Account::CreateCharacterOk {
                                character: character.into(),
                            }),
                        ))
                        .await
                        .expect("poisoned");
                }
                AccountEvent::CreateCharacterFailed {
                    connection_id,
                    reason,
                } => self
                    .outcoming_messages_sender
                    .send((
                        connection_id,
                        ServerPacket::Account(server::Account::CreateCharacterFailed { reason }),
                    ))
                    .await
                    .expect("poisoned"),
            }
        }
    }

    async fn process_incoming_messages(&mut self) {
        let incoming_messages = self.server.poll_incoming_messages().await;

        for (connection_id, message) in incoming_messages {
            match message {
                ClientPacket::Account(message) => match message {
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
                        if let Some(user) = self.users.get(&connection_id) {
                            if let UserState::InAccount {
                                character_names, ..
                            } = &user.state
                            {
                                if character_names.contains(&character) {
                                    self.accounts.enter(connection_id, &character).await
                                }
                            }
                        }
                    }
                    client::Account::CreateCharacter {
                        name,
                        class,
                        race,
                        gender,
                    } => {
                        if let Some(user) = self.users.get(&connection_id) {
                            if let UserState::InAccount { account_name, .. } = &user.state {
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
                    }
                    client::Account::DeleteCharacter { .. } => todo!(),
                },
                ClientPacket::UserAction(_) => todo!(),
                ClientPacket::Bank(_) => todo!(),
                ClientPacket::Commerce(_) => todo!(),
                ClientPacket::Pet(_) => todo!(),
                ClientPacket::Request(_) => todo!(),
            }
        }
    }

    async fn update_world(&mut self) {
        self.world.tick();
    }

    async fn send_outcoming_messages(&mut self) {
        self.server.send_outcoming_messages().await;
    }
}
