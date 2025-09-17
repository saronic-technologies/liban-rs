#[cfg(test)]
mod tests {
    use crate::{
        AcknowledgePacket, RequestPacket, BootModePacket, 
        DeviceInformationPacket, RestoreFactorySettingsPacket,
        ResetPacket, IpConfigurationPacket
    };
    use std::convert::TryInto;

    #[test]
    fn test_acknowledge_packet_length() {
        // Packet ID 0, Length 4
        let packet = AcknowledgePacket {
            packet_id: 185,
            packet_crc: 0x1234,
            result: 0,
        };

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 4, "AcknowledgePacket should be 4 bytes");
    }

    #[test]
    fn test_request_packet_length() {
        // Packet ID 1, Variable length
        let packet = RequestPacket {
            packet_id: 20,
        };

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 1, "RequestPacket should be 1 byte");
        println!("RequestPacket length: {} bytes", bytes.len());
    }

    #[test]
    fn test_boot_mode_packet_length() {
        // Packet ID 2, Length 1
        let packet = BootModePacket {
            boot_mode: 1,
        };

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 1, "BootModePacket should be 1 byte");
    }

    #[test]
    fn test_device_information_packet_length() {
        // Packet ID 3, Length 24
        let packet = DeviceInformationPacket {
            software_version: 0x01020304,
            device_id: 0x05060708,
            hardware_revision: 0x090A0B0C,
            serial_number_1: 111111,
            serial_number_2: 222222,
            serial_number_3: 333333,
        };

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 24, "DeviceInformationPacket should be 24 bytes");
    }

    #[test]
    fn test_restore_factory_settings_packet_length() {
        // Packet ID 4, Length 4
        let packet = RestoreFactorySettingsPacket::new();
        
        // Verify the verification code is correct
        assert_eq!(packet.verification, 0x85429E1C);

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 4, "RestoreFactorySettingsPacket should be 4 bytes");
    }

    #[test]
    fn test_reset_packet_length() {
        // Packet ID 5, Length 4
        let packet = ResetPacket::new();
        
        // Verify the verification code is correct
        assert_eq!(packet.verification, 0x21057A7E);

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 4, "ResetPacket should be 4 bytes");
    }

    #[test]
    fn test_ip_configuration_packet_length() {
        // Packet ID 11, Length 30
        let packet = IpConfigurationPacket {
            permanent: 1,
            dhcp_mode: 0,
            ip_address: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            ip_netmask: u32::from(std::net::Ipv4Addr::new(255, 255, 255, 0)),
            ip_gateway: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1)),
            dns_server: u32::from(std::net::Ipv4Addr::new(8, 8, 8, 8)),
            boreas_serial_number_part_1: 123456,
            boreas_serial_number_part_2: 789012,
            boreas_serial_number_part_3: 345678,
        };

        let bytes: Vec<u8> = packet.try_into().expect("Failed to serialize");
        assert_eq!(bytes.len(), 30, "IpConfigurationPacket should be exactly 30 bytes");
        println!("IpConfigurationPacket length: {} bytes", bytes.len());
    }

    #[test]
    fn test_system_packet_round_trips() {
        // Test that all system packets can round-trip correctly
        
        // AcknowledgePacket round-trip
        let ack_original = AcknowledgePacket {
            packet_id: 202,
            packet_crc: 0xABCD,
            result: 2,
        };
        let ack_bytes: Vec<u8> = ack_original.clone().try_into().unwrap();
        let ack_deserialized: AcknowledgePacket = ack_bytes.try_into().unwrap();
        assert_eq!(ack_original, ack_deserialized);

        // DeviceInformationPacket round-trip
        let dev_original = DeviceInformationPacket {
            software_version: 0x12345678,
            device_id: 0x87654321,
            hardware_revision: 0xDEADBEEF,
            serial_number_1: 987654,
            serial_number_2: 321098,
            serial_number_3: 765432,
        };
        let dev_bytes: Vec<u8> = dev_original.clone().try_into().unwrap();
        let dev_deserialized: DeviceInformationPacket = dev_bytes.try_into().unwrap();
        assert_eq!(dev_original, dev_deserialized);

        // IpConfigurationPacket round-trip
        let ip_original = IpConfigurationPacket {
            permanent: 0,
            dhcp_mode: 1,
            ip_address: 0,
            ip_netmask: 0,
            ip_gateway: 0,
            dns_server: 0,
            boreas_serial_number_part_1: 111111,
            boreas_serial_number_part_2: 222222,
            boreas_serial_number_part_3: 333333,
        };
        let ip_bytes: Vec<u8> = ip_original.clone().try_into().unwrap();
        let ip_deserialized: IpConfigurationPacket = ip_bytes.try_into().unwrap();
        assert_eq!(ip_original, ip_deserialized);

        println!("✅ All system packet round-trips successful with binrw");
    }

    #[test] 
    fn test_system_packet_size_summary() {
        // Summary test that prints all system packet sizes for reference
        println!("\n=== System Packet Size Summary ===");

        // Test AcknowledgePacket
        let ack = AcknowledgePacket {
            packet_id: 0,
            packet_crc: 0,
            result: 0,
        };
        let ack_bytes: Vec<u8> = ack.try_into().unwrap();
        println!("AcknowledgePacket (ID 0): Expected 4, Actual {}", ack_bytes.len());

        // Test RequestPacket
        let req = RequestPacket {
            packet_id: 0,
        };
        let req_bytes: Vec<u8> = req.try_into().unwrap();
        println!("RequestPacket (ID 1): Variable length, Actual {}", req_bytes.len());

        // Test BootModePacket
        let boot = BootModePacket {
            boot_mode: 0,
        };
        let boot_bytes: Vec<u8> = boot.try_into().unwrap();
        println!("BootModePacket (ID 2): Expected 1, Actual {}", boot_bytes.len());

        // Test DeviceInformationPacket
        let dev = DeviceInformationPacket {
            software_version: 0,
            device_id: 0,
            hardware_revision: 0,
            serial_number_1: 0,
            serial_number_2: 0,
            serial_number_3: 0,
        };
        let dev_bytes: Vec<u8> = dev.try_into().unwrap();
        println!("DeviceInformationPacket (ID 3): Expected 24, Actual {}", dev_bytes.len());

        // Test RestoreFactorySettingsPacket
        let restore = RestoreFactorySettingsPacket::new();
        let restore_bytes: Vec<u8> = restore.try_into().unwrap();
        println!("RestoreFactorySettingsPacket (ID 4): Expected 4, Actual {}", restore_bytes.len());

        // Test ResetPacket
        let reset = ResetPacket::new();
        let reset_bytes: Vec<u8> = reset.try_into().unwrap();
        println!("ResetPacket (ID 5): Expected 4, Actual {}", reset_bytes.len());

        // Test IpConfigurationPacket
        let ip = IpConfigurationPacket {
            permanent: 0,
            dhcp_mode: 0,
            ip_address: 0,
            ip_netmask: 0,
            ip_gateway: 0,
            dns_server: 0,
            boreas_serial_number_part_1: 0,
            boreas_serial_number_part_2: 0,
            boreas_serial_number_part_3: 0,
        };
        let ip_bytes: Vec<u8> = ip.try_into().unwrap();
        println!("IpConfigurationPacket (ID 11): Expected 30, Actual {} (binrw format)", ip_bytes.len());

        println!("=== End System Summary ===\n");
    }
}