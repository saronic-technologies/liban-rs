use crate::packet::{HasPacketId, PacketKind, gpio::{AuxiliaryFunction, GpioFunction, GpioVoltage}};
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 3D offset vector for installation alignment
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u16)]
pub enum OffsetType {
    Manual = 0,
    Automatic = 1,
}

/// Automatic offset orientation for dual antenna configuration
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
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub clear_existing: bool,
    #[br(parse_with = |reader, _endian, _args: ()| -> binrw::BinResult<Vec<PacketPeriod>> {
        let mut entries = Vec::new();
        while let Ok(entry) = PacketPeriod::read_le(reader) {
            entries.push(entry);
        }
        Ok(entries)
    })]
    #[bw(write_with = |entries: &Vec<PacketPeriod>, writer, _endian, _args: ()| -> binrw::BinResult<()> {
        for entry in entries {
            entry.write_le(writer)?;
        }
        Ok(())
    })]
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
    pub offset_type: OffsetType,
    /// Automatic offset orientation; ignored when using manual offset
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

#[cfg(test)]
#[path = "tests/config.rs"]
mod tests;
