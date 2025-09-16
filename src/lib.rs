//! # liban - Advanced Navigation Packet Protocol Library (Sans-IO)
//!
//! A sans-io Rust library for working with Advanced Navigation devices using the
//! Advanced Navigation Packet Protocol (ANPP), with specific support for the Boreas D90.
//!
//! This library provides the core protocol implementation without any I/O operations,
//! making it suitable for use in various environments (async, sync, embedded, etc.).
//!
//! ## Features
//!
//! - Full ANPP protocol implementation with CRC16-CCITT validation
//! - Sans-IO design - no built-in networking
//! - Type-safe packet handling and serialization
//! - Comprehensive error handling
//! - Zero-copy packet parsing where possible
//!
//! ## Example Usage
//!
//! ```rust
//! use std::convert::TryFrom;
//! use liban::{AnppProtocol, PacketId, DeviceInformationPacket};
//!
//! // Create a request packet
//! let request_packet = AnppProtocol::create_packet(
//!     PacketId::Request.as_u8(),
//!     &[PacketId::DeviceInformation.as_u8()]
//! )?;
//!
//! // The request packet is ready to send over your chosen transport
//! assert!(!request_packet.is_empty());
//!
//! // Example of parsing using TryFrom
//! // In real usage you'd get data from your transport
//! // let (packet_id, data) = AnppProtocol::parse_packet(&response_bytes)?;
//! // let device_info = DeviceInformationPacket::try_from(data.as_slice())?;
//!
//! // For demonstration without actual network data:
//! let packet_id = PacketId::DeviceInformation.as_u8();
//! println!("Would parse packet with ID: {}", packet_id);
//!
//! # Ok::<(), liban::AnError>(())
//! ```
//!
//! See the `examples/` directory for complete I/O implementations including TCP.

pub mod error;
pub mod packet;
pub mod protocol;

pub use error::{AnError, Result};
pub use packet::PacketId;
pub use packet::system::{
    AcknowledgePacket,
    BootModePacket,
    DeviceInformationPacket,
    IpConfigurationPacket,
    RequestPacket,
    ResetPacket,
    RestoreFactorySettingsPacket,
};
pub use packet::state::{
    StatusPacket,
    SystemStatePacket,
    UnixTimePacket,
};
pub use packet::flags::{
    FilterStatusFlags,
    SystemStatusFlags,
};
pub use packet::config::{
    FilterOptionsPacket,
    InstallationAlignmentPacket,
    IpDataportEntry,
    IpDataportMode,
    IpDataportsConfigurationPacket,
    OdometerConfigurationPacket,
    OffsetVector,
    PacketPeriodEntry,
    PacketTimerPeriodPacket,
    PacketsPeriodPacket,
    ReferencePointOffsetsPacket,
    SetZeroOrientationAlignmentPacket,
    VehicleType,
};
pub use protocol::AnppProtocol;
