#[cfg(test)]
mod tests {
    use crate::packet::system::{
        Acknowledge, AcknowledgeResult, Request, BootMode,
        DeviceInformation, RestoreFactorySettings,
        Reset, IpConfiguration
    };
    use crate::packet::PacketKind;
    use binrw::{BinRead, BinWrite};

    #[test]
    fn test_acknowledge_packet_length() {
        let packet = Acknowledge {
            acknowledged_packet: PacketKind::InstallationAlignment,
            packet_crc: 0x1234,
            result: AcknowledgeResult::Success,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "Acknowledge should be 4 bytes");
    }

    #[test]
    fn test_request_packet_length() {
        let packet = Request {
            requested_packet: PacketKind::SystemState,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 1, "Request should be 1 byte");
    }

    #[test]
    fn test_boot_mode_packet_length() {
        let packet = BootMode { boot_mode: 1 };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 1, "BootMode should be 1 byte");
    }

    #[test]
    fn test_device_information_packet_length() {
        let packet = DeviceInformation {
            software_version: 0x01020304,
            device_type: crate::packet::system::DeviceType::Unknown,
            hardware_revision: 0x090A0B0C,
            serial_number_1: 111111,
            serial_number_2: 222222,
            serial_number_3: 333333,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 24, "DeviceInformation should be 24 bytes");
    }

    #[test]
    fn test_restore_factory_settings_packet_length() {
        let packet = RestoreFactorySettings {};

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "RestoreFactorySettings should be 4 bytes");
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x85429E1C);
    }

    #[test]
    fn test_reset_packet_length() {
        let packet = Reset {};

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "Reset should be 4 bytes");
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x21057A7E);
    }

    #[test]
    fn test_ip_configuration_packet_length() {
        let packet = IpConfiguration {
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
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30, "IpConfiguration should be exactly 30 bytes");
    }

    #[test]
    fn test_system_packet_round_trips() {
        // Acknowledge round-trip
        let ack_original = Acknowledge {
            acknowledged_packet: PacketKind::IpDataportsConfiguration,
            packet_crc: 0xABCD,
            result: AcknowledgeResult::UnknownPacket,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        ack_original.write_le(&mut cursor).unwrap();
        let ack_bytes = cursor.into_inner();
        let mut cursor = std::io::Cursor::new(&ack_bytes);
        let ack_deserialized = Acknowledge::read_le(&mut cursor).unwrap();
        assert_eq!(ack_original, ack_deserialized);

        // DeviceInformation round-trip
        let dev_original = DeviceInformation {
            software_version: 0x12345678,
            device_type: crate::packet::system::DeviceType::Certus,
            hardware_revision: 0xDEADBEEF,
            serial_number_1: 987654,
            serial_number_2: 321098,
            serial_number_3: 765432,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        dev_original.write_le(&mut cursor).unwrap();
        let dev_bytes = cursor.into_inner();
        let mut cursor = std::io::Cursor::new(&dev_bytes);
        let dev_deserialized = DeviceInformation::read_le(&mut cursor).unwrap();
        assert_eq!(dev_original, dev_deserialized);

        // IpConfiguration round-trip
        let ip_original = IpConfiguration {
            permanent: false,
            dhcp_mode: 1,
            ip_address: 0,
            ip_netmask: 0,
            ip_gateway: 0,
            dns_server: 0,
            boreas_serial_number_part_1: 111111,
            boreas_serial_number_part_2: 222222,
            boreas_serial_number_part_3: 333333,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        ip_original.write_le(&mut cursor).unwrap();
        let ip_bytes = cursor.into_inner();
        let mut cursor = std::io::Cursor::new(&ip_bytes);
        let ip_deserialized = IpConfiguration::read_le(&mut cursor).unwrap();
        assert_eq!(ip_original, ip_deserialized);
    }
}
