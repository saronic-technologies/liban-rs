use crate::error::{AnError, Result};
use serde::{Deserialize, Serialize};
use super::impl_packet_conversions;

/// Configuration Packets (180-203)

/// Packet timer period packet structure (Packet ID 180, Length 20) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketTimerPeriodPacket {
    pub packet_timer_period: [u32; 5],
}

impl_packet_conversions!(PacketTimerPeriodPacket);

/// Packets period packet structure (Packet ID 181, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketsPeriodPacket {
    pub packet_periods: Vec<PacketPeriodEntry>,
}

impl_packet_conversions!(PacketsPeriodPacket);

/// Entry for packets period configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketPeriodEntry {
    pub packet_id: u8,
    pub period: u32,
}

/// Installation alignment packet structure (Packet ID 185, Length 48) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationAlignmentPacket {
    pub permanent: u8,
    pub alignment_dcm: [[f32; 3]; 3], // 3x3 Direction Cosine Matrix
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}

impl_packet_conversions!(InstallationAlignmentPacket);

/// 3D offset vector for installation alignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Filter options packet structure (Packet ID 186, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptionsPacket {
    pub permanent: u8,
    pub vehicle_type: VehicleType,
    pub internal_gnss_enabled: u8,
    pub atmospheric_altitude_enabled: u8,
    pub velocity_heading_enabled: u8,
    pub reversing_detection_enabled: u8,
    pub motion_analysis_enabled: u8,
    pub automatic_magnetic_calibration_enabled: u8,
    pub magnetic_heading_enabled: u8,
    pub odometer_mode: u8,
    pub odometer_pulse_to_metre_ratio: f32,
    pub dual_antenna_heading_enabled: u8,
}

impl_packet_conversions!(FilterOptionsPacket);

/// Vehicle type enumeration for filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum VehicleType {
    Car = 0,
    Motorcycle = 1,
    Aircraft = 2,
    Boat = 3,
    PersonalWatercraft = 4,
    Ship = 5,
    Helicopter = 6,
    Train = 7,
    FixedWing = 8,
    MultiRotor = 9,
    Tank = 10,
    Truck = 11,
    ArmoredVehicle = 12,
    Bus = 13,
    Excavator = 14,
    Bulldozer = 15,
}

/// Odometer configuration packet structure (Packet ID 192, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerConfigurationPacket {
    pub permanent: u8,
    pub automatic_pulse_measurement: u8,
    pub pulse_length_filtering: u8,
    pub minimum_pulse_length: u32,
    pub maximum_pulse_length: u32,
    pub pulse_to_metre_ratio: f32,
}

impl_packet_conversions!(OdometerConfigurationPacket);

/// Set zero orientation alignment packet structure (Packet ID 193, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetZeroOrientationAlignmentPacket {
    pub permanent: u8,
}

impl_packet_conversions!(SetZeroOrientationAlignmentPacket);

/// Reference point offsets packet structure (Packet ID 194, Length 13) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub offset: OffsetVector,
}

impl_packet_conversions!(ReferencePointOffsetsPacket);

/// IP dataports configuration packet structure (Packet ID 202, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfigurationPacket {
    pub permanent: u8,
    pub entries: Vec<IpDataportEntry>,
}

impl_packet_conversions!(IpDataportsConfigurationPacket);

/// IP dataport entry for IP dataports configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportEntry {
    pub dataport_id: u8,
    pub mode: IpDataportMode,
    pub ip_address: u32,
    pub port: u16,
}

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 1,
    TcpClient = 2,
    UdpServer = 3,
    UdpClient = 4,
}