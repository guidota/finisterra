use std::{
    fmt::Display,
    ops::{Add, Sub},
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    time::{Duration, Instant},
};

use engine::{
    draw::{
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::GameEngine,
};
use shared::protocol::{client::ClientPacket, server::ServerPacket, ProtocolMessage};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use wtransport::{ClientConfig, Endpoint};

use crate::ui::{
    colors::{GREEN, RED, YELLOW},
    fonts::TAHOMA_BOLD_8_SHADOW_ID,
};

pub struct ConnectionState {
    url: String,
    state: State,
    text: ParsedText,
}

#[derive(Default)]
enum State {
    #[default]
    Disconnected,
    Retry {
        time: Duration,
        elapsed: Duration,
    },
    Connecting {
        connection_receiver: Receiver<Connection>,
    },
    Connected {
        connection: Connection,
    },
}

struct Connection {
    inner: wtransport::Connection,

    outgoing_messages_sender: Sender<ClientPacket>,
    incoming_messages_receiver: Receiver<ServerPacket>,

    last_recv: Instant,

    cancellation_token: CancellationToken,
}

impl ConnectionState {
    pub fn new<E: GameEngine>(url: &str, engine: &mut E) -> Self {
        Self {
            url: url.to_string(),
            state: State::Disconnected,
            text: engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format!("{}", State::Disconnected))
                .expect("can parse"),
        }
    }

    pub fn connect(&mut self) {
        if matches!(self.state, State::Disconnected) {
            let (connection_sender, connection_receiver) = channel();
            self.state = State::Connecting {
                connection_receiver,
            };
            tokio::spawn({
                let url = self.url.clone();
                async move {
                    if let Some(connection) = Connection::establish(&url).await {
                        connection_sender.send(connection).expect("poisoned");
                    }
                }
            });
        }
    }

    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        match &mut self.state {
            State::Disconnected => self.connect(),
            State::Retry {
                ref mut elapsed,
                time,
            } => {
                *elapsed = elapsed.add(delta);
                if elapsed.ge(&time) {
                    self.change_state(State::Disconnected, engine);
                }
            }
            State::Connecting {
                connection_receiver,
            } => match connection_receiver.try_recv() {
                Ok(connection) => self.change_state(State::Connected { connection }, engine),
                Err(TryRecvError::Disconnected) => {
                    self.state = State::Retry {
                        elapsed: Duration::ZERO,
                        time: Duration::from_secs(3),
                    }
                }
                _ => {}
            },
            State::Connected { .. } => {}
        }
    }

    fn change_state<E: GameEngine>(&mut self, state: State, engine: &mut E) {
        self.state = state;
        self.text = engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format!("{}", self.state))
            .expect("can parse");
    }

    pub fn send(&self, message: ClientPacket) {
        match &self.state {
            State::Connected { connection } => {
                if connection.outgoing_messages_sender.send(message).is_err() {
                    error!("poisoned");
                }
            }
            _ => {
                error!("trying to send a message to the server but not connected");
            }
        }
    }

    pub fn read(&mut self) -> Vec<ServerPacket> {
        let mut messages = vec![];
        let mut connection_closed = false;

        if let State::Connected { connection } = &mut self.state {
            loop {
                let now = Instant::now();
                if now - connection.last_recv < Duration::from_millis(0) {
                    break;
                }
                connection.last_recv = now;
                match connection.incoming_messages_receiver.try_recv() {
                    Ok(message) => messages.push(message),
                    Err(TryRecvError::Disconnected) => {
                        connection_closed = true;
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }
        }
        if connection_closed {
            self.close();
            self.state = State::Disconnected;
        }

        messages.reverse();

        messages
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, State::Connected { .. })
    }

    pub fn close(&self) {
        if let State::Connected { connection } = &self.state {
            connection.cancellation_token.cancel();
        }
    }

    pub fn ping(&self) -> u16 {
        if let State::Connected { connection } = &self.state {
            connection.ping()
        } else {
            0
        }
    }

    pub fn draw_state<E: GameEngine>(&mut self, engine: &mut E) {
        if let State::Retry { .. } = self.state {
            self.text = engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format!("{}", self.state))
                .expect("can parse text");
        }

        engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.text,
                position: Position {
                    x: 5 + self.text.total_width / 2,
                    y: 5,
                    z: 1.,
                },
                color: self.color(),
            },
            Target::UI,
        );
    }
}

impl Connection {
    pub async fn establish(url: &str) -> Option<Self> {
        let config = ClientConfig::builder()
            .with_bind_default()
            .with_no_cert_validation()
            .build();

        let connection = Endpoint::client(config).ok()?.connect(url).await.ok()?;

        let (mut connection_sender, mut connection_receiver) =
            connection.open_bi().await.ok()?.await.ok()?;
        info!("connection accepted!");

        let (outgoing_messages_sender, outgoing_messages_receiver) = channel::<ClientPacket>();
        let (incoming_messages_sender, incoming_messages_receiver) = channel::<ServerPacket>();

        let cancellation_token = CancellationToken::new();
        tokio::spawn({
            let token = cancellation_token.clone();
            async move {
                let mut buffer = vec![0; 65536].into_boxed_slice();
                loop {
                    if token.is_cancelled() {
                        break;
                    }
                    match connection_receiver.read(&mut buffer).await {
                        Ok(Some(bytes_read)) => {
                            if let Some(message) = ServerPacket::decode(&buffer[..bytes_read]) {
                                info!("server <= \n {message:?}");

                                if incoming_messages_sender.send(message).is_err() {
                                    error!("poisoned");
                                }
                            } else {
                                error!("couldn't decode server packet");
                                break;
                            }
                        }
                        Ok(None) => info!("no message to read from server"),
                        Err(e) => {
                            error!("server connection closed! {e:?}");
                            break;
                        }
                    }
                }
            }
        });

        tokio::spawn({
            let token = cancellation_token.clone();
            async move {
                let mut last_send = Instant::now();
                loop {
                    if token.is_cancelled() {
                        break;
                    }
                    let now = Instant::now();
                    if now - last_send < Duration::from_millis(0) {
                        continue;
                    }
                    last_send = now;

                    if let Ok(message) = outgoing_messages_receiver.recv() {
                        info!("server => \n {message:?}");
                        if let Some(bytes) = message.encode() {
                            if connection_sender.write_all(&bytes).await.is_err() {
                                error!("failed to send message to server");
                            }
                        } else {
                            error!("couldn't serialize message to send");
                            break;
                        }
                    } else {
                        info!("stop sending messages!");
                        break;
                    }
                }
            }
        });

        Some(Self {
            inner: connection,
            outgoing_messages_sender,
            incoming_messages_receiver,
            cancellation_token,

            last_recv: Instant::now(),
        })
    }

    pub fn ping(&self) -> u16 {
        (self.inner.rtt().as_millis() / 2) as u16
    }
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.state {
            State::Disconnected => f.write_str("Disconnected"),
            State::Retry { elapsed, time } => f.write_str(&format!(
                "Retrying in {:.1}s...",
                time.sub(*elapsed).as_secs_f32()
            )),
            State::Connecting { .. } => f.write_str("Connecting..."),
            State::Connected { .. } => f.write_str("Connected"),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            State::Disconnected => f.write_str("Disconnected"),
            State::Retry { elapsed, time } => f.write_str(&format!(
                "Retrying in {:.1}s...",
                time.sub(*elapsed).as_secs_f32()
            )),
            State::Connecting { .. } => f.write_str("Connecting..."),
            State::Connected { .. } => f.write_str("Connected"),
        }
    }
}

impl ConnectionState {
    pub fn color(&self) -> [u8; 4] {
        match &self.state {
            State::Disconnected | State::Retry { .. } => RED,
            State::Connecting { .. } => YELLOW,
            State::Connected { .. } => GREEN,
        }
    }
}
