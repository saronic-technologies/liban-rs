//! Clean Rust API types without wire format concerns
//!
//! This module provides ergonomic Rust types that are free from:
//! - Packet IDs (implicit in the type)
//! - Reserved fields (handled during serialization)
//! - Bitflags (promoted to individual bool fields)
//!
//! Convert to/from wire format packet types using `From`/`Into`.

use serde::{Serialize, Deserialize};
use crate::packet::AnppPacket;

pub mod system_types;
pub mod state_types;
pub mod config_types;

// Re-export for convenience
pub use system_types::*;
pub use state_types::*;
pub use config_types::*;

/// Clean packet enum for the public API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Packet {
    // System packets
    Acknowledge(Acknowledge),
    Request(Request),
    BootMode(BootMode),
    DeviceInformation(DeviceInformation),
    RestoreFactorySettings(RestoreFactorySettings),
    Reset(Reset),
    IpConfiguration(IpConfiguration),

    // State packets
    SystemState(SystemState),
    UnixTime(UnixTime),
    Status(Status),
    EulerOrientationStdDev(EulerOrientationStdDev),
    RawSensors(RawSensors),
    Satellites(Satellites),
    ExternalTime(ExternalTime),
    Heave(Heave),
    SensorTemperature(SensorTemperature),

    // Config packets
    PacketTimerPeriod(PacketTimerPeriod),
    PacketsPeriod(PacketsPeriod),
    InstallationAlignment(InstallationAlignment),
    FilterOptions(FilterOptions),
    OdometerConfiguration(OdometerConfiguration),
    SetZeroOrientationAlignment(SetZeroOrientationAlignment),
    ReferencePointOffsets(ReferencePointOffsets),
    IpDataportsConfiguration(IpDataportsConfiguration),

    // Unsupported
    Unsupported,
}

impl From<AnppPacket> for Packet {
    fn from(p: AnppPacket) -> Self {
        match p {
            AnppPacket::Acknowledge(inner) => Packet::Acknowledge(Acknowledge::from_packet_with_crc(inner)),
            AnppPacket::Request(inner) => Packet::Request(inner.into()),
            AnppPacket::BootMode(inner) => Packet::BootMode(inner.into()),
            AnppPacket::DeviceInformation(inner) => Packet::DeviceInformation(inner.into()),
            AnppPacket::RestoreFactorySettings(inner) => Packet::RestoreFactorySettings(inner.into()),
            AnppPacket::Reset(inner) => Packet::Reset(inner.into()),
            AnppPacket::IpConfiguration(inner) => Packet::IpConfiguration(inner.into()),
            AnppPacket::SystemState(inner) => Packet::SystemState(inner.into()),
            AnppPacket::UnixTime(inner) => Packet::UnixTime(inner.into()),
            AnppPacket::Status(inner) => Packet::Status(inner.into()),
            AnppPacket::EulerOrientationStdDev(inner) => Packet::EulerOrientationStdDev(inner.into()),
            AnppPacket::RawSensors(inner) => Packet::RawSensors(inner.into()),
            AnppPacket::Satellites(inner) => Packet::Satellites(inner.into()),
            AnppPacket::ExternalTime(inner) => Packet::ExternalTime(inner.into()),
            AnppPacket::Heave(inner) => Packet::Heave(inner.into()),
            AnppPacket::SensorTemperature(inner) => Packet::SensorTemperature(inner.into()),
            AnppPacket::PacketTimerPeriod(inner) => Packet::PacketTimerPeriod(inner.into()),
            AnppPacket::PacketsPeriod(inner) => Packet::PacketsPeriod(inner.into()),
            AnppPacket::InstallationAlignment(inner) => Packet::InstallationAlignment(inner.into()),
            AnppPacket::FilterOptions(inner) => Packet::FilterOptions(inner.into()),
            AnppPacket::OdometerConfiguration(inner) => Packet::OdometerConfiguration(inner.into()),
            AnppPacket::SetZeroOrientationAlignment(inner) => Packet::SetZeroOrientationAlignment(inner.into()),
            AnppPacket::ReferencePointOffsets(inner) => Packet::ReferencePointOffsets(inner.into()),
            AnppPacket::IpDataportsConfiguration(inner) => Packet::IpDataportsConfiguration(inner.into()),
            AnppPacket::Unsupported(_) => Packet::Unsupported,
        }
    }
}

impl Packet {
    /// Convert packet to wire format bytes ready to send
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        let wire_packet: AnppPacket = match self {
            Packet::Request(p) => AnppPacket::Request(p.clone().into()),
            Packet::BootMode(p) => AnppPacket::BootMode(p.clone().into()),
            Packet::RestoreFactorySettings(p) => AnppPacket::RestoreFactorySettings(p.clone().into()),
            Packet::Reset(p) => AnppPacket::Reset(p.clone().into()),
            Packet::IpConfiguration(p) => AnppPacket::IpConfiguration(p.clone().into()),
            Packet::ExternalTime(p) => AnppPacket::ExternalTime(p.clone().into()),
            Packet::PacketTimerPeriod(p) => AnppPacket::PacketTimerPeriod(p.clone().into()),
            Packet::PacketsPeriod(p) => AnppPacket::PacketsPeriod(p.clone().into()),
            Packet::InstallationAlignment(p) => AnppPacket::InstallationAlignment(p.clone().into()),
            Packet::FilterOptions(p) => AnppPacket::FilterOptions(p.clone().into()),
            Packet::OdometerConfiguration(p) => AnppPacket::OdometerConfiguration(p.clone().into()),
            Packet::SetZeroOrientationAlignment(p) => AnppPacket::SetZeroOrientationAlignment(p.clone().into()),
            Packet::ReferencePointOffsets(p) => AnppPacket::ReferencePointOffsets(p.clone().into()),
            Packet::IpDataportsConfiguration(p) => AnppPacket::IpDataportsConfiguration(p.clone().into()),
            _ => return Err(crate::error::AnError::InvalidPacket("Cannot send read-only or unsupported packet types".to_string())),
        };

        crate::protocol::AnppProtocol::get_bytes_from_packet(wire_packet)
    }
}
