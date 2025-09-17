use crate::error::{AnError, Result};
use binrw::{BinRead, BinWrite, binrw};
use super::impl_binrw_packet_conversions;

/// Configuration Packets (180-203)

/// Packet timer period packet structure (Packet ID 180, Length 4) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct PacketTimerPeriodPacket {
    pub permanent: u8,
    pub utc_synchronisation: u8,
    pub packet_timer_period: u16,
}

impl_binrw_packet_conversions!(PacketTimerPeriodPacket);

/// Packets period packet structure (Packet ID 181, Variable length) - Read/Write  
/// Uses a length-prefixed format: u8 count followed by that many entries
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq)]
pub struct PacketsPeriodPacket {
    #[br(temp)]
    #[bw(calc = packet_periods.len() as u8)]
    count: u8,
    #[br(count = count)]
    pub packet_periods: Vec<PacketPeriodEntry>,
}

impl_binrw_packet_conversions!(PacketsPeriodPacket);

/// Entry for packets period configuration
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct PacketPeriodEntry {
    pub packet_id: u8,
    pub period: u32,
}

/// Installation alignment packet structure (Packet ID 185, Length 73) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct InstallationAlignmentPacket {
    pub permanent: u8,
    pub alignment_dcm: [[f32; 3]; 3], // 3x3 Direction Cosine Matrix
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}

impl_binrw_packet_conversions!(InstallationAlignmentPacket);

/// 3D offset vector for installation alignment
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Filter options packet structure (Packet ID 186, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct FilterOptionsPacket {
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

impl_binrw_packet_conversions!(FilterOptionsPacket);

/// Vehicle type enumeration for filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite)]
#[brw(repr = u8)]
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

/// Odometer configuration packet structure (Packet ID 192, Length 8) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct OdometerConfigurationPacket {
    pub permanent: u8,
    pub automatic_pulse_measurement: u8,
    pub reserved: u16,
    pub pulse_length: f32,
}

impl_binrw_packet_conversions!(OdometerConfigurationPacket);

/// Set zero orientation alignment packet structure (Packet ID 193, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct SetZeroOrientationAlignmentPacket {
    pub permanent: u8,
}

impl_binrw_packet_conversions!(SetZeroOrientationAlignmentPacket);

/// Reference point offsets packet structure (Packet ID 194, Length 13) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub offset: OffsetVector,
}

impl_binrw_packet_conversions!(ReferencePointOffsetsPacket);

/// IP dataports configuration packet structure (Packet ID 202, Length 30) - Read/Write
/// Contains exactly 4 dataport entries as per ANPP specification
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct IpDataportsConfigurationPacket {
    pub reserved: u16,
    pub dataports: [IpDataportEntry; 4],
}

impl_binrw_packet_conversions!(IpDataportsConfigurationPacket);

/// IP dataport entry for IP dataports configuration
/// Fields: ip_address(u32), port(u16), mode(u8) = 7 bytes per entry
#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct IpDataportEntry {
    pub ip_address: u32,
    pub port: u16,
    pub mode: IpDataportMode,
}

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite)]
#[brw(repr = u8)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 2,
    TcpClient = 3,
    UdpClient = 4,
}

#[cfg(test)]
#[path = "tests/config.rs"]
mod tests;