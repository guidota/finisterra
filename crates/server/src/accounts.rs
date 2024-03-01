use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::info;

use database::{Account, Character, Database};

pub struct Accounts {
    database: Arc<Database>,

    account_events_sender: Sender<AccountEvent>,
    account_events_receiver: Receiver<AccountEvent>,
}

#[derive(Debug)]
pub enum AccountEvent {
    Created {
        connection_id: u32,
        id: i64,
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
        character: String,
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

    pub async fn create(&self, connection_id: u32, mail: &str, password: &str) {
        tokio::spawn({
            let database = self.database.clone();

            let mail = mail.to_string();
            let password = password.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                let result = match database.create_account(&mail, &password).await {
                    Ok(id) => AccountEvent::Created { connection_id, id },
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

    pub async fn login(&self, connection_id: u32, account_name: &str, password: &str) {
        tokio::spawn({
            let database = self.database.clone();
            let account_name = account_name.to_string();
            let login_password = password.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                let account = database.account_by_name(&account_name).await;

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

    pub async fn create_character(&self, connection_id: u32, account_id: i64, character: &str) {
        tokio::spawn({
            let database = self.database.clone();
            let character = character.to_string();

            let account_events_sender = self.account_events_sender.clone();

            async move {
                if (database.insert_character(account_id, &character).await).is_ok() {
                    account_events_sender
                        .send(AccountEvent::CreateCharacterOk {
                            connection_id,
                            character: Character {
                                account_name,
                                name: character,
                            },
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

            let account_events_sender = self.account_events_sender.clone();

            async move {
                if let Ok(account_characters) = database.account_characters(account_name).await {
                    for account_character in account_characters {
                        if account_character.name.to_lowercase() == character.to_lowercase() {
                            account_events_sender
                                .send(AccountEvent::LoginCharacterOk {
                                    connection_id,
                                    character: character.to_string(),
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
