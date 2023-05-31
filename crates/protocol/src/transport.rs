use bincode::{Decode, Encode};

pub enum TransportError {
    ConnectionClosed,
    Protocol,
}

pub type TransportResult<T> = Result<T, TransportError>;

pub trait BinaryTransport<R: Decode, S: Encode> {
    fn receive(&self) -> TransportResult<R>;
    fn send(&self) -> TransportResult<S>;
}

struct Transport;

impl<R: Decode, S: Encode> BinaryTransport<R, S> for Transport {
    fn receive(&self) -> TransportResult<R> {
        todo!()
    }

    fn send(&self) -> TransportResult<S> {
        todo!()
    }
}
