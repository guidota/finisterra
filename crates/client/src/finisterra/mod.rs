use definitions::{
    ao_20, ao_99z,
    atlas::{AtlasResource, AtlasType},
    client::ClientResources,
};
use engine::game::Game;
use protocol::client::{Account, ClientPacket};
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver};
use tracing::info;

use self::{connection::Connection, login::Login, world::World};

mod connection;
mod login;
pub mod world;

pub struct Finisterra {
    pub resources: ClientResources,
    pub state: State,
}

pub enum State {
    Loading,
    Connect,
    Connecting {
        receiver: Receiver<Connection>,
    },
    Login {
        login: Login,
        connection: Connection,
    },
    World {
        world: World,
        connection: Connection,
    },
    Disconnected,
}

impl Game for Finisterra {
    fn initialize<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
        // initialize resources
        Self::ao_20()
    }

    fn tick<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        match &mut self.state {
            State::Loading => {
                // do some rendering screen?
                self.state = State::Connect;
            }
            State::Connect => {
                let (sender, receiver) = channel();

                let url = "https://[::1]:7666";
                info!("connecting to {url}...");
                tokio::spawn(async move {
                    if let Some(connection) = Connection::establish(url).await {
                        sender.send(connection).expect("poisoned");
                    }
                });
                self.state = State::Connecting { receiver };
            }
            State::Connecting { ref mut receiver } => match receiver.try_recv() {
                Ok(connection) => {
                    connection.send(ClientPacket::Account(Account::LoginAccount {
                        mail: "a@a.com".to_string(),
                        password: "asd".to_string(),
                    }));

                    self.state = State::Login {
                        login: Login {},
                        connection,
                    };

                    info!("connected!");
                }
                Err(TryRecvError::Closed) => {
                    self.state = State::Disconnected;
                    info!("disconnected");
                }
                Err(_) => {}
            },
            State::Login { ref mut login, .. } => {
                login.update(engine);
                login.render(engine);
            }
            State::World { ref mut world, .. } => {
                world.update(engine);
                world.render(engine);
            }
            State::Disconnected => {
                // oops! something wrong with the connection
            }
        }
    }
}

impl Finisterra {
    pub const TAHOMA_ID: u64 = 1;

    pub fn ao_20() -> Self {
        let paths = ao_20::client::ClientResourcesPaths {
            bodies: "./assets/ao_20/init/cuerpos.dat",
            templates: "./assets/ao_20/init/moldes.ini",
            heads: "./assets/ao_20/init/cabezas.ini",
            weapons: "./assets/ao_20/init/armas.dat",
            shields: "./assets/ao_20/init/escudos.dat",
            headgears: "./assets/ao_20/init/cascos.ini",
            fxs: "./assets/ao_20/init/fxs.ind",
            maps: "./assets/ao_20/maps/",
            graphics: "./assets/ao_20/init/graficos.ind",
            atlas: None,
        };
        let resources =
            ao_20::client::load_client_resources(paths).expect("can load client resources");

        Self {
            resources,
            state: State::Loading,
        }
    }

    pub fn ao_99z() -> Self {
        let atlas = AtlasResource {
            metadata_path: "./assets/finisterra/atlas.toml",
            image_id: 0,
            atlas_type: AtlasType::Yatp,
        };
        let paths = ao_99z::client::ClientResourcesPaths {
            bodies: "./assets/ao_99z/Personajes.ind",
            heads: "./assets/ao_99z/Cabezas.ind",
            weapons: "./assets/ao_99z/Armas.dat",
            shields: "./assets/ao_99z/Escudos.dat",
            headgears: "./assets/ao_99z/Cascos.ind",
            fxs: "./assets/ao_99z/Fxs.ind",
            maps: "./assets/ao_99z/maps/",
            graphics: "./assets/ao_99z/Graficos.ind",
            atlas: Some(atlas),
        };

        let resources =
            ao_99z::client::load_client_resources(paths).expect("can load client resources");

        Self {
            resources,
            state: State::Loading,
        }
    }
}

impl Drop for Finisterra {
    fn drop(&mut self) {
        match &mut self.state {
            State::Login { connection, .. } | State::World { connection, .. } => {
                connection.close();
            }
            _ => {}
        }
    }
}
