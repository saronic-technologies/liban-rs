use binrw::{binrw, BinRead, BinWrite};
use serde::{Serialize, Deserialize};

use super::PacketKind;

/// Acknowledge result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcknowledgeResult {
    Success = 0,
    Failure = 1,
    UnknownPacket = 2,
}

impl From<u8> for AcknowledgeResult {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Success,
            1 => Self::Failure,
            2 => Self::UnknownPacket,
            _ => Self::Failure,
        }
    }
}

/// Acknowledge packet (Packet ID 0, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Acknowledge {
    #[br(map = |x: u8| PacketKind::from(x))]
    #[bw(map = |x: &PacketKind| x.packet_id())]
    pub acknowledged_packet: PacketKind,
    pub packet_crc: u16,
    #[br(map = |x: u8| AcknowledgeResult::from(x))]
    #[bw(map = |x: &AcknowledgeResult| *x as u8)]
    pub result: AcknowledgeResult,
}

/// Request packet (Packet ID 1, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Request {
    #[br(map = |x: u8| PacketKind::from(x))]
    #[bw(map = |x: &PacketKind| x.packet_id())]
    pub requested_packet: PacketKind,
}

/// Boot mode packet (Packet ID 2, Length 1) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct BootMode {
    pub boot_mode: u8,
}

/// Advanced Navigation device type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u32)]
pub enum DeviceType {
    #[default]
    Unknown = 0,
    Spatial = 1,
    SpatialFog = 4,
    SpatialDual = 5,
    Orientus = 11,
    AirDataUnit = 13,
    Subsonus = 14,
    SpatialFogDual = 16,
    Motus = 17,
    GnssCompass = 19,
    SubsonusTag = 21,
    Poseidon = 22,
    Certus = 26,
    BoreasD90 = 28,
    BoreasD70 = 41,
    BoreasA90 = 43,
    BoreasA70 = 44,
    CertusMiniA = 49,
    CertusMiniN = 50,
    CertusMiniD = 51,
    BoreasD50 = 54,
    BoreasA50 = 56,
}

impl From<u32> for DeviceType {
    fn from(v: u32) -> Self {
        match v {
            1 => Self::Spatial,
            4 => Self::SpatialFog,
            5 => Self::SpatialDual,
            11 => Self::Orientus,
            13 => Self::AirDataUnit,
            14 => Self::Subsonus,
            16 => Self::SpatialFogDual,
            17 => Self::Motus,
            19 => Self::GnssCompass,
            21 => Self::SubsonusTag,
            22 => Self::Poseidon,
            26 => Self::Certus,
            28 => Self::BoreasD90,
            41 => Self::BoreasD70,
            43 => Self::BoreasA90,
            44 => Self::BoreasA70,
            49 => Self::CertusMiniA,
            50 => Self::CertusMiniN,
            51 => Self::CertusMiniD,
            54 => Self::BoreasD50,
            56 => Self::BoreasA50,
            _ => Self::Unknown,
        }
    }
}

/// Device information packet (Packet ID 3, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DeviceInformation {
    pub software_version: u32,
    pub device_type: DeviceType,
    pub hardware_revision: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}

/// Restore factory settings packet (Packet ID 4, Length 4) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreFactorySettings {
    #[br(temp)]
    #[bw(calc = 0x85429E1Cu32)]
    _verification: u32,
}

/// Reset packet (Packet ID 5, Length 4) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset {
    #[br(temp)]
    #[bw(calc = 0x21057A7Eu32)]
    _verification: u32,
}

/// IP configuration packet (Packet ID 11, Length 30) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct IpConfiguration {
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acknowledge_round_trip() {
        let ack = Acknowledge {
            acknowledged_packet: PacketKind::SystemState,
            packet_crc: 0xABCD,
            result: AcknowledgeResult::Success,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        ack.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = Acknowledge::read_le(&mut cursor).unwrap();
        assert_eq!(ack, deserialized);
    }

    #[test]
    fn test_request_round_trip() {
        let req = Request {
            requested_packet: PacketKind::SystemState,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        req.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 20); // SystemState packet ID

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = Request::read_le(&mut cursor).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_restore_factory_settings_write() {
        let packet = RestoreFactorySettings {};

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x85429E1C);
    }

    #[test]
    fn test_reset_write() {
        let packet = Reset {};

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x21057A7E);
    }

    #[test]
    fn test_ip_configuration_round_trip() {
        let ip = IpConfiguration {
            permanent: true,
            dhcp_mode: 0,
            ip_address: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            ip_netmask: u32::from(std::net::Ipv4Addr::new(255, 255, 255, 0)),
            ip_gateway: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1)),
            dns_server: u32::from(std::net::Ipv4Addr::new(8, 8, 8, 8)),
            boreas_serial_number_part_1: 123456,
            boreas_serial_number_part_2: 789012,
            boreas_serial_number_part_3: 345678,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        ip.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = IpConfiguration::read_le(&mut cursor).unwrap();
        assert_eq!(ip, deserialized);
    }
}

#[cfg(test)]
#[path = "tests/system.rs"]
mod system_length_tests;
