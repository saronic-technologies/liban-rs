#[cfg(test)]
mod tests {
    use crate::packet::config::{
        PacketTimerPeriod, PacketsPeriod, PacketPeriod,
        InstallationAlignment, OffsetVector, FilterOptions, VehicleType,
        OdometerConfiguration, SetZeroOrientationAlignment,
        ReferencePointOffsets, IpDataportsConfiguration,
        IpDataport, IpDataportMode,
    };
    use crate::packet::PacketKind;
    use binrw::{BinRead, BinWrite};
    use std::time::Duration;

    #[test]
    fn test_packet_timer_period_packet_length() {
        let packet = PacketTimerPeriod {
            permanent: true,
            utc_synchronisation: true,
            packet_timer_period: Duration::from_millis(10000),
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "PacketTimerPeriod should be 4 bytes");
    }

    #[test]
    fn test_packets_period_packet_length_empty() {
        let packet = PacketsPeriod {
            permanent: false,
            clear_existing: false,
            packet_periods: vec![],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 2, "Empty PacketsPeriod should be 2 bytes");
    }

    #[test]
    fn test_packets_period_packet_length_with_entries() {
        let packet = PacketsPeriod {
            permanent: true,
            clear_existing: false,
            packet_periods: vec![
                PacketPeriod { packet_type: PacketKind::SystemState, period: Duration::from_millis(1000) },
                PacketPeriod { packet_type: PacketKind::UnixTime, period: Duration::from_millis(2000) },
            ],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "PacketsPeriod with 2 entries should be 12 bytes");
    }

    #[test]
    fn test_packets_period_packet_round_trip() {
        let original = PacketsPeriod {
            permanent: true,
            clear_existing: true,
            packet_periods: vec![
                PacketPeriod { packet_type: PacketKind::PacketTimerPeriod, period: Duration::from_millis(10000) },
                PacketPeriod { packet_type: PacketKind::FilterOptions, period: Duration::from_millis(20000) },
                PacketPeriod { packet_type: PacketKind::IpDataportsConfiguration, period: Duration::from_millis(5000) },
            ],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 17, "Should be 2 + 3x5 = 17 bytes");

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = PacketsPeriod::read_le(&mut cursor).expect("Failed to deserialize");

        assert_eq!(deserialized.permanent, original.permanent);
        assert_eq!(deserialized.clear_existing, original.clear_existing);
        assert_eq!(deserialized.packet_periods, original.packet_periods);
    }

    #[test]
    fn test_installation_alignment_packet_length() {
        let packet = InstallationAlignment {
            permanent: true,
            alignment_dcm: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            gnss_antenna_offset: OffsetVector { x: 0.0, y: 0.0, z: 1.5 },
            odometer_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
            external_data_offset: OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 73, "InstallationAlignment should be 73 bytes");
    }

    #[test]
    fn test_filter_options_packet_length() {
        let packet = FilterOptions {
            permanent: true,
            vehicle_type: VehicleType::Boat,
            internal_gnss_enabled: true,
            atmospheric_altitude_enabled: false,
            velocity_heading_enabled: true,
            reversing_detection_enabled: false,
            motion_analysis_enabled: true,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 17, "FilterOptions should be 17 bytes");
    }

    #[test]
    fn test_odometer_configuration_packet_length() {
        let packet = OdometerConfiguration {
            permanent: true,
            automatic_pulse_measurement: true,
            pulse_length: 1.5,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8, "OdometerConfiguration should be 8 bytes");
    }

    #[test]
    fn test_set_zero_orientation_alignment_packet_length() {
        let packet = SetZeroOrientationAlignment { permanent: true };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 1, "SetZeroOrientationAlignment should be 1 byte");
    }

    #[test]
    fn test_reference_point_offsets_packet_length() {
        let packet = ReferencePointOffsets {
            permanent: true,
            heave_point_1: OffsetVector { x: 1.0, y: 2.0, z: 3.0 },
            heave_point_2: OffsetVector { x: 4.0, y: 5.0, z: 6.0 },
            heave_point_3: OffsetVector { x: 7.0, y: 8.0, z: 9.0 },
            heave_point_4: OffsetVector { x: 10.0, y: 11.0, z: 12.0 },
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 49, "ReferencePointOffsets should be 49 bytes");
    }

    #[test]
    fn test_ip_dataports_configuration_packet_length() {
        let disabled_entry = IpDataport {
            ip_address: 0,
            port: 0,
            mode: IpDataportMode::Disabled,
        };

        let tcp_server_entry = IpDataport {
            ip_address: 0,
            port: 17000,
            mode: IpDataportMode::TcpServer,
        };

        let packet = IpDataportsConfiguration {
            dataports: [disabled_entry, tcp_server_entry, disabled_entry, disabled_entry],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30, "IpDataportsConfiguration should be exactly 30 bytes");
    }

    #[test]
    fn test_ip_dataports_configuration_packet_round_trip() {
        let original = IpDataportsConfiguration {
            dataports: [
                IpDataport { ip_address: 0, port: 0, mode: IpDataportMode::Disabled },
                IpDataport { ip_address: 0, port: 8080, mode: IpDataportMode::TcpServer },
                IpDataport { ip_address: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1)), port: 9090, mode: IpDataportMode::TcpClient },
                IpDataport { ip_address: u32::from(std::net::Ipv4Addr::new(172, 16, 0, 1)), port: 5000, mode: IpDataportMode::UdpClient },
            ],
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 30);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = IpDataportsConfiguration::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_filter_options_round_trip() {
        let original = FilterOptions {
            permanent: true,
            vehicle_type: VehicleType::Car,
            internal_gnss_enabled: true,
            atmospheric_altitude_enabled: false,
            velocity_heading_enabled: true,
            reversing_detection_enabled: false,
            motion_analysis_enabled: true,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 17);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = FilterOptions::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_odometer_configuration_round_trip() {
        let original = OdometerConfiguration {
            permanent: true,
            automatic_pulse_measurement: false,
            pulse_length: 2.5,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = OdometerConfiguration::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }
}
