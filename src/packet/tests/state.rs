#[cfg(test)]
mod tests {
    use crate::packet::state::{
        SystemState, UnixTime, Status, PositionStdDev, VelocityStdDev,
        EulerOrientationStdDev, RawSensors, SensorTemperature,
        GnssPositionVelocityTime, GnssOrientation,
        SystemStatus, FilterStatus, GnssPvtStatus, GnssOrientationStatus,
        GnssFixType,
    };
    use binrw::{BinRead, BinWrite};

    #[test]
    fn test_system_state_packet_length() {
        let packet = SystemState {
            system_status: SystemStatus::default(),
            filter_status: FilterStatus {
                orientation_filter_initialised: true,
                ..Default::default()
            },
            unix_time_seconds: 1640995200,
            microseconds: 123456,
            latitude: 0.78539816,
            longitude: 0.52359878,
            height: 100.5,
            velocity_north: 1.5,
            velocity_east: 2.5,
            velocity_down: -0.1,
            body_acceleration_x: 0.02,
            body_acceleration_y: -0.01,
            body_acceleration_z: 9.81,
            g_force: 1.0,
            roll: 0.26179939,
            pitch: 0.17453293,
            heading: 1.5707963,
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
        assert_eq!(bytes.len(), 100, "SystemState should be 100 bytes");
    }

    #[test]
    fn test_unix_time_packet_length() {
        let packet = UnixTime {
            unix_time_seconds: 1640995200,
            microseconds: 123456,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 8, "UnixTime should be 8 bytes");
    }

    #[test]
    fn test_status_packet_length() {
        let packet = Status {
            system_status: SystemStatus { gnss_failure: true, low_voltage_alarm: true, ..Default::default() },
            filter_status: FilterStatus { orientation_filter_initialised: true, heading_initialised: true, ..Default::default() },
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 4, "Status should be 4 bytes");
    }

    #[test]
    fn test_position_std_dev_packet_length() {
        let packet = PositionStdDev {
            latitude_std_dev: 0.5,
            longitude_std_dev: 0.6,
            height_std_dev: 1.0,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "PositionStdDev should be 12 bytes");
    }

    #[test]
    fn test_velocity_std_dev_packet_length() {
        let packet = VelocityStdDev {
            velocity_north_std_dev: 0.1,
            velocity_east_std_dev: 0.2,
            velocity_down_std_dev: 0.3,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "VelocityStdDev should be 12 bytes");
    }

    #[test]
    fn test_euler_orientation_std_dev_packet_length() {
        let packet = EulerOrientationStdDev {
            roll_std_dev: 0.01,
            pitch_std_dev: 0.015,
            heading_std_dev: 0.02,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 12, "EulerOrientationStdDev should be 12 bytes");
    }

    #[test]
    fn test_raw_sensors_packet_length() {
        let packet = RawSensors {
            accelerometer_x: 0.02,
            accelerometer_y: -0.01,
            accelerometer_z: 9.81,
            gyroscope_x: 0.001,
            gyroscope_y: 0.002,
            gyroscope_z: 0.003,
            imu_temperature: 25.0,
            pressure: 101325.0,
            pressure_temperature: 24.5,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 48, "RawSensors should be 48 bytes");
    }

    #[test]
    fn test_sensor_temperature_packet_length() {
        let packet = SensorTemperature {
            accelerometer_temp_0: 25.0,
            accelerometer_temp_1: 25.1,
            accelerometer_temp_2: 25.2,
            gyroscope_temp_0: 26.0,
            gyroscope_temp_1: 26.1,
            gyroscope_temp_2: 26.2,
            pressure_sensor_temp: 24.5,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 32, "SensorTemperature should be 32 bytes");
    }

    #[test]
    fn test_gnss_position_velocity_time_packet_length() {
        let packet = GnssPositionVelocityTime {
            gnss_id: 0,
            status: GnssPvtStatus {
                gnss_fix_status: GnssFixType::Fix3D,
                velocity_valid: true,
                time_valid: true,
                ..Default::default()
            },
            posix_time_seconds: 1700000000,
            posix_time_microseconds: 500000,
            latitude: 0.78539816,
            longitude: 0.52359878,
            altitude: 100.5,
            position_std_dev_north: 0.5,
            position_std_dev_east: 0.6,
            position_std_dev_down: 1.0,
            velocity_north: 1.5,
            velocity_east: 2.5,
            velocity_down: -0.1,
            velocity_std_dev_north: 0.01,
            velocity_std_dev_east: 0.02,
            velocity_std_dev_down: 0.03,
            latency: 1000,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 76, "GnssPositionVelocityTime should be 76 bytes");
    }

    #[test]
    fn test_gnss_orientation_packet_length() {
        let packet = GnssOrientation {
            gnss_id: 0,
            status: GnssOrientationStatus {
                gnss_fix_status: GnssFixType::RtkFixed,
                ..Default::default()
            },
            posix_time_seconds: 1700000000,
            posix_time_microseconds: 500000,
            azimuth: 1.5707963,
            azimuth_std_dev: 0.01,
            tilt: 0.0,
            tilt_std_dev: 0.02,
            baseline_length: 1.0,
            latency: 500,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        packet.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 36, "GnssOrientation should be 36 bytes");
    }

    #[test]
    fn test_system_state_round_trip() {
        let original = SystemState {
            system_status: SystemStatus { system_failure: true, gnss_antenna_disconnected: true, ..Default::default() },
            filter_status: FilterStatus { navigation_filter_initialised: true, utc_time_initialised: true, ..Default::default() },
            unix_time_seconds: 1700000000,
            microseconds: 999999,
            latitude: -0.34906585,
            longitude: 2.6179939,
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

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 100);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = SystemState::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_gnss_pvt_round_trip() {
        let original = GnssPositionVelocityTime {
            gnss_id: 1,
            status: GnssPvtStatus {
                gnss_fix_status: GnssFixType::RtkFixed,
                velocity_valid: true,
                time_valid: true,
                ..Default::default()
            },
            posix_time_seconds: 1700000000,
            posix_time_microseconds: 123456,
            latitude: 0.6,
            longitude: -1.2,
            altitude: 50.0,
            position_std_dev_north: 0.01,
            position_std_dev_east: 0.02,
            position_std_dev_down: 0.03,
            velocity_north: 1.0,
            velocity_east: 2.0,
            velocity_down: -0.5,
            velocity_std_dev_north: 0.001,
            velocity_std_dev_east: 0.002,
            velocity_std_dev_down: 0.003,
            latency: 5000,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 76);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = GnssPositionVelocityTime::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_gnss_orientation_round_trip() {
        let original = GnssOrientation {
            gnss_id: 0,
            status: GnssOrientationStatus {
                gnss_fix_status: GnssFixType::RtkFloat,
                antenna_disconnected: false,
                antenna_short: false,
                gnss_failure: false,
                ..Default::default()
            },
            posix_time_seconds: 1700000000,
            posix_time_microseconds: 654321,
            azimuth: 1.5,
            azimuth_std_dev: 0.01,
            tilt: 0.05,
            tilt_std_dev: 0.02,
            baseline_length: 1.2,
            latency: 3000,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 36);

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = GnssOrientation::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_position_std_dev_round_trip() {
        let original = PositionStdDev {
            latitude_std_dev: 0.5,
            longitude_std_dev: 0.6,
            height_std_dev: 1.0,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = PositionStdDev::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_velocity_std_dev_round_trip() {
        let original = VelocityStdDev {
            velocity_north_std_dev: 0.1,
            velocity_east_std_dev: 0.2,
            velocity_down_std_dev: 0.3,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        original.write_le(&mut cursor).expect("Failed to serialize");
        let bytes = cursor.into_inner();

        let mut cursor = std::io::Cursor::new(&bytes);
        let deserialized = VelocityStdDev::read_le(&mut cursor).expect("Failed to deserialize");
        assert_eq!(deserialized, original);
    }
}
