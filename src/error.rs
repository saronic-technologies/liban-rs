use thiserror::Error;

pub type Result<T> = std::result::Result<T, AnError>;

#[derive(Error, Debug)]
pub enum AnError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("Invalid packet format: {0}")]
    InvalidPacket(String),

    #[error("Invalid packet checksum")]
    InvalidChecksum,

    #[error("Invalid packet length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },

    #[error("Device error: {0}")]
    Device(String),

    #[error("Configuration validation failed: {0}")]
    ValidationFailed(String),

    #[error("Connection not established")]
    NotConnected,

    #[error("Unsupported packet ID: {0}")]
    UnsupportedPacketId(u8),

    #[error("Packet data too long: {0} bytes (max 255)")]
    PacketTooLong(usize),
}
