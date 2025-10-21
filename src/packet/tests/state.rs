#[cfg(test)]
mod tests {
    use crate::{
        SystemStatePacket, UnixTimePacket, StatusPacket, EulerOrientationStdDevPacket,
        SystemStatusFlags, FilterStatusFlags
    };
    use binrw::{BinRead, BinWrite};

    #[test]
    fn test_system_state_packet_length() {
        // Packet ID 20, Length 100
        let packet = SystemStatePacket {
            system_status: SystemStatusFlags::empty(),
            filter_status: FilterStatusFlags::ORIENTATION_FILTER_INITIALISED,
            unix_time_seconds: 1640995200,
            microseconds: 123456,
            latitude: 0.78539816, // π/4 radians
            longitude: 0.52359878, // π/6 radians
            height: 100.5,
            velocity_north: 1.5,
            velocity_east: 2.5,
            velocity_down: -0.1,
            body_acceleration_x: 0.02,
            body_acceleration_y: -0.01,
            body_acceleration_z: 9.81,
            g_force: 1.0,
            roll: 0.26179939, // π/12 radians
            pitch: 0.17453293, // π/18 radians
            heading: 1.5707963, // π/2 radians
            angular_velocity_x: 0.001,
            angular_velocity_y: 0.002,
            angular_velocity_z: 0.003,
            latitude_std_dev: 0.5,
            longitude_std_dev: 0.6,
            height_std_dev: 1.0,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 100, "SystemStatePacket should be 100 bytes");
    }

    #[test]
    fn test_unix_time_packet_length() {
        // Packet ID 21, Length 8
        let packet = UnixTimePacket {
            unix_time_seconds: 1640995200,
            microseconds: 123456,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8, "UnixTimePacket should be 8 bytes");
    }

    #[test]
    fn test_status_packet_length() {
        // Packet ID 23, Length 4
        let packet = StatusPacket {
            system_status: SystemStatusFlags::GNSS_FAILURE | SystemStatusFlags::LOW_VOLTAGE_ALARM,
            filter_status: FilterStatusFlags::ORIENTATION_FILTER_INITIALISED | FilterStatusFlags::HEADING_INITIALISED,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "StatusPacket should be 4 bytes");
        println!("StatusPacket length: {} bytes", bytes.len());
    }

    #[test]
    fn test_system_state_packet_round_trip() {
        // Test that serialization/deserialization maintains data integrity
        let original = SystemStatePacket {
            system_status: SystemStatusFlags::SYSTEM_FAILURE | SystemStatusFlags::GNSS_ANTENNA_DISCONNECTED,
            filter_status: FilterStatusFlags::NAVIGATION_FILTER_INITIALISED | FilterStatusFlags::UTC_TIME_INITIALISED,
            unix_time_seconds: 1700000000,
            microseconds: 999999,
            latitude: -0.34906585, // -20 degrees in radians
            longitude: 2.6179939,  // 150 degrees in radians
            height: -50.25,
            velocity_north: -5.0,
            velocity_east: 10.2,
            velocity_down: 0.5,
            body_acceleration_x: -0.1,
            body_acceleration_y: 0.05,
            body_acceleration_z: 9.75,
            g_force: 0.98,
            roll: -0.17453293,
            pitch: 0.08726646,
            heading: 3.14159265,
            angular_velocity_x: -0.01,
            angular_velocity_y: 0.005,
            angular_velocity_z: -0.002,
            latitude_std_dev: 1.2,
            longitude_std_dev: 1.5,
            height_std_dev: 2.0,
        };

        // Serialize
        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 100, "SystemStatePacket should serialize to 100 bytes");

        // Deserialize
        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = SystemStatePacket::read_le(&mut cursor).expect("Failed to deserialize");

        // Verify round-trip consistency
        assert_eq!(deserialized.system_status, original.system_status);
        assert_eq!(deserialized.filter_status, original.filter_status);
        assert_eq!(deserialized.unix_time_seconds, original.unix_time_seconds);
        assert_eq!(deserialized.microseconds, original.microseconds);
        assert_eq!(deserialized.latitude, original.latitude);
        assert_eq!(deserialized.longitude, original.longitude);
        // Test a few more key fields
        assert_eq!(deserialized.height, original.height);
        assert_eq!(deserialized.heading, original.heading);

        println!("✅ SystemStatePacket round-trip successful with binrw");
    }

    #[test]
    fn test_unix_time_packet_round_trip() {
        // Test that UnixTimePacket serialization/deserialization works correctly
        let original = UnixTimePacket {
            unix_time_seconds: 1234567890,
            microseconds: 555555,
        };

        // Serialize
        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8, "UnixTimePacket should serialize to 8 bytes");

        // Deserialize
        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = UnixTimePacket::read_le(&mut cursor).expect("Failed to deserialize");

        // Verify round-trip consistency
        assert_eq!(deserialized, original);
        println!("✅ UnixTimePacket round-trip successful with binrw");
    }

    #[test]
    fn test_euler_orientation_std_dev_packet_length() {
        // Packet ID 26, Length 12
        let packet = EulerOrientationStdDevPacket {
            roll_std_dev: 0.01,
            pitch_std_dev: 0.015,
            heading_std_dev: 0.02,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "EulerOrientationStdDevPacket should be 12 bytes");
    }

    #[test]
    fn test_euler_orientation_std_dev_packet_round_trip() {
        // Test that EulerOrientationStdDevPacket serialization/deserialization works correctly
        let original = EulerOrientationStdDevPacket {
            roll_std_dev: 0.0523599,  // ~3 degrees in radians
            pitch_std_dev: 0.0349066, // ~2 degrees in radians
            heading_std_dev: 0.0872665, // ~5 degrees in radians
        };

        // Serialize
        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "EulerOrientationStdDevPacket should serialize to 12 bytes");

        // Deserialize
        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = EulerOrientationStdDevPacket::read_le(&mut cursor).expect("Failed to deserialize");

        // Verify round-trip consistency
        assert_eq!(deserialized, original);
        println!("✅ EulerOrientationStdDevPacket round-trip successful with binrw");
    }

    #[test]
    fn test_state_packet_size_summary() {
        // Summary test that prints all state packet sizes for reference
        println!("\n=== State Packet Size Summary ===");

        // Test SystemStatePacket
        let system_state = SystemStatePacket {
            system_status: SystemStatusFlags::empty(),
            filter_status: FilterStatusFlags::empty(),
            unix_time_seconds: 0,
            microseconds: 0,
            latitude: 0.0,
            longitude: 0.0,
            height: 0.0,
            velocity_north: 0.0,
            velocity_east: 0.0,
            velocity_down: 0.0,
            body_acceleration_x: 0.0,
            body_acceleration_y: 0.0,
            body_acceleration_z: 0.0,
            g_force: 0.0,
            roll: 0.0,
            pitch: 0.0,
            heading: 0.0,
            angular_velocity_x: 0.0,
            angular_velocity_y: 0.0,
            angular_velocity_z: 0.0,
            latitude_std_dev: 0.0,
            longitude_std_dev: 0.0,
            height_std_dev: 0.0,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        system_state.write_le(&mut cursor).unwrap();
        let system_state_bytes = cursor.into_inner();
        println!("SystemStatePacket (ID 20): Expected 100, Actual {}", system_state_bytes.len());

        // Test UnixTimePacket
        let unix_time = UnixTimePacket {
            unix_time_seconds: 0,
            microseconds: 0,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        unix_time.write_le(&mut cursor).unwrap();
        let unix_time_bytes = cursor.into_inner();
        println!("UnixTimePacket (ID 21): Expected 8, Actual {}", unix_time_bytes.len());

        // Test StatusPacket
        let status = StatusPacket {
            system_status: SystemStatusFlags::empty(),
            filter_status: FilterStatusFlags::empty(),
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        status.write_le(&mut cursor).unwrap();
        let status_bytes = cursor.into_inner();
        println!("StatusPacket (ID 23): Expected 4, Actual {}", status_bytes.len());

        // Test EulerOrientationStdDevPacket
        let euler_std_dev = EulerOrientationStdDevPacket {
            roll_std_dev: 0.0,
            pitch_std_dev: 0.0,
            heading_std_dev: 0.0,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        euler_std_dev.write_le(&mut cursor).unwrap();
        let euler_std_dev_bytes = cursor.into_inner();
        println!("EulerOrientationStdDevPacket (ID 26): Expected 12, Actual {}", euler_std_dev_bytes.len());

        println!("=== End State Summary ===\n");
    }
}