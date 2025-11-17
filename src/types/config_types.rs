//! Clean Rust API types for config packets

use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::packet::{config, PacketKind, Packet};

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

impl From<config::PacketPeriodEntry> for PacketPeriod {
    fn from(p: config::PacketPeriodEntry) -> Self {
        Self {
            packet_type: PacketKind::from(p.packet_id),
            period: Duration::from_millis(p.period as u64),
        }
    }
}

impl From<PacketPeriod> for config::PacketPeriodEntry {
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

impl From<config::PacketTimerPeriodPacket> for PacketTimerPeriod {
    fn from(p: config::PacketTimerPeriodPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            utc_synchronisation: p.utc_synchronisation != 0,
            packet_timer_period: Duration::from_millis(p.packet_timer_period as u64),
        }
    }
}

impl From<PacketTimerPeriod> for config::PacketTimerPeriodPacket {
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

impl From<config::PacketsPeriodPacket> for PacketsPeriod {
    fn from(p: config::PacketsPeriodPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            clear_existing: p.clear_existing != 0,
            packet_periods: p.packet_periods.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PacketsPeriod> for config::PacketsPeriodPacket {
    fn from(p: PacketsPeriod) -> Self {
        Self {
            permanent: p.permanent as u8,
            clear_existing: p.clear_existing as u8,
            packet_periods: p.packet_periods.into_iter().map(Into::into).collect(),
        }
    }
}

/// 3D offset vector - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<config::OffsetVector> for OffsetVector {
    fn from(o: config::OffsetVector) -> Self {
        Self {
            x: o.x,
            y: o.y,
            z: o.z,
        }
    }
}

impl From<OffsetVector> for config::OffsetVector {
    fn from(o: OffsetVector) -> Self {
        Self {
            x: o.x,
            y: o.y,
            z: o.z,
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

impl From<config::InstallationAlignmentPacket> for InstallationAlignment {
    fn from(p: config::InstallationAlignmentPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            alignment_dcm: p.alignment_dcm,
            gnss_antenna_offset: p.gnss_antenna_offset.into(),
            odometer_offset: p.odometer_offset.into(),
            external_data_offset: p.external_data_offset.into(),
        }
    }
}

impl From<InstallationAlignment> for config::InstallationAlignmentPacket {
    fn from(i: InstallationAlignment) -> Self {
        Self {
            permanent: i.permanent as u8,
            alignment_dcm: i.alignment_dcm,
            gnss_antenna_offset: i.gnss_antenna_offset.into(),
            odometer_offset: i.odometer_offset.into(),
            external_data_offset: i.external_data_offset.into(),
        }
    }
}

/// Vehicle type enumeration for filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl From<config::VehicleType> for VehicleType {
    fn from(v: config::VehicleType) -> Self {
        match v {
            config::VehicleType::Unlimited => VehicleType::Unlimited,
            config::VehicleType::BicycleOrMotorcycle => VehicleType::BicycleOrMotorcycle,
            config::VehicleType::Car => VehicleType::Car,
            config::VehicleType::Hovercraft => VehicleType::Hovercraft,
            config::VehicleType::Submarine => VehicleType::Submarine,
            config::VehicleType::Underwater3D => VehicleType::Underwater3D,
            config::VehicleType::FixedWingPlane => VehicleType::FixedWingPlane,
            config::VehicleType::Aircraft3D => VehicleType::Aircraft3D,
            config::VehicleType::Human => VehicleType::Human,
            config::VehicleType::Boat => VehicleType::Boat,
            config::VehicleType::LargeShip => VehicleType::LargeShip,
            config::VehicleType::Stationary => VehicleType::Stationary,
            config::VehicleType::StuntPlane => VehicleType::StuntPlane,
            config::VehicleType::RaceCar => VehicleType::RaceCar,
            config::VehicleType::Train => VehicleType::Train,
        }
    }
}

impl From<VehicleType> for config::VehicleType {
    fn from(v: VehicleType) -> Self {
        match v {
            VehicleType::Unlimited => config::VehicleType::Unlimited,
            VehicleType::BicycleOrMotorcycle => config::VehicleType::BicycleOrMotorcycle,
            VehicleType::Car => config::VehicleType::Car,
            VehicleType::Hovercraft => config::VehicleType::Hovercraft,
            VehicleType::Submarine => config::VehicleType::Submarine,
            VehicleType::Underwater3D => config::VehicleType::Underwater3D,
            VehicleType::FixedWingPlane => config::VehicleType::FixedWingPlane,
            VehicleType::Aircraft3D => config::VehicleType::Aircraft3D,
            VehicleType::Human => config::VehicleType::Human,
            VehicleType::Boat => config::VehicleType::Boat,
            VehicleType::LargeShip => config::VehicleType::LargeShip,
            VehicleType::Stationary => config::VehicleType::Stationary,
            VehicleType::StuntPlane => config::VehicleType::StuntPlane,
            VehicleType::RaceCar => config::VehicleType::RaceCar,
            VehicleType::Train => config::VehicleType::Train,
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

impl From<config::FilterOptionsPacket> for FilterOptions {
    fn from(p: config::FilterOptionsPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            vehicle_type: p.vehicle_type.into(),
            internal_gnss_enabled: p.internal_gnss_enabled != 0,
            atmospheric_altitude_enabled: p.atmospheric_altitude_enabled != 0,
            velocity_heading_enabled: p.velocity_heading_enabled != 0,
            reversing_detection_enabled: p.reversing_detection_enabled != 0,
            motion_analysis_enabled: p.motion_analysis_enabled != 0,
        }
    }
}

impl From<FilterOptions> for config::FilterOptionsPacket {
    fn from(f: FilterOptions) -> Self {
        Self {
            permanent: f.permanent as u8,
            vehicle_type: f.vehicle_type.into(),
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

impl From<config::OdometerConfigurationPacket> for OdometerConfiguration {
    fn from(p: config::OdometerConfigurationPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            automatic_pulse_measurement: p.automatic_pulse_measurement != 0,
            pulse_length: p.pulse_length,
        }
    }
}

impl From<OdometerConfiguration> for config::OdometerConfigurationPacket {
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

impl From<config::SetZeroOrientationAlignmentPacket> for SetZeroOrientationAlignment {
    fn from(p: config::SetZeroOrientationAlignmentPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
        }
    }
}

impl From<SetZeroOrientationAlignment> for config::SetZeroOrientationAlignmentPacket {
    fn from(s: SetZeroOrientationAlignment) -> Self {
        Self {
            permanent: s.permanent as u8,
        }
    }
}

/// Reference point offsets - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferencePointOffsets {
    pub permanent: bool,
    pub offset: OffsetVector,
}

impl From<config::ReferencePointOffsetsPacket> for ReferencePointOffsets {
    fn from(p: config::ReferencePointOffsetsPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            offset: p.offset.into(),
        }
    }
}

impl From<ReferencePointOffsets> for config::ReferencePointOffsetsPacket {
    fn from(r: ReferencePointOffsets) -> Self {
        Self {
            permanent: r.permanent as u8,
            offset: r.offset.into(),
        }
    }
}

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 2,
    TcpClient = 3,
    UdpClient = 4,
}

impl From<config::IpDataportMode> for IpDataportMode {
    fn from(m: config::IpDataportMode) -> Self {
        match m {
            config::IpDataportMode::Disabled => IpDataportMode::Disabled,
            config::IpDataportMode::TcpServer => IpDataportMode::TcpServer,
            config::IpDataportMode::TcpClient => IpDataportMode::TcpClient,
            config::IpDataportMode::UdpClient => IpDataportMode::UdpClient,
        }
    }
}

impl From<IpDataportMode> for config::IpDataportMode {
    fn from(m: IpDataportMode) -> Self {
        match m {
            IpDataportMode::Disabled => config::IpDataportMode::Disabled,
            IpDataportMode::TcpServer => config::IpDataportMode::TcpServer,
            IpDataportMode::TcpClient => config::IpDataportMode::TcpClient,
            IpDataportMode::UdpClient => config::IpDataportMode::UdpClient,
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

impl From<config::IpDataportEntry> for IpDataport {
    fn from(e: config::IpDataportEntry) -> Self {
        Self {
            ip_address: e.ip_address,
            port: e.port,
            mode: e.mode.into(),
        }
    }
}

impl From<IpDataport> for config::IpDataportEntry {
    fn from(i: IpDataport) -> Self {
        Self {
            ip_address: i.ip_address,
            port: i.port,
            mode: i.mode.into(),
        }
    }
}

/// IP dataports configuration - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfiguration {
    pub dataports: [IpDataport; 4],
}

impl From<config::IpDataportsConfigurationPacket> for IpDataportsConfiguration {
    fn from(p: config::IpDataportsConfigurationPacket) -> Self {
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

impl From<IpDataportsConfiguration> for config::IpDataportsConfigurationPacket {
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
