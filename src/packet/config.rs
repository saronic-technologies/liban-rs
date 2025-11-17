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
pub(crate) struct OffsetVector {
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
pub(crate) enum VehicleType {
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


/// Reference point offsets packet structure (Packet ID 194, Length 13) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub offset: OffsetVector,
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
pub(crate) enum IpDataportMode {
    Disabled = 0,
    TcpServer = 2,
    TcpClient = 3,
    UdpClient = 4,
}

#[cfg(test)]
#[path = "tests/config.rs"]
mod tests;