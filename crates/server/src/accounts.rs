use std::sync::Arc;

use database::{
    model::{
        Account, Attributes, Character, CreateAccount, CreateCharacter, Equipment, Statistics,
    },
    Database,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::info;

pub struct Accounts {
    database: Arc<Database>,

    account_events_sender: Sender<AccountEvent>,
    account_events_receiver: Receiver<AccountEvent>,
}

#[derive(Debug)]
pub enum AccountEvent {
    Created {
        connection_id: u32,
        account_name: String,
    },
    CreateFailed {
        connection_id: u32,
        reason: String,
    },
    LoginAccountOk {
        connection_id: u32,
        account_name: String,
        characters: Vec<Character>,
    },
    LoginAccountFailed {
        connection_id: u32,
    },
    CreateCharacterOk {
        connection_id: u32,
        character: Character,
    },
    CreateCharacterFailed {
        connection_id: u32,
        reason: String,
    },
    LoginCharacterOk {
        connection_id: u32,
        character: Character,
    },
    LoginCharacterFailed {
        connection_id: u32,
    },
}

impl Accounts {
    pub fn initialize(database: Arc<Database>) -> Self {
        let (account_events_sender, account_events_receiver) = channel(100);
        Self {
            database,
            account_events_receiver,
            account_events_sender,
        }
    }

    pub async fn create(
        &self,
        connection_id: u32,
        name: &str,
        email: &str,
        password: &str,
        pin: usize,
    ) {
        tokio::spawn({
            let database = self.database.clone();

            let name = name.to_string();
            let email = email.to_string();
            let password = password.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                let create_account = CreateAccount {
                    name: &name,
                    email: &email,
                    password: &password,
                    pin: pin as i32,
                };
                let result = match database.create_account(&create_account).await {
                    Ok(account) => AccountEvent::Created {
                        connection_id,
                        account_name: account.name,
                    },
                    _ => AccountEvent::CreateFailed {
                        connection_id,
                        reason: "Invalid ID".to_string(),
                    },
                };
                info!("account creation result: {result:?}");

                account_events_sender.send(result).await.expect("poisoned");
            }
        });
    }

    pub async fn login(&self, connection_id: u32, name: &str, password: &str) {
        tokio::spawn({
            let database = self.database.clone();
            let name = name.to_string();
            let login_password = password.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                let account = database.account(&name).await;

                let result = match account {
                    Ok(Account { name, password, .. }) if password == login_password => {
                        let result = database.account_characters(&name).await;
                        let characters = result.ok().unwrap_or_else(std::vec::Vec::new);

                        AccountEvent::LoginAccountOk {
                            connection_id,
                            account_name: name,
                            characters,
                        }
                    }
                    _ => AccountEvent::LoginAccountFailed { connection_id },
                };

                account_events_sender.send(result).await.expect("poisoned");
            }
        });
    }

    pub async fn create_character(
        &self,
        connection_id: u32,
        account_name: &str,
        character: CreateCharacter,
    ) {
        tokio::spawn({
            let database = self.database.clone();
            let character = character.clone();
            let account_name = account_name.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                let attributes = Attributes::default();
                let equipment = Equipment::default();
                let statistics = Statistics::default();

                if let Ok(character) = database
                    .insert_character(
                        &account_name,
                        character,
                        &attributes,
                        &equipment,
                        &statistics,
                    )
                    .await
                {
                    account_events_sender
                        .send(AccountEvent::CreateCharacterOk {
                            connection_id,
                            character,
                        })
                        .await
                        .expect("poisoned");
                } else {
                    account_events_sender
                        .send(AccountEvent::CreateCharacterFailed {
                            connection_id,
                            reason: "Invalid name".to_string(),
                        })
                        .await
                        .expect("poisoned");
                }
            }
        });
    }

    pub async fn enter(&self, connection_id: u32, account_name: &str, character: &str) {
        tokio::spawn({
            let database = self.database.clone();
            let character = character.to_string();
            let account_name = account_name.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                if let Ok(account_characters) = database.account_characters(&account_name).await {
                    for account_character in account_characters {
                        if account_character.name.to_lowercase() == character.to_lowercase() {
                            account_events_sender
                                .send(AccountEvent::LoginCharacterOk {
                                    connection_id,
                                    character: account_character,
                                })
                                .await
                                .expect("poisoned");
                            return;
                        }
                    }
                    account_events_sender
                        .send(AccountEvent::LoginCharacterFailed { connection_id })
                        .await
                        .expect("poisoned");
                }
            }
        });
    }

    pub async fn poll_account_events(&mut self) -> Vec<AccountEvent> {
        let mut events = vec![];
        while let Ok(event) = self.account_events_receiver.try_recv() {
            events.push(event);
        }
        events
    }
}
