//! # liban - Advanced Navigation Packet Protocol Library (Sans-IO)
//!
//! A sans-io Rust library for working with Advanced Navigation devices using the
//! Advanced Navigation Packet Protocol (ANPP), with specific support for the Boreas D90.

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
