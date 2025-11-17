use binrw::{BinRead, BinWrite};
use serde::{Serialize, Deserialize};

/// System Packets (0-14)
/// Acknowledge packet structure (Packet ID 0, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct AcknowledgePacket {
    pub packet_id: u8,
    pub packet_crc: u16,
    pub result: u8,
}


/// Request packet structure (Packet ID 1, Variable length) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct RequestPacket {
    pub packet_id: u8,
}


/// Boot mode packet structure (Packet ID 2, Length 1) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct BootModePacket {
    pub boot_mode: u8,
}


/// Device information packet structure (Packet ID 3, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct DeviceInformationPacket {
    pub software_version: u32,
    pub device_id: u32,
    pub hardware_revision: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}



/// Restore factory settings packet structure (Packet ID 4, Length 4) - Write only
///
/// Note: A Factory Reset will re-enable the DHCP Client and lose any static IP address settings.
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct RestoreFactorySettingsPacket {
    pub verification: u32, // Verification code (must be 0x85429E1C)
}


impl RestoreFactorySettingsPacket {
    /// Create a new restore factory settings packet with the correct verification code
    pub fn new() -> Self {
        Self {
            verification: 0x85429E1C,
        }
    }
}

impl Default for RestoreFactorySettingsPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Reset packet structure (Packet ID 5, Length 4) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct ResetPacket {
    pub verification: u32, // Verification code (must be 0x21057A7E)
}


impl ResetPacket {
    /// Create a new reset packet with the correct verification code
    pub fn new() -> Self {
        Self {
            verification: 0x21057A7E,
        }
    }
}

impl Default for ResetPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// IP configuration packet structure (Packet ID 11, Length 30) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct IpConfigurationPacket {
    pub permanent: u8,
    pub dhcp_mode: u8,
    pub ip_address: u32,
    pub ip_netmask: u32,
    pub ip_gateway: u32,
    pub dns_server: u32,
    pub boreas_serial_number_part_1: u32,
    pub boreas_serial_number_part_2: u32,
    pub boreas_serial_number_part_3: u32,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_packet_try_from_try_into() {
        // Test serialization and deserialization using direct binrw
        let original_packet = ResetPacket::new();

        // Serialize using BinWrite
        let mut cursor = std::io::Cursor::new(Vec::new());
        original_packet.write_le(&mut cursor).unwrap();
        let serialized = cursor.into_inner();

        // Deserialize using BinRead
        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = ResetPacket::read_le(&mut cursor).unwrap();

        assert_eq!(original_packet, deserialized);
        assert_eq!(deserialized.verification, 0x21057A7E);
    }

    #[test]
    fn test_device_information_try_from_vec() {
        // Create a mock device information
        let device_info = DeviceInformationPacket {
            software_version: 123,
            device_id: 456,
            hardware_revision: 789,
            serial_number_1: 111,
            serial_number_2: 222,
            serial_number_3: 333,
        };

        // Serialize using BinWrite
        let mut cursor = std::io::Cursor::new(Vec::new());
        device_info.write_le(&mut cursor).unwrap();
        let serialized = cursor.into_inner();

        // Deserialize using BinRead
        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = DeviceInformationPacket::read_le(&mut cursor).unwrap();

        assert_eq!(device_info, deserialized);
    }

    #[test]
    fn test_boot_mode_packet_try_from_slice() {
        let boot_mode = BootModePacket { boot_mode: 42 };

        let mut cursor = std::io::Cursor::new(Vec::new());
        boot_mode.write_le(&mut cursor).unwrap();
        let serialized = cursor.into_inner();

        // Test direct binrw deserialization
        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = BootModePacket::read_le(&mut cursor).unwrap();

        assert_eq!(boot_mode, deserialized);
        assert_eq!(deserialized.boot_mode, 42);
    }
}

#[cfg(test)]
#[path = "tests/system.rs"]
mod system_length_tests;

// ============================================================================
// Public API Types
// ============================================================================

use super::PacketKind;

/// Acknowledge result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcknowledgeResult {
    Success = 0,
    Failure = 1,
    UnknownPacket = 2,
}

/// Acknowledge packet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acknowledge {
    pub acknowledged_packet: PacketKind,
    pub result: AcknowledgeResult,
}

impl Acknowledge {
    /// Create from wire format packet
    pub(crate) fn from_packet_with_crc(p: AcknowledgePacket) -> Self {
        let result = match p.result {
            0 => AcknowledgeResult::Success,
            1 => AcknowledgeResult::Failure,
            2 => AcknowledgeResult::UnknownPacket,
            _ => AcknowledgeResult::Failure,
        };

        Self {
            acknowledged_packet: PacketKind::from(p.packet_id),
            result,
        }
    }

    /// Convert to wire format packet with the provided CRC
    #[allow(dead_code)]
    pub(crate) fn to_packet_with_crc(&self, packet_crc: u16) -> AcknowledgePacket {
        AcknowledgePacket {
            packet_id: self.acknowledged_packet as u8,
            packet_crc,
            result: self.result as u8,
        }
    }
}

/// Request packet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub requested_packet: PacketKind,
}

impl From<RequestPacket> for Request {
    fn from(p: RequestPacket) -> Self {
        Self {
            requested_packet: PacketKind::from(p.packet_id),
        }
    }
}

impl From<Request> for RequestPacket {
    fn from(r: Request) -> Self {
        Self {
            packet_id: r.requested_packet as u8,
        }
    }
}

/// Boot mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootMode {
    pub boot_mode: u8,
}

impl From<BootModePacket> for BootMode {
    fn from(p: BootModePacket) -> Self {
        Self {
            boot_mode: p.boot_mode,
        }
    }
}

impl From<BootMode> for BootModePacket {
    fn from(b: BootMode) -> Self {
        Self {
            boot_mode: b.boot_mode,
        }
    }
}

/// Device information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInformation {
    pub software_version: u32,
    pub device_id: u32,
    pub hardware_revision: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}

impl From<DeviceInformationPacket> for DeviceInformation {
    fn from(p: DeviceInformationPacket) -> Self {
        Self {
            software_version: p.software_version,
            device_id: p.device_id,
            hardware_revision: p.hardware_revision,
            serial_number_1: p.serial_number_1,
            serial_number_2: p.serial_number_2,
            serial_number_3: p.serial_number_3,
        }
    }
}

impl From<DeviceInformation> for DeviceInformationPacket {
    fn from(d: DeviceInformation) -> Self {
        Self {
            software_version: d.software_version,
            device_id: d.device_id,
            hardware_revision: d.hardware_revision,
            serial_number_1: d.serial_number_1,
            serial_number_2: d.serial_number_2,
            serial_number_3: d.serial_number_3,
        }
    }
}

/// Restore factory settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreFactorySettings;

impl From<RestoreFactorySettingsPacket> for RestoreFactorySettings {
    fn from(_: RestoreFactorySettingsPacket) -> Self {
        Self
    }
}

impl From<RestoreFactorySettings> for RestoreFactorySettingsPacket {
    fn from(_: RestoreFactorySettings) -> Self {
        Self::new()
    }
}

/// Reset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset;

impl From<ResetPacket> for Reset {
    fn from(_: ResetPacket) -> Self {
        Self
    }
}

impl From<Reset> for ResetPacket {
    fn from(_: Reset) -> Self {
        Self::new()
    }
}

/// IP configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpConfiguration {
    pub permanent: bool,
    pub dhcp_mode: u8,
    pub ip_address: u32,
    pub ip_netmask: u32,
    pub ip_gateway: u32,
    pub dns_server: u32,
    pub boreas_serial_number_part_1: u32,
    pub boreas_serial_number_part_2: u32,
    pub boreas_serial_number_part_3: u32,
}

impl From<IpConfigurationPacket> for IpConfiguration {
    fn from(p: IpConfigurationPacket) -> Self {
        Self {
            permanent: p.permanent != 0,
            dhcp_mode: p.dhcp_mode,
            ip_address: p.ip_address,
            ip_netmask: p.ip_netmask,
            ip_gateway: p.ip_gateway,
            dns_server: p.dns_server,
            boreas_serial_number_part_1: p.boreas_serial_number_part_1,
            boreas_serial_number_part_2: p.boreas_serial_number_part_2,
            boreas_serial_number_part_3: p.boreas_serial_number_part_3,
        }
    }
}

impl From<IpConfiguration> for IpConfigurationPacket {
    fn from(i: IpConfiguration) -> Self {
        Self {
            permanent: i.permanent as u8,
            dhcp_mode: i.dhcp_mode,
            ip_address: i.ip_address,
            ip_netmask: i.ip_netmask,
            ip_gateway: i.ip_gateway,
            dns_server: i.dns_server,
            boreas_serial_number_part_1: i.boreas_serial_number_part_1,
            boreas_serial_number_part_2: i.boreas_serial_number_part_2,
            boreas_serial_number_part_3: i.boreas_serial_number_part_3,
        }
    }
}