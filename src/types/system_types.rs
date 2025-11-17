//! Clean Rust API types for system packets

use serde::{Serialize, Deserialize};
use crate::packet::{system, PacketKind};

/// Acknowledge result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcknowledgeResult {
    Success = 0,
    Failure = 1,
    UnknownPacket = 2,
}

/// Acknowledge packet - clean API (no CRC, packet type instead of ID)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acknowledge {
    pub acknowledged_packet: PacketKind,
    pub result: AcknowledgeResult,
}

impl Acknowledge {
    /// Create from wire format packet when you have the original packet CRC
    pub fn from_packet_with_crc(p: system::AcknowledgePacket) -> Self {
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
    pub fn to_packet_with_crc(&self, packet_crc: u16) -> system::AcknowledgePacket {
        system::AcknowledgePacket {
            packet_id: self.acknowledged_packet as u8,
            packet_crc,
            result: self.result as u8,
        }
    }
}

/// Request packet - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub requested_packet: PacketKind,
}

impl From<system::RequestPacket> for Request {
    fn from(p: system::RequestPacket) -> Self {
        Self {
            requested_packet: PacketKind::from(p.packet_id),
        }
    }
}

impl From<Request> for system::RequestPacket {
    fn from(r: Request) -> Self {
        Self {
            packet_id: r.requested_packet as u8,
        }
    }
}

/// Boot mode packet - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootMode {
    pub boot_mode: u8,
}

impl From<system::BootModePacket> for BootMode {
    fn from(p: system::BootModePacket) -> Self {
        Self {
            boot_mode: p.boot_mode,
        }
    }
}

impl From<BootMode> for system::BootModePacket {
    fn from(b: BootMode) -> Self {
        Self {
            boot_mode: b.boot_mode,
        }
    }
}

/// Device information - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInformation {
    pub software_version: u32,
    pub device_id: u32,
    pub hardware_revision: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}

impl From<system::DeviceInformationPacket> for DeviceInformation {
    fn from(p: system::DeviceInformationPacket) -> Self {
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

impl From<DeviceInformation> for system::DeviceInformationPacket {
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

/// Restore factory settings - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreFactorySettings;

impl From<system::RestoreFactorySettingsPacket> for RestoreFactorySettings {
    fn from(_: system::RestoreFactorySettingsPacket) -> Self {
        Self
    }
}

impl From<RestoreFactorySettings> for system::RestoreFactorySettingsPacket {
    fn from(_: RestoreFactorySettings) -> Self {
        Self::new()
    }
}

/// Reset - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset;

impl From<system::ResetPacket> for Reset {
    fn from(_: system::ResetPacket) -> Self {
        Self
    }
}

impl From<Reset> for system::ResetPacket {
    fn from(_: Reset) -> Self {
        Self::new()
    }
}

/// IP configuration - clean API
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

impl From<system::IpConfigurationPacket> for IpConfiguration {
    fn from(p: system::IpConfigurationPacket) -> Self {
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

impl From<IpConfiguration> for system::IpConfigurationPacket {
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
