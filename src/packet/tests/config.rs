#[cfg(test)]
mod tests {
    use crate::packet::config::{
        PacketTimerPeriodPacket, PacketsPeriodPacket, PacketPeriodEntry,
        InstallationAlignmentPacket, OffsetVector, FilterOptionsPacket, VehicleType,
        OdometerConfigurationPacket, SetZeroOrientationAlignmentPacket,
        ReferencePointOffsetsPacket, IpDataportsConfigurationPacket,
        IpDataportEntry, IpDataportMode
    };
    use binrw::{BinRead, BinWrite};

    #[test]
    fn test_packet_timer_period_packet_length() {
        // Packet ID 180, Length 4
        let packet = PacketTimerPeriodPacket {
            permanent: 1,
            utc_synchronisation: 1,
            packet_timer_period: 10000, // 10 ms in microseconds
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "PacketTimerPeriodPacket should be 4 bytes");
    }

    #[test]
    fn test_packets_period_packet_length_empty() {
        // Packet ID 181, Variable length - test with empty entries
        let packet = PacketsPeriodPacket {
            permanent: 0,
            clear_existing: 0,
            packet_periods: vec![],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        // Empty vector should have 2 bytes for permanent and clear_existing
        println!("Empty PacketsPeriodPacket length: {} bytes", bytes.len());
        assert_eq!(bytes.len(), 2, "Empty PacketsPeriodPacket should be 2 bytes (permanent + clear_existing)");
    }

    #[test]
    fn test_packets_period_packet_length_with_entries() {
        // Packet ID 181, Variable length - test with some entries
        let packet = PacketsPeriodPacket {
            permanent: 1,
            clear_existing: 0,
            packet_periods: vec![
                PacketPeriodEntry { packet_id: 20, period: 1000 },
                PacketPeriodEntry { packet_id: 21, period: 2000 },
            ],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        // permanent (1) + clear_existing (1) + 2 entries × (u8 + u32) = 2 + 2 × 5 = 12 bytes
        println!("PacketsPeriodPacket with 2 entries length: {} bytes", bytes.len());
        assert_eq!(bytes.len(), 12, "PacketsPeriodPacket with 2 entries should be 12 bytes");
    }

    #[test]
    fn test_packets_period_packet_round_trip() {
        // Test that binrw serialization/deserialization works correctly
        let original = PacketsPeriodPacket {
            permanent: 1,
            clear_existing: 1,
            packet_periods: vec![
                PacketPeriodEntry { packet_id: 180, period: 10000 },
                PacketPeriodEntry { packet_id: 186, period: 20000 },
                PacketPeriodEntry { packet_id: 202, period: 5000 },
            ],
        };

        // Serialize
        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        println!("PacketsPeriodPacket with 3 entries serialized to {} bytes", bytes.len());
        assert_eq!(bytes.len(), 17, "Should be 2 + 3×5 = 17 bytes");

        // Verify first two bytes are permanent and clear_existing
        assert_eq!(bytes[0], 1, "First byte should be permanent");
        assert_eq!(bytes[1], 1, "Second byte should be clear_existing");

        // Deserialize
        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = PacketsPeriodPacket::read_le(&mut cursor).expect("Failed to deserialize");

        // Verify round-trip consistency
        assert_eq!(deserialized.permanent, original.permanent);
        assert_eq!(deserialized.clear_existing, original.clear_existing);
        assert_eq!(deserialized.packet_periods, original.packet_periods);
        println!("✅ PacketsPeriodPacket round-trip successful with binrw");
    }

    #[test]
    fn test_installation_alignment_packet_length() {
        // Packet ID 185, Length 73
        let packet = InstallationAlignmentPacket {
            permanent: 1,
            alignment_dcm: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            gnss_antenna_offset: OffsetVector { x: 0.0, y: 0.0, z: 1.5 },
            odometer_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            external_data_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 73, "InstallationAlignmentPacket should be 73 bytes");
    }

    #[test]
    fn test_filter_options_packet_length() {
        // Packet ID 186, Length 17
        let packet = FilterOptionsPacket {
            permanent: 1,
            vehicle_type: VehicleType::Boat,
            internal_gnss_enabled: 1,
            reserved1: 0,
            atmospheric_altitude_enabled: 0,
            velocity_heading_enabled: 1,
            reversing_detection_enabled: 0,
            motion_analysis_enabled: 1,
            reserved2: 0,
            reserved3: [0; 8],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 17, "FilterOptionsPacket should be 17 bytes");
    }

    #[test]
    fn test_odometer_configuration_packet_length() {
        // Packet ID 192, Length 8
        let packet = OdometerConfigurationPacket {
            permanent: 1,
            automatic_pulse_measurement: 1,
            reserved: 0,
            pulse_length: 1.5,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8, "OdometerConfigurationPacket should be 8 bytes");
    }

    #[test]
    fn test_set_zero_orientation_alignment_packet_length() {
        // Packet ID 193, Length 1
        let packet = SetZeroOrientationAlignmentPacket {
            permanent: 1,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 1, "SetZeroOrientationAlignmentPacket should be 1 byte");
    }

    #[test]
    fn test_reference_point_offsets_packet_length() {
        // Packet ID 194, Length 49
        let packet = ReferencePointOffsetsPacket {
            permanent: 1,
            heave_point_1: OffsetVector { x: 1.0, y: 2.0, z: 3.0 },
            heave_point_2: OffsetVector { x: 4.0, y: 5.0, z: 6.0 },
            heave_point_3: OffsetVector { x: 7.0, y: 8.0, z: 9.0 },
            heave_point_4: OffsetVector { x: 10.0, y: 11.0, z: 12.0 },
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 49, "ReferencePointOffsetsPacket should be 49 bytes");
    }

    #[test]
    fn test_ip_dataports_configuration_packet_length() {
        // Packet ID 202, Length 30 - This uses binrw now!
        let disabled_entry = IpDataportEntry {
            ip_address: 0,
            port: 0,
            mode: IpDataportMode::Disabled,
        };

        let tcp_server_entry = IpDataportEntry {
            ip_address: 0,
            port: 17000,
            mode: IpDataportMode::TcpServer,
        };

        let packet = IpDataportsConfigurationPacket {
            reserved: 0,
            dataports: [disabled_entry, tcp_server_entry, disabled_entry, disabled_entry],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30, "IpDataportsConfigurationPacket should be exactly 30 bytes");
    }

    #[test]
    fn test_ip_dataports_configuration_packet_all_modes() {
        // Test all dataport modes to ensure consistent 30-byte length
        let entries = [
            IpDataportEntry { ip_address: 0, port: 0, mode: IpDataportMode::Disabled },
            IpDataportEntry { ip_address: 0, port: 17000, mode: IpDataportMode::TcpServer },
            IpDataportEntry { ip_address: u32::from(std::net::Ipv4Addr::new(192, 168, 0, 42)), port: 8080, mode: IpDataportMode::TcpClient },
            IpDataportEntry { ip_address: u32::from(std::net::Ipv4Addr::new(10, 0, 0, 1)), port: 9999, mode: IpDataportMode::UdpClient },
        ];

        let packet = IpDataportsConfigurationPacket {
            reserved: 1, // Test with permanent flag set
            dataports: entries,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30, "IpDataportsConfigurationPacket with all modes should be exactly 30 bytes");

        // Verify the packet structure
        assert_eq!(bytes[0], 1);  // reserved low byte (permanent flag)
        assert_eq!(bytes[1], 0);  // reserved high byte

        // Verify first entry (disabled)
        assert_eq!(&bytes[2..9], [0, 0, 0, 0, 0, 0, 0]); // IP=0, Port=0, Mode=0

        // Verify second entry (TCP server)
        assert_eq!(&bytes[9..16], [0, 0, 0, 0, 0x68, 0x42, 2]); // IP=0, Port=17000, Mode=2
    }

    #[test]
    fn test_ip_dataports_configuration_packet_round_trip() {
        // Test that binrw serialization/deserialization maintains exact 30 bytes
        let original = IpDataportsConfigurationPacket {
            reserved: 0,
            dataports: [
                IpDataportEntry { ip_address: 0, port: 0, mode: IpDataportMode::Disabled },
                IpDataportEntry { ip_address: 0, port: 8080, mode: IpDataportMode::TcpServer },
                IpDataportEntry { ip_address: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1)), port: 9090, mode: IpDataportMode::TcpClient },
                IpDataportEntry { ip_address: u32::from(std::net::Ipv4Addr::new(172, 16, 0, 1)), port: 5000, mode: IpDataportMode::UdpClient },
            ],
        };

        // Serialize
        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30, "Serialized packet should be 30 bytes");

        // Deserialize
        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = IpDataportsConfigurationPacket::read_le(&mut cursor).expect("Failed to deserialize");

        // Verify round-trip consistency
        assert_eq!(deserialized.reserved, original.reserved);
        assert_eq!(deserialized.dataports, original.dataports);
    }

    #[test] 
    fn test_packet_size_summary() {
        // Summary test that prints all packet sizes for reference
        println!("\n=== Packet Size Summary ===");

        // Test each packet type and print its actual size
        let timer_packet = PacketTimerPeriodPacket {
            permanent: 0,
            utc_synchronisation: 0,
            packet_timer_period: 1000,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        timer_packet.write_le(&mut cursor).unwrap();
        let timer_bytes = cursor.into_inner();
        println!("PacketTimerPeriodPacket (ID 180): Expected 4, Actual {}", timer_bytes.len());

        let periods_packet = PacketsPeriodPacket {
            permanent: 0,
            clear_existing: 0,
            packet_periods: vec![],
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        periods_packet.write_le(&mut cursor).unwrap();
        let periods_bytes = cursor.into_inner();
        println!("PacketsPeriodPacket (ID 181): Variable length, Empty = {} (binrw format)", periods_bytes.len());

        let alignment_packet = InstallationAlignmentPacket {
            permanent: 0,
            alignment_dcm: [[0.0; 3]; 3],
            gnss_antenna_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            odometer_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            external_data_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        alignment_packet.write_le(&mut cursor).unwrap();
        let alignment_bytes = cursor.into_inner();
        println!("InstallationAlignmentPacket (ID 185): Expected 73, Actual {}", alignment_bytes.len());

        let filter_packet = FilterOptionsPacket {
            permanent: 0,
            vehicle_type: VehicleType::Car,
            internal_gnss_enabled: 0,
            reserved1: 0,
            atmospheric_altitude_enabled: 0,
            velocity_heading_enabled: 0,
            reversing_detection_enabled: 0,
            motion_analysis_enabled: 0,
            reserved2: 0,
            reserved3: [0; 8],
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        filter_packet.write_le(&mut cursor).unwrap();
        let filter_bytes = cursor.into_inner();
        println!("FilterOptionsPacket (ID 186): Expected 17, Actual {}", filter_bytes.len());

        let odometer_packet = OdometerConfigurationPacket {
            permanent: 0,
            automatic_pulse_measurement: 0,
            reserved: 0,
            pulse_length: 0.0,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        odometer_packet.write_le(&mut cursor).unwrap();
        let odometer_bytes = cursor.into_inner();
        println!("OdometerConfigurationPacket (ID 192): Expected 8, Actual {}", odometer_bytes.len());

        let zero_align_packet = SetZeroOrientationAlignmentPacket {
            permanent: 0,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        zero_align_packet.write_le(&mut cursor).unwrap();
        let zero_align_bytes = cursor.into_inner();
        println!("SetZeroOrientationAlignmentPacket (ID 193): Expected 1, Actual {}", zero_align_bytes.len());

        let ref_point_packet = ReferencePointOffsetsPacket {
            permanent: 0,
            heave_point_1: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            heave_point_2: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            heave_point_3: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            heave_point_4: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        ref_point_packet.write_le(&mut cursor).unwrap();
        let ref_point_bytes = cursor.into_inner();
        println!("ReferencePointOffsetsPacket (ID 194): Expected 49, Actual {}", ref_point_bytes.len());

        let dataports_packet = IpDataportsConfigurationPacket {
            reserved: 0,
            dataports: [IpDataportEntry { ip_address: 0, port: 0, mode: IpDataportMode::Disabled }; 4],
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        dataports_packet.write_le(&mut cursor).unwrap();
        let dataports_bytes = cursor.into_inner();
        println!("IpDataportsConfigurationPacket (ID 202): Expected 30, Actual {} ✅", dataports_bytes.len());

        println!("=== End Summary ===\n");
    }
}