
use crate::{Result, error::AnError};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use std::io::{Seek, Write};

pub mod config;
pub mod gpio;
pub mod satellite;
pub mod state;
pub mod system;

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

/// Describes the expected payload length for a packet kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketLength {
    /// Exactly one valid length.
    Fixed(usize),
    /// A fixed set of valid lengths (e.g. optional trailing fields).
    OneOf(&'static [usize]),
    /// Length varies arbitrarily; no header check performed.
    Variable,
}

use PacketLength::*;

/// Writes each element of a `Vec<T>` in sequence, for use with `#[bw(write_with = ...)]`.
// binrw's write_with macro passes &Vec<T> by field type, so &[T] is not accepted here
#[allow(clippy::ptr_arg)]
pub(crate) fn write_vec<W, T>(data: &Vec<T>, writer: &mut W, endian: Endian, _args: ()) -> BinResult<()>
where
    W: Write + Seek,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    data.iter().try_for_each(|item| item.write_options(writer, endian, ()))
}

// Import packet types from their respective modules
use system::{Acknowledge, Request, BootMode, DeviceInformation,
            RestoreFactorySettings, Reset, SerialPortPassthrough, IpConfiguration,
            SubcomponentInformation};
use state::{SystemState, UnixTime, FormattedTime, Status, PositionStdDev, VelocityStdDev,
            EulerOrientationStdDev, QuaternionOrientationStdDev,
            RawSensors, RawGnss, Satellites,
            GeodeticPosition, EcefPosition, UtmPosition, NedVelocity, BodyVelocity,
            Acceleration, BodyAcceleration, EulerOrientation, QuaternionOrientation,
            DcmOrientation, AngularVelocity, AngularAcceleration,
            ExternalPositionVelocity, ExternalPosition, ExternalVelocity,
            ExternalBodyVelocity, ExternalHeading,
            RunningTime, OdometerState, ExternalTime, ExternalDepth, GeoidHeight, RtcmCorrections,
            Wind, Heave, RawSatelliteData, RawSatelliteEphemeris,
            ExternalOdometer, ExternalAirData, GimbalState, Automotive,
            ExtendedSatellites,
            NorthSeekingInitialisationStatus, RawDvlData,
            GnssReceiverInformation, ZeroAngularVelocity, SensorTemperature, SystemTemperature,
            VesselMotion,
            GnssPositionVelocityTime, GnssOrientation};
use config::{BaudRates, CanConfiguration, DualAntennaConfiguration, FilterOptions,
            GnssConfiguration, GpioConfiguration, GpioInputConfiguration, GpioOutputConfiguration,
            InstallationAlignment, IpDataportsConfiguration, OdometerConfiguration,
            PacketTimerPeriod, PacketsPeriod, ReferencePointOffsets,
            SetZeroOrientationAlignment, UserData};

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
            /// Get the expected payload length for this packet kind
            pub fn byte_length(&self) -> PacketLength {
                match self {
                    $( PacketKind::$variant => $length, )+
                    PacketKind::Unsupported => Variable,
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
    Acknowledge => 0, Fixed(4),
    Request => 1, Fixed(1),
    BootMode => 2, Fixed(1),
    DeviceInformation => 3, Fixed(24),
    RestoreFactorySettings => 4, Fixed(4),
    Reset => 5, Fixed(4),
    SerialPortPassthrough => 10, Variable,
    IpConfiguration => 11, Fixed(30),
    SubcomponentInformation => 14, Variable,

    // State Packets (20-93)
    SystemState => 20, Fixed(100),
    UnixTime => 21, Fixed(8),
    FormattedTime => 22, Fixed(14),
    Status => 23, Fixed(4),
    PositionStdDev => 24, Fixed(12),
    VelocityStdDev => 25, Fixed(12),
    EulerOrientationStdDev => 26, Fixed(12),
    QuaternionOrientationStdDev => 27, Fixed(16),
    RawSensors => 28, Fixed(48),
    RawGnss => 29, Fixed(74),
    Satellites => 30, Fixed(13),
    GeodeticPosition => 32, Fixed(24),
    EcefPosition => 33, Fixed(24),
    UtmPosition => 34, Fixed(26),
    NedVelocity => 35, Fixed(12),
    BodyVelocity => 36, Fixed(12),
    Acceleration => 37, Fixed(12),
    BodyAcceleration => 38, Fixed(16),
    EulerOrientation => 39, Fixed(12),
    QuaternionOrientation => 40, Fixed(16),
    DcmOrientation => 41, Fixed(36),
    AngularVelocity => 42, Fixed(12),
    AngularAcceleration => 43, Fixed(12),
    ExternalPositionVelocity => 44, Fixed(60),
    ExternalPosition => 45, Fixed(36),
    ExternalVelocity => 46, Fixed(24),
    ExternalBodyVelocity => 47, OneOf(&[16, 24]),
    ExternalHeading => 48, Fixed(8),
    RunningTime => 49, Fixed(8),
    OdometerState => 51, Fixed(20),
    ExternalTime => 52, Fixed(8),
    ExternalDepth => 53, Fixed(8),
    GeoidHeight => 54, Fixed(4),
    RtcmCorrections => 55, Variable,
    Wind => 57, Fixed(12),
    Heave => 58, Fixed(16),
    RawSatelliteData => 60, Variable,
    RawSatelliteEphemeris => 61, OneOf(&[94, 132]),
    ExternalOdometer => 67, Fixed(13),
    ExternalAirData => 68, Fixed(25),
    GnssReceiverInformation => 69, Fixed(68),
    NorthSeekingInitialisationStatus => 71, Fixed(28),
    GimbalState => 72, Fixed(8),
    Automotive => 73, Fixed(24),
    RawDvlData => 70, Fixed(60),
    ZeroAngularVelocity => 83, Fixed(8),
    ExtendedSatellites => 84, Variable,
    SensorTemperature => 85, Fixed(32),
    SystemTemperature => 86, Fixed(64),
    VesselMotion => 89, Fixed(48),
    GnssPositionVelocityTime => 92, Fixed(76),
    GnssOrientation => 93, Fixed(36),

    // Configuration Packets (180-203)
    PacketTimerPeriod => 180, Fixed(4),
    PacketsPeriod => 181, Variable,
    BaudRates => 182, Fixed(17),
    InstallationAlignment => 185, Fixed(73),
    FilterOptions => 186, Fixed(17),
    GpioConfiguration => 188, Fixed(13),
    OdometerConfiguration => 192, Fixed(8),
    SetZeroOrientationAlignment => 193, Fixed(5),
    ReferencePointOffsets => 194, Fixed(49),
    GpioOutputConfiguration => 195, Fixed(183),
    DualAntennaConfiguration => 196, Fixed(17),
    GnssConfiguration => 197, Fixed(85),
    UserData => 198, Fixed(64),
    GpioInputConfiguration => 199, Fixed(65),
    IpDataportsConfiguration => 202, Fixed(30),
    CanConfiguration => 203, Fixed(11),
);

impl Packet {
    /// Convert packet to wire format bytes ready to send (with ANPP framing)
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        match self {
            Packet::Request(_) | Packet::BootMode(_) |
            Packet::RestoreFactorySettings(_) | Packet::Reset(_) |
            Packet::SerialPortPassthrough(_) |
            Packet::IpConfiguration(_) |
            Packet::ExternalPositionVelocity(_) | Packet::ExternalPosition(_) |
            Packet::ExternalVelocity(_) | Packet::ExternalBodyVelocity(_) |
            Packet::ExternalHeading(_) | Packet::ExternalTime(_) |
            Packet::RtcmCorrections(_) |
            Packet::PacketTimerPeriod(_) | Packet::PacketsPeriod(_) |
            Packet::InstallationAlignment(_) | Packet::FilterOptions(_) |
            Packet::GpioConfiguration(_) |
            Packet::OdometerConfiguration(_) | Packet::SetZeroOrientationAlignment(_) |
            Packet::ReferencePointOffsets(_) | Packet::GpioOutputConfiguration(_) |
            Packet::DualAntennaConfiguration(_) | Packet::GnssConfiguration(_) |
            Packet::GpioInputConfiguration(_) | Packet::CanConfiguration(_) |
            Packet::UserData(_) |
            Packet::IpDataportsConfiguration(_) => {
                let packet_id = PacketId::new(self.packet_id());
                let data = self.payload_bytes()?;
                crate::protocol::AnppProtocol::get_packet_bytes(packet_id, &data)
            }
            _ => Err(crate::error::AnError::InvalidPacket("Cannot send read-only or unsupported packet types".to_string())),
        }
    }
}
