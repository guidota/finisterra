use std::{collections::HashMap, env, sync::Arc, time::Duration};

use anyhow::Result;
use protocol::{client::ClientPacket, server::ServerPacket, ProtocolMessage};
use tokio::sync::{
    mpsc::{channel, Receiver},
    Mutex,
};
use tracing::{error, info};
use wtransport::{Certificate, Endpoint, SendStream, ServerConfig};

enum ConnectionEvent {
    Accepted {
        connection_id: u32,
        stream: SendStream,
    },
    Disconnected {
        connection_id: u32,
    },
}

pub struct Server {
    connection_events_receiver: Receiver<ConnectionEvent>,
    incoming_messages_receiver: Receiver<(u32, ClientPacket)>,
    outcoming_messages_receiver: Receiver<(u32, ServerPacket)>,

    streams: Arc<Mutex<HashMap<u32, SendStream>>>,
}

impl Server {
    pub async fn initialize(
        outcoming_messages_receiver: Receiver<(u32, ServerPacket)>,
    ) -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let port = args
            .get(1)
            .map(|port| port.parse::<u16>().unwrap_or(0))
            .unwrap_or(0);

        let certificate = Certificate::self_signed(["localhost"]);
        let config = ServerConfig::builder()
            .with_bind_default(port)
            .with_certificate(certificate)
            .keep_alive_interval(Some(Duration::from_secs(3)))
            .build();

        let endpoint = Endpoint::server(config)?;
        let port = endpoint.local_addr().unwrap().port();
        info!("Server running on port {}", port);

        let (incoming_messages_sender, incoming_messages_receiver) = channel(3000);
        let (connection_events_sender, connection_events_receiver) = channel(100);

        tokio::spawn({
            async move {
                for connection_id in 0.. {
                    let incoming_session = endpoint.accept().await;
                    tokio::spawn({
                        let connection_events_sender = connection_events_sender.clone();
                        let incoming_messages_sender = incoming_messages_sender.clone();

                        async move {
                            let session_request = incoming_session.await.unwrap();
                            let connection = session_request.accept().await.unwrap();
                            let (stream, mut recv) = connection.accept_bi().await.unwrap();
                            info!("connection accepted {connection_id}!");

                            connection_events_sender
                                .send(ConnectionEvent::Accepted {
                                    connection_id,
                                    stream,
                                })
                                .await
                                .expect("poisoned");

                            let mut buffer = vec![0; 65536].into_boxed_slice();
                            while let Ok(Some(bytes_read)) = recv.read(&mut buffer).await {
                                if let Some(message) = ClientPacket::decode(&buffer[..bytes_read]) {
                                    info!("connection {connection_id} <= {message:#?}");
                                    incoming_messages_sender
                                        .send((connection_id, message))
                                        .await
                                        .expect("poisoned");
                                } else {
                                    error!("connection {connection_id} sent an invalid packet, kicking...");
                                    break;
                                }
                            }

                            info!("connection dropped {connection_id}!");
                            connection_events_sender
                                .send(ConnectionEvent::Disconnected { connection_id })
                                .await
                                .expect("poisoned");
                        }
                    });
                }
            }
        });

        let streams = Arc::new(Mutex::new(HashMap::new()));

        Ok(Self {
            connection_events_receiver,
            incoming_messages_receiver,
            outcoming_messages_receiver,
            streams,
        })
    }

    pub async fn update_connections(&mut self) -> (Vec<u32>, Vec<u32>) {
        let mut connections = vec![];
        let mut disconnections = vec![];
        let mut streams = self.streams.lock().await;

        while let Ok(event) = self.connection_events_receiver.try_recv() {
            match event {
                ConnectionEvent::Accepted {
                    connection_id,
                    stream,
                } => {
                    streams.insert(connection_id, stream);
                    connections.push(connection_id);
                }
                ConnectionEvent::Disconnected { connection_id } => {
                    streams.remove(&connection_id);
                    disconnections.push(connection_id);
                }
            }
        }
        (connections, disconnections)
    }

    pub async fn poll_incoming_messages(&mut self) -> Vec<(u32, ClientPacket)> {
        let mut messages = vec![];
        while let Ok(incoming_message) = self.incoming_messages_receiver.try_recv() {
            messages.push(incoming_message);
        }

        messages
    }

    pub async fn send_outcoming_messages(&mut self) {
        let mut outcoming_messages = vec![];
        while let Ok(message) = self.outcoming_messages_receiver.try_recv() {
            outcoming_messages.push(message);
        }
        tokio::spawn({
            let streams = self.streams.clone();

            async move {
                for (connection_id, message) in outcoming_messages {
                    info!("connection {connection_id} => {message:#?}");
                    if let Some(stream) = streams.lock().await.get_mut(&connection_id) {
                        if let Some(bytes) = message.encode() {
                            if stream.write_all(&bytes).await.is_err() {
                                error!("failed to send message to client");
                            }
                        } else {
                            error!("failed to encode message before sending to client");
                        }
                    }
                }
            }
        });
    }
}
