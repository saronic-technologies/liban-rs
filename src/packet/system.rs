use super::PacketKind;
use binrw::{binrw, BinRead, BinWrite};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};

/// Acknowledge result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum AcknowledgeResult {
    Success = 0,
    /// Failure due to a CRC error
    CrcError = 1,
    /// Failure due to an incorrect packet size
    PacketSizeIncorrect = 2,
    /// Failure due to values outside of valid ranges
    ValuesOutsideRange = 3,
    /// Failure due to a system flash memory failure
    FlashMemoryFailure = 4,
    /// Failure because the system is not ready
    SystemNotReady = 5,
    /// Failure because the packet is unknown
    UnknownPacket = 6,
    /// Result code not defined by the protocol
    #[num_enum(default)]
    Unknown = 255,
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
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
    AirDataUnit2 = 69,
}

/// Device information packet (Packet ID 3, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DeviceInformation {
    pub software_version: u32,
    #[br(map = |x: u32| DeviceType::from(x))]
    #[bw(map = |x: &DeviceType| *x as u32)]
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

/// Reset mode selected by the Reset packet's verification word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u32)]
#[repr(u32)]
pub enum ResetMode {
    /// Perform a powers cycle. No configuration settings or state data are lost.
    HotStart = 0x21057A7E,
    /// Clears all filters, and connections are reset and must be re-established. 
    /// No configuration settings are lost.
    ColdStart = 0x9A5D38B7,
}

/// Reset packet (Packet ID 5, Length 4) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset {
    /// Reset mode the device performs.
    pub mode: ResetMode,
}

/// Passthrough route for SerialPortPassthrough
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum PassthroughRoute {
    /// GPIO 1 and 2
    Gpio1And2 = 1,
    /// Auxiliary port
    Auxiliary = 2,
}

/// Serial port passthrough packet (Packet ID 10, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SerialPortPassthrough {
    /// Passthrough route
    pub route: PassthroughRoute,
    /// Passthrough data
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = super::write_vec)]
    pub data: Vec<u8>,
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

/// Extended device information packet (Packet ID 13, Length 36) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedDeviceInformation {
    /// Software version
    pub software_version: u32,
    /// Device type identifier
    #[br(map = |x: u32| DeviceType::from(x))]
    #[bw(map = |x: &DeviceType| *x as u32)]
    pub device_id: DeviceType,
    /// Hardware revision
    pub hardware_revision: u32,
    /// Device serial number
    pub serial_number: [u32; 3],
    /// Device sub-type
    pub device_sub_type: u32,
    #[br(temp)]
    #[bw(calc = [0u8; 8])]
    _reserved: [u8; 8],
}

/// Subcomponent device identifier
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum SubcomponentDeviceId {
    #[default]
    Unknown = 0,
    SpatialMemsImu = 5,
    EvoMemsImu = 17,
    Aries = 27,
}

/// Single subcomponent entry within SubcomponentInformation
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SubcomponentEntry {
    /// Subcomponent software version
    pub software_version: u32,
    /// Subcomponent device identifier
    #[br(map = |x: u32| SubcomponentDeviceId::from(x))]
    #[bw(map = |x: &SubcomponentDeviceId| *x as u32)]
    pub device_id: SubcomponentDeviceId,
    /// Subcomponent hardware revision
    pub hardware_revision: u32,
    /// Subcomponent serial number
    pub serial_number: [u32; 3],
}

/// Subcomponent information packet (Packet ID 14, Variable length) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SubcomponentInformation {
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = super::write_vec)]
    pub subcomponents: Vec<SubcomponentEntry>,
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
        let packet = Reset {
            mode: ResetMode::HotStart,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x21057A7E);
    }

    #[test]
    fn test_reset_cold_start_write() {
        let packet = Reset {
            mode: ResetMode::ColdStart,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).unwrap();
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x9A5D38B7);
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
