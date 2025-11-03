
use binrw::{BinRead, BinWrite};
use crate::{Result, error::AnError};
pub mod system;
pub mod state;
pub mod config;
pub mod flags;

/// ANPP packet identifier structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite)]
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
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct AnppHeader {
    pub header_lrc: u8,
    pub packet_id: PacketId,
    pub length: u8,
    pub crc16: u16,
}
pub trait Packet {
    const PACKET_ID: PacketId;
}

// Import packet types from their respective modules
use system::{AcknowledgePacket, RequestPacket, BootModePacket, DeviceInformationPacket,
            RestoreFactorySettingsPacket, ResetPacket, IpConfigurationPacket};
use state::{SystemStatePacket, UnixTimePacket, StatusPacket, EulerOrientationStdDevPacket, RawSensorsPacket};
use config::{PacketTimerPeriodPacket, PacketsPeriodPacket, InstallationAlignmentPacket,
            FilterOptionsPacket, OdometerConfigurationPacket, SetZeroOrientationAlignmentPacket,
            ReferencePointOffsetsPacket, IpDataportsConfigurationPacket};

macro_rules! define_packets {
    ( $( $variant:ident => $code:expr, $length:expr ),+ $(,)? ) => {
        paste::paste! {
            $(
                impl Packet for [<$variant Packet>] {
                    const PACKET_ID: PacketId = PacketId { id: $code };
                }
            )+

            /// Core enum that represents the packet kind
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            }

            impl From<u8> for PacketKind {
                fn from(id: u8) -> Self {
                    match id {
                        $( $code => PacketKind::$variant, )+
                        _ => PacketKind::Unsupported,
                    }
                }
            }

            /// Detailed enum that holds the associated payload
            #[derive(Debug, Clone)]
            pub enum AnppPacket {
                $( $variant([<$variant Packet>]), )+
                Unsupported(Vec<u8>),
            }

            impl AnppPacket {
                /// Get the packet kind for this packet
                pub fn kind(&self) -> PacketKind {
                    match self {
                        $( AnppPacket::$variant(_) => PacketKind::$variant, )+
                        AnppPacket::Unsupported(_) => PacketKind::Unsupported,
                    }
                }

                /// Get the packet ID for this packet
                pub fn packet_id(&self) -> u8 {
                    match self {
                        $( AnppPacket::$variant(_) => $code, )+
                        AnppPacket::Unsupported(_) => 0xFF,
                    }
                }

                /// Get the expected byte length for this packet
                pub fn byte_length(&self) -> Option<usize> {
                    self.kind().byte_length()
                }

                /// Parse a packet from raw bytes
                pub fn from_bytes(packet_id: u8, data: &[u8]) -> Result<Self> {
                    use binrw::BinRead;
                    use std::io::Cursor;

                    let packet = match PacketKind::from(packet_id) {
                        $(
                            PacketKind::$variant => {
                                let mut cursor = Cursor::new(data);
                                AnppPacket::$variant([<$variant Packet>]::read_le(&mut cursor)
                                    .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize {}: {}", stringify!([<$variant Packet>]), e)))?)
                            },
                        )+
                        PacketKind::Unsupported => AnppPacket::Unsupported(data.to_vec()),
                    };
                    Ok(packet)
                }

                /// Convert packet to bytes
                pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
                    use binrw::BinWrite;
                    use std::io::Cursor;

                    match self {
                        $(
                            AnppPacket::$variant(p) => {
                                let mut cursor = Cursor::new(Vec::new());
                                p.write_le(&mut cursor)
                                    .map_err(|e| crate::error::AnError::InvalidPacket(format!("Failed to serialize {}: {}", stringify!([<$variant Packet>]), e)))?;
                                Ok(cursor.into_inner())
                            },
                        )+
                        AnppPacket::Unsupported(data) => Ok(data.clone()),
                    }
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

    // State Packets (20-28)
    SystemState => 20, Some(100),
    UnixTime => 21, Some(8),
    Status => 23, Some(4),
    EulerOrientationStdDev => 26, Some(12),
    RawSensors => 28, Some(48),

    // Configuration Packets (180-203)
    PacketTimerPeriod => 180, Some(4),
    PacketsPeriod => 181, None,
    InstallationAlignment => 185, Some(73),
    FilterOptions => 186, Some(17),
    OdometerConfiguration => 192, Some(8),
    SetZeroOrientationAlignment => 193, Some(1),
    ReferencePointOffsets => 194, Some(13),
    IpDataportsConfiguration => 202, Some(30),
);


