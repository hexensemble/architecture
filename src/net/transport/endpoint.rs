use crate::net::protocol::message::*;
use crate::net::transport::error::*;

pub trait ClientEndpoint {
    fn send(&mut self, msg: ClientMessage) -> Result<(), TransportError>;
    fn recv(&mut self) -> Option<ServerMessage>;
}

pub trait ServerEndpoint {
    fn send(&mut self, msg: ServerMessage) -> Result<(), TransportError>;
    fn recv(&mut self) -> Option<ClientMessage>;
}
