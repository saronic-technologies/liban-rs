use binrw::{BinRead, BinWrite};
use serde::{Serialize, Deserialize};

/// Configuration Packets (180-203)
/// Packet timer period packet structure (Packet ID 180, Length 4) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct PacketTimerPeriodPacket {
    pub permanent: u8,
    pub utc_synchronisation: u8,
    pub packet_timer_period: u16,
}


/// Packets period packet structure (Packet ID 181, Length: 2 + (5 x number of packet periods)) - Read/Write
/// Format: Permanent (1) | Clear Existing (1) | [Packet ID (1) | Period (4)]...
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct PacketsPeriodPacket {
    /// Permanent setting (Field 1)
    pub permanent: u8,
    /// Clear existing packet periods (Field 2)
    pub clear_existing: u8,
    /// Variable number of packet period entries (Fields 3-4 repeating)
    /// Each entry is 5 bytes: packet_id (1) + period (4)
    #[br(parse_with = |reader, _endian, _args: ()| -> binrw::BinResult<Vec<PacketPeriodEntry>> {
        let mut entries = Vec::new();
        // Read remaining entries until end of stream
        while let Ok(entry) = PacketPeriodEntry::read_le(reader) {
            entries.push(entry);
        }
        Ok(entries)
    })]
    #[bw(write_with = |entries: &Vec<PacketPeriodEntry>, writer, _endian, _args: ()| -> binrw::BinResult<()> {
        for entry in entries {
            entry.write_le(writer)?;
        }
        Ok(())
    })]
    pub packet_periods: Vec<PacketPeriodEntry>,
}


/// Entry for packets period configuration
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct PacketPeriodEntry {
    pub packet_id: u8,
    pub period: u32,
}

/// Installation alignment packet structure (Packet ID 185, Length 73) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct InstallationAlignmentPacket {
    pub permanent: u8,
    pub alignment_dcm: [[f32; 3]; 3], // 3x3 Direction Cosine Matrix
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}


/// 3D offset vector for installation alignment
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Filter options packet structure (Packet ID 186, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct FilterOptionsPacket {
    pub permanent: u8,
    pub vehicle_type: VehicleType,
    pub internal_gnss_enabled: u8,
    pub reserved1: u8,
    pub atmospheric_altitude_enabled: u8,
    pub velocity_heading_enabled: u8,
    pub reversing_detection_enabled: u8,
    pub motion_analysis_enabled: u8,
    pub reserved2: u8,
    pub reserved3: [u8; 8],
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

/// Odometer configuration packet structure (Packet ID 192, Length 8) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct OdometerConfigurationPacket {
    pub permanent: u8,
    pub automatic_pulse_measurement: u8,
    pub reserved: u16,
    pub pulse_length: f32,
}


/// Set zero orientation alignment packet structure (Packet ID 193, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct SetZeroOrientationAlignmentPacket {
    pub permanent: u8,
}


/// Reference point offsets packet structure (Packet ID 194, Length 49) - Read/Write
/// Heave point 1 is the primary reference point offset
/// Heave point 2 is used for COG Lever Arm offset
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub heave_point_1: OffsetVector,
    pub heave_point_2: OffsetVector,
    pub heave_point_3: OffsetVector,
    pub heave_point_4: OffsetVector,
}


/// IP dataports configuration packet structure (Packet ID 202, Length 30) - Read/Write
/// Contains exactly 4 dataport entries as per ANPP specification
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct IpDataportsConfigurationPacket {
    pub reserved: u16,
    pub dataports: [IpDataportEntry; 4],
}


/// IP dataport entry for IP dataports configuration
/// Fields: ip_address(u32), port(u16), mode(u8) = 7 bytes per entry
#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct IpDataportEntry {
    pub ip_address: u32,
    pub port: u16,
    pub mode: IpDataportMode,
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

// Public API Types

use std::time::Duration;
use crate::packet::{PacketKind, Packet};

/// Packet period - clean API using PacketKind enum instead of raw IDs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketPeriod {
    pub packet_type: PacketKind,
    #[serde(with = "duration_as_millis")]
    pub period: Duration,
}

mod duration_as_millis {
    use std::time::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

impl From<PacketPeriodEntry> for PacketPeriod {
    fn from(p: PacketPeriodEntry) -> Self {
        Self {
            packet_type: PacketKind::from(p.packet_id),
            period: Duration::from_millis(p.period as u64),
        }
    }
}

impl From<PacketPeriod> for PacketPeriodEntry {
    fn from(p: PacketPeriod) -> Self {
        Self {
            packet_id: p.packet_type as u8,
            period: p.period.as_millis() as u32,
        }
    }
}

impl PacketPeriod {
    /// Create a packet period from a specific packet type
    pub fn from_packet<P: Packet>(period: Duration) -> Self {
        Self {
            packet_type: PacketKind::from(P::PACKET_ID.as_u8()),
            period,
        }
    }
}

/// Packet timer period - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketTimerPeriod {
    pub permanent: bool,
    pub utc_synchronisation: bool,
    #[serde(with = "duration_as_millis_u16")]
    pub packet_timer_period: Duration,
}

mod duration_as_millis_u16 {
    use std::time::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(duration.as_millis() as u16)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u16::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis as u64))
    }
}

impl From<PacketTimerPeriodPacket> for PacketTimerPeriod {
    fn from(p: PacketTimerPeriodPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            utc_synchronisation: p.utc_synchronisation != 0,
            packet_timer_period: Duration::from_millis(p.packet_timer_period as u64),
        }
    }
}

impl From<PacketTimerPeriod> for PacketTimerPeriodPacket {
    fn from(p: PacketTimerPeriod) -> Self {
        Self {
            permanent: p.permanent as u8,
            utc_synchronisation: p.utc_synchronisation as u8,
            packet_timer_period: p.packet_timer_period.as_millis() as u16,
        }
    }
}

/// Packets period configuration - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketsPeriod {
    pub permanent: bool,
    pub clear_existing: bool,
    pub packet_periods: Vec<PacketPeriod>,
}

impl From<PacketsPeriodPacket> for PacketsPeriod {
    fn from(p: PacketsPeriodPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            clear_existing: p.clear_existing != 0,
            packet_periods: p.packet_periods.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PacketsPeriod> for PacketsPeriodPacket {
    fn from(p: PacketsPeriod) -> Self {
        Self {
            permanent: p.permanent as u8,
            clear_existing: p.clear_existing as u8,
            packet_periods: p.packet_periods.into_iter().map(Into::into).collect(),
        }
    }
}

/// Installation alignment - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationAlignment {
    pub permanent: bool,
    pub alignment_dcm: [[f32; 3]; 3],
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}

impl From<InstallationAlignmentPacket> for InstallationAlignment {
    fn from(p: InstallationAlignmentPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            alignment_dcm: p.alignment_dcm,
            gnss_antenna_offset: p.gnss_antenna_offset,
            odometer_offset: p.odometer_offset,
            external_data_offset: p.external_data_offset,
        }
    }
}

impl From<InstallationAlignment> for InstallationAlignmentPacket {
    fn from(i: InstallationAlignment) -> Self {
        Self {
            permanent: i.permanent as u8,
            alignment_dcm: i.alignment_dcm,
            gnss_antenna_offset: i.gnss_antenna_offset,
            odometer_offset: i.odometer_offset,
            external_data_offset: i.external_data_offset,
        }
    }
}

/// Filter options - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptions {
    pub permanent: bool,
    pub vehicle_type: VehicleType,
    pub internal_gnss_enabled: bool,
    pub atmospheric_altitude_enabled: bool,
    pub velocity_heading_enabled: bool,
    pub reversing_detection_enabled: bool,
    pub motion_analysis_enabled: bool,
}

impl From<FilterOptionsPacket> for FilterOptions {
    fn from(p: FilterOptionsPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            vehicle_type: p.vehicle_type,
            internal_gnss_enabled: p.internal_gnss_enabled != 0,
            atmospheric_altitude_enabled: p.atmospheric_altitude_enabled != 0,
            velocity_heading_enabled: p.velocity_heading_enabled != 0,
            reversing_detection_enabled: p.reversing_detection_enabled != 0,
            motion_analysis_enabled: p.motion_analysis_enabled != 0,
        }
    }
}

impl From<FilterOptions> for FilterOptionsPacket {
    fn from(f: FilterOptions) -> Self {
        Self {
            permanent: f.permanent as u8,
            vehicle_type: f.vehicle_type,
            internal_gnss_enabled: f.internal_gnss_enabled as u8,
            reserved1: 0,  // Auto-filled
            atmospheric_altitude_enabled: f.atmospheric_altitude_enabled as u8,
            velocity_heading_enabled: f.velocity_heading_enabled as u8,
            reversing_detection_enabled: f.reversing_detection_enabled as u8,
            motion_analysis_enabled: f.motion_analysis_enabled as u8,
            reserved2: 0,  // Auto-filled
            reserved3: [0; 8],  // Auto-filled
        }
    }
}

/// Odometer configuration - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerConfiguration {
    pub permanent: bool,
    pub automatic_pulse_measurement: bool,
    pub pulse_length: f32,
}

impl From<OdometerConfigurationPacket> for OdometerConfiguration {
    fn from(p: OdometerConfigurationPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            automatic_pulse_measurement: p.automatic_pulse_measurement != 0,
            pulse_length: p.pulse_length,
        }
    }
}

impl From<OdometerConfiguration> for OdometerConfigurationPacket {
    fn from(o: OdometerConfiguration) -> Self {
        Self {
            permanent: o.permanent as u8,
            automatic_pulse_measurement: o.automatic_pulse_measurement as u8,
            reserved: 0,  // Auto-filled
            pulse_length: o.pulse_length,
        }
    }
}

/// Set zero orientation alignment - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetZeroOrientationAlignment {
    pub permanent: bool,
}

impl From<SetZeroOrientationAlignmentPacket> for SetZeroOrientationAlignment {
    fn from(p: SetZeroOrientationAlignmentPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
        }
    }
}

impl From<SetZeroOrientationAlignment> for SetZeroOrientationAlignmentPacket {
    fn from(s: SetZeroOrientationAlignment) -> Self {
        Self {
            permanent: s.permanent as u8,
        }
    }
}

/// Reference point offsets
/// Heave point 1 is the primary reference point offset
/// Heave point 2 is used for COG Lever Arm offset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferencePointOffsets {
    pub permanent: bool,
    pub heave_point_1: OffsetVector,
    pub heave_point_2: OffsetVector,
    pub heave_point_3: OffsetVector,
    pub heave_point_4: OffsetVector,
}

impl From<ReferencePointOffsetsPacket> for ReferencePointOffsets {
    fn from(p: ReferencePointOffsetsPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            heave_point_1: p.heave_point_1,
            heave_point_2: p.heave_point_2,
            heave_point_3: p.heave_point_3,
            heave_point_4: p.heave_point_4,
        }
    }
}

impl From<ReferencePointOffsets> for ReferencePointOffsetsPacket {
    fn from(r: ReferencePointOffsets) -> Self {
        Self {
            permanent: r.permanent as u8,
            heave_point_1: r.heave_point_1,
            heave_point_2: r.heave_point_2,
            heave_point_3: r.heave_point_3,
            heave_point_4: r.heave_point_4,
        }
    }
}

/// IP dataport configuration - clean API
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IpDataport {
    pub ip_address: u32,
    pub port: u16,
    pub mode: IpDataportMode,
}

impl From<IpDataportEntry> for IpDataport {
    fn from(e: IpDataportEntry) -> Self {
        Self {
            ip_address: e.ip_address,
            port: e.port,
            mode: e.mode,
        }
    }
}

impl From<IpDataport> for IpDataportEntry {
    fn from(i: IpDataport) -> Self {
        Self {
            ip_address: i.ip_address,
            port: i.port,
            mode: i.mode,
        }
    }
}

/// IP dataports configuration - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfiguration {
    pub dataports: [IpDataport; 4],
}

impl From<IpDataportsConfigurationPacket> for IpDataportsConfiguration {
    fn from(p: IpDataportsConfigurationPacket) -> Self {
        Self {
            dataports: [
                p.dataports[0].into(),
                p.dataports[1].into(),
                p.dataports[2].into(),
                p.dataports[3].into(),
            ],
        }
    }
}

impl From<IpDataportsConfiguration> for IpDataportsConfigurationPacket {
    fn from(i: IpDataportsConfiguration) -> Self {
        Self {
            reserved: 0,  // Auto-filled
            dataports: [
                i.dataports[0].into(),
                i.dataports[1].into(),
                i.dataports[2].into(),
                i.dataports[3].into(),
            ],
        }
    }
}

#[cfg(test)]
#[path = "tests/config.rs"]
mod tests;