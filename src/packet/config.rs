use crate::packet::{HasPacketId, PacketKind, gpio::{AuxiliaryFunction, GpioFunction, GpioVoltage}};
use binrw::{binrw, BinRead, BinWrite};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 3D offset vector for installation alignment
#[derive(Debug, Clone, Copy, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for OffsetVector {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<[f64; 3]> for OffsetVector {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

/// Vehicle type enumeration for filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum VehicleType {
    Unlimited = 0,
    BicycleOrMotorcycle = 1,
    Car = 2,
    Hovercraft = 3,
    Submarine = 4,
    Underwater3D = 5,
    FixedWingPlane = 6,
    Aircraft3D = 7,
    Human = 8,
    Boat = 9,
    LargeShip = 10,
    Stationary = 11,
    StuntPlane = 12,
    RaceCar = 13,
    Train = 14,
}

/// Offset type for dual antenna configuration
#[deprecated(note = "recent firmware versions report zero")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u16)]
pub enum OffsetType {
    Manual = 0,
    Automatic = 1,
}

/// Automatic offset orientation for dual antenna configuration
#[deprecated(note = "unused on recent firmware versions")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum AutomaticOffsetOrientation {
    PrimaryFrontSecondaryRear = 0,
    PrimaryRearSecondaryFront = 1,
    PrimaryRightSecondaryLeft = 2,
    PrimaryLeftSecondaryRight = 3,
}

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 2,
    TcpClient = 3,
    UdpClient = 4,
}

/// Input mode for a serial port in GpioOutputConfiguration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum PortInputMode {
    Inactive = 0,
    Nmea0183 = 6,
    Anpp = 11,
    GnssReceiverPassthrough = 38,
}

/// Output mode for a serial port in GpioOutputConfiguration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum PortOutputMode {
    Inactive = 0,
    Nmea0183 = 7,
    Anpp = 12,
    GnssReceiverPassthrough = 38,
    Tss1 = 39,
    Simrad1000 = 40,
    Simrad3000 = 41,
}

/// NMEA fix behaviour for a port in GpioOutputConfiguration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum NmeaFixBehaviour {
    Normal = 0,
    /// Always indicate 3D fix when the navigation filter is initialized
    AlwaysIndicate3dFix = 1,
}

/// Output rate for a single NMEA sentence on a port
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum PortOutputRate {
    Disabled = 0,
    Rate0_1Hz = 1,
    Rate0_2Hz = 2,
    Rate0_5Hz = 3,
    Rate1Hz = 4,
    Rate2Hz = 5,
    Rate5Hz = 6,
    Rate10Hz = 7,
    Rate25Hz = 8,
    Rate50Hz = 9,
    Rate8Hz = 10,
}

/// CAN protocol selection for CanConfiguration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum CanProtocol {
    CanOpen = 0,
}

// ===========================================================================
// Serde helpers for Duration fields
// ===========================================================================

mod duration_as_millis {
    use std::time::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de> {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

mod duration_as_millis_u16 {
    use std::time::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_u16(duration.as_millis() as u16)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de> {
        let millis = u16::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis as u64))
    }
}

// ===========================================================================
// Packet Structs
// ===========================================================================

/// Packet period entry used within PacketsPeriod
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PacketPeriod {
    #[br(map = |x: u8| PacketKind::from(x))]
    #[bw(map = |x: &PacketKind| x.packet_id())]
    pub packet_type: PacketKind,
    /// Period in units of the packet timer period
    #[br(map = |x: u32| Duration::from_millis(x as u64))]
    #[bw(map = |x: &Duration| x.as_millis() as u32)]
    #[serde(with = "duration_as_millis")]
    pub period: Duration,
}

impl PacketPeriod {
    /// Create a packet period from a specific packet type
    pub fn from_packet<P: HasPacketId>(period: Duration) -> Self {
        Self {
            packet_type: PacketKind::from(P::PACKET_ID.as_u8()),
            period,
        }
    }
}

/// Packet timer period packet (Packet ID 180, Length 4) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PacketTimerPeriod {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub utc_synchronisation: bool,
    #[br(map = |x: u16| Duration::from_millis(x as u64))]
    #[bw(map = |x: &Duration| x.as_millis() as u16)]
    #[serde(with = "duration_as_millis_u16")]
    pub packet_timer_period: Duration,
}

/// Packets period packet (Packet ID 181, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PacketsPeriod {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// When true, deletes any existing packet rates; when false, existing packet rates remain
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub clear_existing: bool,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = super::write_vec)]
    pub packet_periods: Vec<PacketPeriod>,
}

/// Baud rates packet (Packet ID 182, Length 17) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaudRates {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Primary RS232/RS422 baud rate; bit 31 selects RS422 protocol
    pub primary_baud_rate: u32,
    /// GPIO 1 and 2 baud rate
    pub gpio_baud_rate: u32,
    /// Auxiliary RS232/RS422 baud rate; bit 31 selects RS422 protocol
    pub auxiliary_baud_rate: u32,
    #[br(temp)]
    #[bw(calc = [0u8; 4])]
    _reserved: [u8; 4],
}

/// Installation alignment packet (Packet ID 185, Length 73) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct InstallationAlignment {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    pub alignment_dcm: [[f32; 3]; 3],
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}

/// Filter options packet (Packet ID 186, Length 17) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptions {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    pub vehicle_type: VehicleType,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub internal_gnss_enabled: bool,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved1: u8,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub atmospheric_altitude_enabled: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub velocity_heading_enabled: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub reversing_detection_enabled: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub motion_analysis_enabled: bool,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved2: u8,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub dual_antenna_disabled: bool,
    #[br(temp)]
    #[bw(calc = [0u8; 7])]
    _reserved3: [u8; 7],
}

/// GPIO configuration packet (Packet ID 188, Length 13) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpioConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// GPIO 1 function
    pub gpio1_function: GpioFunction,
    /// GPIO 2 function
    pub gpio2_function: GpioFunction,
    /// Auxiliary RS232 transmit function
    pub auxiliary_tx_function: AuxiliaryFunction,
    /// Auxiliary RS232 receive function
    pub auxiliary_rx_function: AuxiliaryFunction,
    /// GPIO voltage selection
    pub gpio_voltage: GpioVoltage,
    #[br(temp)]
    #[bw(calc = [0u8; 7])]
    _reserved: [u8; 7],
}

/// Odometer configuration packet (Packet ID 192, Length 8) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerConfiguration {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub automatic_pulse_measurement: bool,
    #[br(temp)]
    #[bw(calc = 0u16)]
    _reserved: u16,
    pub pulse_length: f32,
}

/// Set zero orientation alignment packet (Packet ID 193, Length 5) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetZeroOrientationAlignment {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    #[br(temp)]
    #[bw(calc = 0x9A4E8055u32)]
    _verification: u32,
}

/// Reference point offsets packet (Packet ID 194, Length 49) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ReferencePointOffsets {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    pub heave_point_1: OffsetVector,
    pub heave_point_2: OffsetVector,
    pub heave_point_3: OffsetVector,
    pub heave_point_4: OffsetVector,
}

/// NMEA sentence output rates for a single port
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct NmeaOutputRates {
    /// GPZDA time and date rate
    pub gpzda: PortOutputRate,
    /// GPGGA GPS fix data rate
    pub gpgga: PortOutputRate,
    /// GPVTG course over ground and ground speed rate
    pub gpvtg: PortOutputRate,
    /// GPRMC recommended minimum specific GPS/transit data rate
    pub gprmc: PortOutputRate,
    /// GPHDT heading, true rate
    pub gphdt: PortOutputRate,
    /// GPGLL geographic position, latitude and longitude rate
    pub gpgll: PortOutputRate,
    /// PASHR proprietary roll, pitch, and heading rate
    pub pashr: PortOutputRate,
    /// TSS1 heading and motion data rate
    pub tss1: PortOutputRate,
    /// Simrad heading and motion data rate
    pub simrad: PortOutputRate,
    /// GPROT rate of turn rate
    pub gprot: PortOutputRate,
    /// GPHEV heave rate
    pub gphev: PortOutputRate,
    /// GPGSV satellites in view rate
    pub gpgsv: PortOutputRate,
    /// PFECAtt attitude rate
    pub pfecatt: PortOutputRate,
    /// PFECHve heave rate
    pub pfechve: PortOutputRate,
    /// GPGST position error statistics rate
    pub gpgst: PortOutputRate,
}

/// Configuration for a single serial port in GpioOutputConfiguration
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortConfiguration {
    /// Input mode for this port
    pub input_mode: PortInputMode,
    /// Output mode for this port
    pub output_mode: PortOutputMode,
    /// NMEA fix behaviour for this port
    pub nmea_fix_behaviour: NmeaFixBehaviour,
    /// NMEA sentence output rates for this port
    pub output_rates: NmeaOutputRates,
    #[br(temp)]
    #[bw(calc = [0u8; 8])]
    _reserved: [u8; 8],
}

/// GPIO output configuration packet (Packet ID 195, Length 183) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpioOutputConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Auxiliary RS232 port configuration
    pub auxiliary_port: PortConfiguration,
    /// GPIO port configuration
    pub gpio_port: PortConfiguration,
    /// Logging port configuration
    pub logging_port: PortConfiguration,
    /// Data port 1 configuration
    pub data_port_1: PortConfiguration,
    /// Data port 2 configuration
    pub data_port_2: PortConfiguration,
    /// Data port 3 configuration
    pub data_port_3: PortConfiguration,
    /// Data port 4 configuration
    pub data_port_4: PortConfiguration,
}

/// IP dataport configuration entry
#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct IpDataport {
    pub ip_address: u32,
    pub port: u16,
    pub mode: IpDataportMode,
}

/// Dual antenna configuration packet (Packet ID 196, Length 17) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DualAntennaConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Offset type
    #[deprecated(note = "recent firmware versions report zero")]
    pub offset_type: OffsetType,
    /// Automatic offset orientation; ignored when using manual offset
    #[deprecated(note = "unused on recent firmware versions")]
    pub automatic_offset_orientation: AutomaticOffsetOrientation,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
    /// Manual offset X in meters, measured from secondary to primary antenna in the body frame
    pub manual_offset_x: f32,
    /// Manual offset Y in meters, measured from secondary to primary antenna in the body frame
    pub manual_offset_y: f32,
    /// Manual offset Z in meters, measured from secondary to primary antenna in the body frame
    pub manual_offset_z: f32,
}

bitflags! {
    /// GNSS frequency enable flags for GnssConfiguration
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GnssFrequencies: u64 {
        const GPS_L1CA = 1 << 0;
        const GPS_L1C = 1 << 1;
        const GPS_L1P = 1 << 2;
        const GPS_L2C = 1 << 3;
        const GPS_L2P = 1 << 4;
        const GPS_L2M = 1 << 5;
        const GPS_L5 = 1 << 6;
        const GLONASS_G1CA = 1 << 7;
        const GLONASS_G1P = 1 << 8;
        const GLONASS_L1OC = 1 << 9;
        const GLONASS_L1SC = 1 << 10;
        const GLONASS_G2CA = 1 << 11;
        const GLONASS_G2P = 1 << 12;
        const GLONASS_L2OC = 1 << 13;
        const GLONASS_L2SC = 1 << 14;
        const GLONASS_L3OC = 1 << 15;
        const GLONASS_L3SC = 1 << 16;
        const BEIDOU_B1 = 1 << 17;
        const BEIDOU_B2 = 1 << 18;
        const BEIDOU_B3 = 1 << 19;
        const GALILEO_E1 = 1 << 20;
        const GALILEO_E5A = 1 << 21;
        const GALILEO_E5B = 1 << 22;
        const GALILEO_E5AB = 1 << 23;
        const GALILEO_E6 = 1 << 24;
        const QZSS_L1CA = 1 << 25;
        const QZSS_L1SAIF = 1 << 26;
        const QZSS_L1C = 1 << 27;
        const QZSS_L2C = 1 << 28;
        const QZSS_L5 = 1 << 29;
        const QZSS_LEX = 1 << 30;
        const SBAS_L1CA = 1 << 31;
        const SBAS_L5 = 1 << 32;
        const NAVIC_L5 = 1 << 33;
        const NAVIC_S1 = 1 << 34;
        const BEIDOU_B2A = 1 << 35;
    }
}

/// GNSS configuration packet (Packet ID 197, Length 85) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// GNSS frequencies bitfield; each bit enables tracking of a specific constellation frequency
    #[br(map = GnssFrequencies::from_bits_retain)]
    #[bw(map = |x: &GnssFrequencies| x.bits())]
    pub gnss_frequencies: GnssFrequencies,
    #[br(temp)]
    #[bw(calc = [0u8; 76])]
    _reserved: [u8; 76],
}

/// User data packet (Packet ID 198, Length 64) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct UserData {
    #[serde(with = "serde_bytes_64")]
    pub data: [u8; 64],
}

mod serde_bytes_64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_bytes(data)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where D: Deserializer<'de> {
        let v = <Vec<u8>>::deserialize(deserializer)?;
        v.try_into().map_err(|_| serde::de::Error::custom("expected 64 bytes"))
    }
}

/// GPIO input configuration packet (Packet ID 199, Length 65) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpioInputConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Gimbal radians per encoder tick
    pub gimbal_radians_per_encoder_tick: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 60])]
    _reserved: [u8; 60],
}

/// IP dataports configuration packet (Packet ID 202, Length 30) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfiguration {
    #[br(temp)]
    #[bw(calc = 0u16)]
    _reserved: u16,
    pub dataports: [IpDataport; 4],
}

/// CAN configuration packet (Packet ID 203, Length 11) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanConfiguration {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// CAN interface enabled
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub enabled: bool,
    /// Baud rate
    pub baud_rate: u32,
    /// CAN protocol
    pub protocol: CanProtocol,
    #[br(temp)]
    #[bw(calc = [0u8; 4])]
    _reserved: [u8; 4],
}

bitflags! {
    /// Aiding source enable flags for AidingSourceConfiguration1
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AidingSourceBitmask1: u16 {
        const INTERNAL_GNSS_PVT = 1 << 0;
        const INTERNAL_GNSS_ORIENTATION = 1 << 1;
        const INTERNAL_MAGNETOMETERS = 1 << 2;
        const INTERNAL_PRESSURE = 1 << 3;
        const EXTERNAL_GNSS_PVT = 1 << 4;
        const EXTERNAL_GNSS_ORIENTATION = 1 << 5;
        const EXTERNAL_POSITION = 1 << 6;
        const EXTERNAL_ODOMETER = 1 << 7;
        const EXTERNAL_HEADING = 1 << 8;
        const EXTERNAL_PRESSURE = 1 << 9;
        const EXTERNAL_VELOCITY = 1 << 10;
        const EXTERNAL_POSITION_VELOCITY = 1 << 11;
        const EXTERNAL_BODY_VELOCITY = 1 << 12;
        const EXTERNAL_AIR_DATA = 1 << 13;
        const EXTERNAL_MAGNETOMETERS = 1 << 14;
        const EXTERNAL_LVS = 1 << 15;
    }
}

/// Aiding source configuration packet 1 (Packet ID 207, Length 243) - Read/Write
///
/// A position offset is the position of the sensor from the INS defined body
/// point, in body coordinates. Updates to the Installation Alignment packet's
/// GNSS antenna, odometer, and external data offsets result in updates to the
/// corresponding fields of this packet.
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidingSourceConfiguration1 {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Enabled aiding sources
    #[br(map = AidingSourceBitmask1::from_bits_retain)]
    #[bw(map = |x: &AidingSourceBitmask1| x.bits())]
    pub enabled_sources: AidingSourceBitmask1,
    pub internal_gnss_pvt_position_offset: OffsetVector,
    pub internal_gnss_orientation_orientation_offset: OffsetVector,
    pub external_gnss_pvt_position_offset: OffsetVector,
    pub external_gnss_orientation_orientation_offset: OffsetVector,
    pub external_position_position_offset: OffsetVector,
    pub external_odometer_position_offset: OffsetVector,
    pub external_heading_orientation_offset: OffsetVector,
    pub external_pressure_position_offset: OffsetVector,
    pub external_velocity_position_offset: OffsetVector,
    pub external_position_velocity_position_offset: OffsetVector,
    pub external_body_velocity_position_offset: OffsetVector,
    pub external_body_velocity_orientation_offset: OffsetVector,
    pub external_air_data_position_offset: OffsetVector,
    pub external_air_data_orientation_offset: OffsetVector,
    pub external_magnetometers_orientation_offset: OffsetVector,
    pub external_lvs_position_offset: OffsetVector,
    pub external_lvs_orientation_offset: OffsetVector,
    #[br(temp)]
    #[bw(calc = [0u8; 36])]
    _reserved: [u8; 36],
}

bitflags! {
    /// Aiding source enable flags for AidingSourceConfiguration2
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AidingSourceBitmask2: u16 {
        const INTERNAL_DEPTH_SENSOR = 1 << 0;
        const EXTERNAL_SUBSONUS = 1 << 1;
        const EXTERNAL_DVL_DATA = 1 << 2;
        const EXTERNAL_DEPTH = 1 << 3;
        const EXTERNAL_USBL = 1 << 4;
    }
}

/// Aiding source configuration packet 2 (Packet ID 208, Length 243) - Read/Write
///
/// A position offset is the position of the sensor from the INS defined body
/// point, in body coordinates. Updates to the Installation Alignment packet's
/// odometer and external data offsets result in updates to the corresponding
/// fields of this packet.
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidingSourceConfiguration2 {
    /// Whether the configuration is saved to non-volatile memory
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    /// Enabled aiding sources
    #[br(map = AidingSourceBitmask2::from_bits_retain)]
    #[bw(map = |x: &AidingSourceBitmask2| x.bits())]
    pub enabled_sources: AidingSourceBitmask2,
    pub internal_depth_sensor_position_offset: OffsetVector,
    pub external_subsonus_position_offset: OffsetVector,
    pub external_subsonus_orientation_offset: OffsetVector,
    pub external_dvl_data_position_offset: OffsetVector,
    pub external_dvl_data_orientation_offset: OffsetVector,
    pub external_depth_position_offset: OffsetVector,
    pub external_usbl_position_offset: OffsetVector,
    #[br(temp)]
    #[bw(calc = [0u8; 156])]
    _reserved: [u8; 156],
}

#[cfg(test)]
#[path = "tests/config.rs"]
mod config_length_tests;
