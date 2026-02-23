use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum TransportError {
    Disconnected,
    SendFailed,
}

impl Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Transport disconnected."),
            Self::SendFailed => write!(f, "Failed to send message."),
        }
    }
}

impl std::error::Error for TransportError {}
