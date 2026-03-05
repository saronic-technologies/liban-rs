
use binrw::{BinRead, BinWrite};
use serde::{Serialize, Deserialize};
use crate::{Result, error::AnError};
pub mod system;
pub mod state;
pub mod config;

/// ANPP packet identifier structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PacketId {
    pub id: u8,
}

impl PacketId {
    /// Get the packet type for this ID
    pub fn packet_type(&self) -> PacketKind {
        PacketKind::from(self.id)
    }

    /// Create a new PacketId from a u8 value
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    /// Get the u8 value of the PacketId
    pub fn as_u8(&self) -> u8 {
        self.id
    }
}

/// ANPP packet header structure
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AnppHeader {
    pub header_lrc: u8,
    pub packet_id: PacketId,
    pub length: u8,
    pub crc16: u16,
}

pub trait HasPacketId {
    const PACKET_ID: PacketId;
}

// Import packet types from their respective modules
use system::{Acknowledge, Request, BootMode, DeviceInformation,
            RestoreFactorySettings, Reset, IpConfiguration};
use state::{SystemState, UnixTime, Status, PositionStdDev, VelocityStdDev,
            EulerOrientationStdDev, RawSensors, Satellites, ExternalTime, Heave,
            SensorTemperature, GnssPositionVelocityTime, GnssOrientation};
use config::{PacketTimerPeriod, PacketsPeriod, InstallationAlignment,
            FilterOptions, OdometerConfiguration, SetZeroOrientationAlignment,
            ReferencePointOffsets, IpDataportsConfiguration};

macro_rules! define_packets {
    ( $( $variant:ident => $code:expr, $length:expr ),+ $(,)? ) => {
        $(
            impl HasPacketId for $variant {
                const PACKET_ID: PacketId = PacketId { id: $code };
            }
        )+

        /// Core enum that represents the packet kind
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum PacketKind {
            $( $variant, )+
            Unsupported,
        }

        impl PacketKind {
            /// Get the expected byte length for this packet kind
            pub fn byte_length(&self) -> Option<usize> {
                match self {
                    $( PacketKind::$variant => $length, )+
                    PacketKind::Unsupported => None,
                }
            }

            /// Get the packet ID for this packet kind
            pub fn packet_id(&self) -> u8 {
                match self {
                    $( PacketKind::$variant => $code, )+
                    PacketKind::Unsupported => 0xFF,
                }
            }
        }

        impl From<u8> for PacketKind {
            fn from(id: u8) -> Self {
                match id {
                    $( $code => PacketKind::$variant, )+
                    _ => PacketKind::Unsupported,
                }
            }
        }

        /// Packet enum — the single public type for all ANPP packets.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum Packet {
            $( $variant($variant), )+
            Unsupported(Vec<u8>),
        }

        impl Packet {
            /// Get the packet ID
            pub fn packet_id(&self) -> u8 {
                match self {
                    $( Packet::$variant(_) => $code, )+
                    Packet::Unsupported(_) => 0xFF,
                }
            }

            /// Parse a packet from raw bytes
            pub(crate) fn from_bytes(packet_id: u8, data: &[u8]) -> Result<Self> {
                use binrw::BinRead;
                use std::io::Cursor;

                let packet = match PacketKind::from(packet_id) {
                    $(
                        PacketKind::$variant => {
                            let mut cursor = Cursor::new(data);
                            Packet::$variant($variant::read_le(&mut cursor)
                                .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize {}: {}", stringify!($variant), e)))?)
                        },
                    )+
                    PacketKind::Unsupported => Packet::Unsupported(data.to_vec()),
                };
                Ok(packet)
            }

            /// Serialize just the payload (no ANPP framing)
            pub(crate) fn payload_bytes(&self) -> crate::Result<Vec<u8>> {
                use binrw::BinWrite;
                use std::io::Cursor;

                match self {
                    $(
                        Packet::$variant(p) => {
                            let mut cursor = Cursor::new(Vec::new());
                            p.write_le(&mut cursor)
                                .map_err(|e| crate::error::AnError::InvalidPacket(format!("Failed to serialize {}: {}", stringify!($variant), e)))?;
                            Ok(cursor.into_inner())
                        },
                    )+
                    Packet::Unsupported(data) => Ok(data.clone()),
                }
            }
        }
    };
}

define_packets!(
    // System Packets (0-14)
    Acknowledge => 0, Some(4),
    Request => 1, Some(1),
    BootMode => 2, Some(1),
    DeviceInformation => 3, Some(24),
    RestoreFactorySettings => 4, Some(4),
    Reset => 5, Some(4),
    IpConfiguration => 11, Some(30),

    // State Packets (20-93)
    SystemState => 20, Some(100),
    UnixTime => 21, Some(8),
    Status => 23, Some(4),
    PositionStdDev => 24, Some(12),
    VelocityStdDev => 25, Some(12),
    EulerOrientationStdDev => 26, Some(12),
    RawSensors => 28, Some(48),
    Satellites => 30, Some(13),
    ExternalTime => 52, Some(8),
    Heave => 58, Some(16),
    SensorTemperature => 85, Some(32),
    GnssPositionVelocityTime => 92, Some(76),
    GnssOrientation => 93, Some(36),

    // Configuration Packets (180-203)
    PacketTimerPeriod => 180, Some(4),
    PacketsPeriod => 181, None,
    InstallationAlignment => 185, Some(73),
    FilterOptions => 186, Some(17),
    OdometerConfiguration => 192, Some(8),
    SetZeroOrientationAlignment => 193, Some(1),
    ReferencePointOffsets => 194, Some(49),
    IpDataportsConfiguration => 202, Some(30),
);

impl Packet {
    /// Convert packet to wire format bytes ready to send (with ANPP framing)
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        match self {
            Packet::Request(_) | Packet::BootMode(_) |
            Packet::RestoreFactorySettings(_) | Packet::Reset(_) |
            Packet::IpConfiguration(_) | Packet::ExternalTime(_) |
            Packet::PacketTimerPeriod(_) | Packet::PacketsPeriod(_) |
            Packet::InstallationAlignment(_) | Packet::FilterOptions(_) |
            Packet::OdometerConfiguration(_) | Packet::SetZeroOrientationAlignment(_) |
            Packet::ReferencePointOffsets(_) | Packet::IpDataportsConfiguration(_) => {
                let packet_id = PacketId::new(self.packet_id());
                let data = self.payload_bytes()?;
                crate::protocol::AnppProtocol::get_packet_bytes(packet_id, &data)
            }
            _ => Err(crate::error::AnError::InvalidPacket("Cannot send read-only or unsupported packet types".to_string())),
        }
    }
}
