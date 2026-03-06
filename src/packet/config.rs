use binrw::{binrw, BinRead, BinWrite};
use serde::{Serialize, Deserialize};
use std::time::Duration;

use crate::packet::{PacketKind, HasPacketId};

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

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 2,
    TcpClient = 3,
    UdpClient = 4,
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
    #[br(temp)]
    #[bw(calc = [0u8; 8])]
    _reserved3: [u8; 8],
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

/// Set zero orientation alignment packet (Packet ID 193, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SetZeroOrientationAlignment {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub permanent: bool,
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

/// IP dataport configuration entry
#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct IpDataport {
    pub ip_address: u32,
    pub port: u16,
    pub mode: IpDataportMode,
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
