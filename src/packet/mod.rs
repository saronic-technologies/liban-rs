use crate::{Result, error::AnError};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use std::io::{Seek, Write};

pub mod config;
pub mod gpio;
pub mod receiver;
pub mod satellite;
pub mod state;
pub mod system;

use state::{FilterStatus, GnssPvtStatus, RawGnssStatus};

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
use system::{Acknowledge, BootMode, DeviceInformation, ExtendedDeviceInformation,
            IpConfiguration, Request, Reset, RestoreFactorySettings, SerialPortPassthrough,
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

            /// Get the name of the packet variant as a static string.
            pub fn type_name(&self) -> &'static str {
                match self {
                    $( Packet::$variant(_) => stringify!($variant), )+
                    Packet::Unsupported(_) => stringify!(Unsupported),
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
    ExtendedDeviceInformation => 13, Fixed(36),
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
    GnssReceiverInformation => 69, OneOf(&[48, 68]),
    RawDvlData => 70, Fixed(60),
    NorthSeekingInitialisationStatus => 71, Fixed(28),
    GimbalState => 72, Fixed(8),
    Automotive => 73, Fixed(24),
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
    /// Return the timestamp carried by this packet as `(seconds, microseconds)`
    /// since the Unix epoch, or `None` when the packet carries no timestamp.
    ///
    /// Exceptions to the plain `(seconds, microseconds)` reading:
    /// - `SystemState`, `RawGnss`, and `GnssPositionVelocityTime` return `None`
    ///   when the device flags their time fields invalid.
    /// - `RawSatelliteData` reports nanoseconds, rounded here to the nearest
    ///   microsecond; read its `nanoseconds` field directly when that precision
    ///   matters.
    /// - `RawSatelliteEphemeris` carries no sub-second field, so its
    ///   microseconds value is always zero.
    /// - `FormattedTime` can express times before unix epoch, but `u32` does not
    ///   support negatives, so anything before 1970-01-01 is dropped.
    pub fn timestamp(&self) -> Option<(u32, u32)> {
        match self {
            Packet::SystemState(p) => p
                .filter_status
                .contains(FilterStatus::UTC_TIME_INITIALISED)
                .then_some((p.unix_time_seconds, p.microseconds)),
            Packet::UnixTime(p) => Some((p.unix_time_seconds, p.microseconds)),
            Packet::FormattedTime(p) => p
                .unix_time_seconds()
                .and_then(|t| u32::try_from(t).ok())
                .map(|s| (s, p.microseconds)),
            Packet::RawGnss(p) => p
                .status
                .contains(RawGnssStatus::TIME_VALID)
                .then_some((p.unix_time_seconds, p.microseconds)),
            Packet::ExternalTime(p) => Some((p.unix_time_seconds, p.microseconds)),
            Packet::RawDvlData(p) => Some((p.unix_time_seconds, p.microseconds)),
            Packet::GnssPositionVelocityTime(p) => p
                .status
                .contains(GnssPvtStatus::TIME_VALID)
                .then_some((p.posix_time_seconds, p.posix_time_microseconds)),
            Packet::GnssOrientation(p) => Some((p.posix_time_seconds, p.posix_time_microseconds)),
            Packet::RawSatelliteData(p) => {
                let rounded_micros = (p.nanoseconds % 1_000_000_000 + 500) / 1000;
                Some((
                    p.unix_time + rounded_micros / 1_000_000,
                    rounded_micros % 1_000_000,
                ))
            }
            Packet::RawSatelliteEphemeris(p) => Some((p.unix_time, 0)),
            _ => None,
        }
    }

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

#[cfg(test)]
mod tests {
    use crate::packet::{
        Packet,
        satellite::{EphemerisData, SatelliteSystem},
        state::{FormattedTime, RawGnss, RawGnssStatus, RawSatelliteData, RawSatelliteEphemeris},
    };

    #[test]
    fn timestamp_gated_on_validity_bit() {
        let mut gnss = RawGnss {
            unix_time_seconds: 1_700_000_000,
            microseconds: 250,
            latitude: 0.0,
            longitude: 0.0,
            height: 0.0,
            velocity_north: 0.0,
            velocity_east: 0.0,
            velocity_down: 0.0,
            latitude_std_dev: 0.0,
            longitude_std_dev: 0.0,
            height_std_dev: 0.0,
            tilt: 0.0,
            heading: 0.0,
            tilt_std_dev: 0.0,
            heading_std_dev: 0.0,
            status: RawGnssStatus::from_bits_retain(0),
        };
        assert_eq!(Packet::RawGnss(gnss.clone()).timestamp(), None);

        gnss.status |= RawGnssStatus::TIME_VALID;
        assert_eq!(Packet::RawGnss(gnss).timestamp(), Some((1_700_000_000, 250)));
    }

    #[test]
    fn timestamp_rounds_satellite_data_nanoseconds() {
        let base = RawSatelliteData {
            unix_time: 1_700_000_000,
            nanoseconds: 1_500,
            receiver_clock_offset: 0,
            receiver_number: 0,
            packet_number: 1,
            total_packets: 1,
            satellites: Vec::new(),
        };
        assert_eq!(Packet::RawSatelliteData(base.clone()).timestamp(), Some((1_700_000_000, 2)));

        let carry = RawSatelliteData { nanoseconds: 999_999_750, ..base };
        assert_eq!(Packet::RawSatelliteData(carry).timestamp(), Some((1_700_000_001, 0)));
    }

    #[test]
    fn timestamp_satellite_ephemeris_has_zero_microseconds() {
        let ephemeris = RawSatelliteEphemeris {
            unix_time: 1_700_000_000,
            satellite_system: SatelliteSystem::Gps,
            prn: 1,
            data: EphemerisData::Unknown,
        };
        assert_eq!(Packet::RawSatelliteEphemeris(ephemeris).timestamp(), Some((1_700_000_000, 0)));
    }

    #[test]
    fn timestamp_formatted_time() {
        let cases = [
            // (year, month, day, hour, minute, second), (doy, dow), expected)
            // negative time: None
            ((1969, 12, 31, 23, 59, 59), (0, 3), None),
            // epoch: OK
            ((1970, 1, 1, 0, 0, 0), (0, 4), Some(0)),
            // bad doy: None
            ((1970, 1, 1, 0, 0, 1), (1, 4), None),
            // bad dow: None
            ((1970, 1, 1, 1, 0, 0), (0, 5), None),
            // large date: OK
            ((2100, 3, 1, 0, 0, 0), (59, 1), Some(4107542400)),
        ];
        for (i, ((year, month, month_day, hour, minute, second), (year_day, week_day), expected)) in cases.into_iter().enumerate() {
            let microseconds = i as u32;
            let packet = FormattedTime {
                microseconds,
                year,
                year_day,
                month,
                month_day,
                week_day,
                hour,
                minute,
                second,
            };
            assert_eq!(Packet::FormattedTime(packet).timestamp(), expected.map(|ts| (ts, microseconds)));
        }
    }
}
