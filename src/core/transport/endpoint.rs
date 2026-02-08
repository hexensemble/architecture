use crate::core::protocol::message::*;
use crate::core::transport::error::*;

/// Client-side endpoint - Sends ClientMessage, receives ServerMessage
pub trait ClientEndpoint {
    fn send(&mut self, msg: ClientMessage) -> Result<(), TransportError>;
    fn recv(&mut self) -> Option<ServerMessage>;
}

/// Server-side endpoint - Sends ServerMessage, receives ClientMessage
pub trait ServerEndpoint {
    fn send(&mut self, msg: ServerMessage) -> Result<(), TransportError>;
    fn recv(&mut self) -> Option<ClientMessage>;
}
