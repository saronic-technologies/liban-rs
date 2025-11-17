use binrw::{BinRead, BinWrite};
use serde::{Serialize, Deserialize};

/// System Packets (0-14)

/// Acknowledge packet structure (Packet ID 0, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AcknowledgePacket {
    pub packet_id: u8,
    pub packet_crc: u16,
    pub result: u8,
}


/// Request packet structure (Packet ID 1, Variable length) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RequestPacket {
    pub packet_id: u8,
}


/// Boot mode packet structure (Packet ID 2, Length 1) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct BootModePacket {
    pub boot_mode: u8,
}


/// Device information packet structure (Packet ID 3, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DeviceInformationPacket {
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
pub struct RestoreFactorySettingsPacket {
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
pub struct ResetPacket {
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
pub struct IpConfigurationPacket {
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