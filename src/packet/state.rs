use binrw::{BinRead, BinWrite};
use super::flags::{SystemStatusFlags, FilterStatusFlags};

/// State Packets (20-23)

/// System state packet structure (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct SystemStatePacket {
    /// System Status (Field 1)
    pub system_status: SystemStatusFlags,
    /// Filter Status (Field 2)
    pub filter_status: FilterStatusFlags,
    /// Unix Time Seconds (Field 3)
    pub unix_time_seconds: u32,
    /// Microseconds (Field 4)
    pub microseconds: u32,
    /// Latitude in radians (Field 5)
    pub latitude: f64,
    /// Longitude in radians (Field 6)
    pub longitude: f64,
    /// Height in meters (Field 7)
    pub height: f64,
    /// Velocity north in m/s (Field 8)
    pub velocity_north: f32,
    /// Velocity east in m/s (Field 9)
    pub velocity_east: f32,
    /// Velocity down in m/s (Field 10)
    pub velocity_down: f32,
    /// Body acceleration X in m/s/s (Field 11)
    pub body_acceleration_x: f32,
    /// Body acceleration Y in m/s/s (Field 12)
    pub body_acceleration_y: f32,
    /// Body acceleration Z in m/s/s (Field 13)
    pub body_acceleration_z: f32,
    /// G force in g (Field 14)
    pub g_force: f32,
    /// Roll in radians (Field 15)
    pub roll: f32,
    /// Pitch in radians (Field 16)
    pub pitch: f32,
    /// Heading in radians (Field 17)
    pub heading: f32,
    /// Angular velocity X in rad/s (Field 18)
    pub angular_velocity_x: f32,
    /// Angular velocity Y in rad/s (Field 19)
    pub angular_velocity_y: f32,
    /// Angular velocity Z in rad/s (Field 20)
    pub angular_velocity_z: f32,
    /// Latitude standard deviation in m (Field 21)
    pub latitude_std_dev: f32,
    /// Longitude standard deviation in m (Field 22)
    pub longitude_std_dev: f32,
    /// Height standard deviation in m (Field 23)
    pub height_std_dev: f32,
}


/// Unix time packet structure (Packet ID 21, Length 8) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct UnixTimePacket {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}


/// Status packet structure (Packet ID 23, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(little)]
pub struct StatusPacket {
    pub system_status: SystemStatusFlags,
    pub filter_status: FilterStatusFlags,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_bitflags() {
        // Test individual flags
        let status = SystemStatusFlags::SYSTEM_FAILURE | SystemStatusFlags::GNSS_FAILURE;

        assert!(status.contains(SystemStatusFlags::SYSTEM_FAILURE));
        assert!(status.contains(SystemStatusFlags::GNSS_FAILURE));
        assert!(!status.contains(SystemStatusFlags::LOW_VOLTAGE_ALARM));

        // Test serialization/deserialization with minimal SystemState
        let system_state = SystemStatePacket {
            system_status: status,
            filter_status: FilterStatusFlags::ORIENTATION_FILTER_INITIALISED | FilterStatusFlags::HEADING_INITIALISED,
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
        let serialized = cursor.into_inner();
        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = SystemStatePacket::read_le(&mut cursor).unwrap();

        assert_eq!(system_state, deserialized);
        assert!(deserialized.system_status.contains(SystemStatusFlags::SYSTEM_FAILURE));
        assert!(deserialized.system_status.contains(SystemStatusFlags::GNSS_FAILURE));
    }


    #[test]
    fn test_system_state_serialization() {
        use std::f64::consts::PI;

        // Create a SystemState packet for serialization testing
        let system_state = SystemStatePacket {
            system_status: SystemStatusFlags::empty(),
            filter_status: FilterStatusFlags::ORIENTATION_FILTER_INITIALISED
                         | FilterStatusFlags::NAVIGATION_FILTER_INITIALISED
                         | FilterStatusFlags::HEADING_INITIALISED,
            unix_time_seconds: 1640995200,
            microseconds: 123456,
            latitude: PI / 4.0,
            longitude: PI / 6.0,
            height: 100.5,
            velocity_north: 1.5,
            velocity_east: 2.5,
            velocity_down: -0.1,
            body_acceleration_x: 0.02,
            body_acceleration_y: -0.01,
            body_acceleration_z: 9.81,
            g_force: 1.0,
            roll: (PI / 12.0) as f32,
            pitch: (PI / 18.0) as f32,
            heading: (PI / 2.0) as f32,
            angular_velocity_x: 0.001,
            angular_velocity_y: 0.002,
            angular_velocity_z: 0.003,
            latitude_std_dev: 0.5,
            longitude_std_dev: 0.6,
            height_std_dev: 1.0,
        };

        // Test serialization/deserialization roundtrip
        let mut cursor = std::io::Cursor::new(Vec::new());
        system_state.write_le(&mut cursor).unwrap();
        let serialized = cursor.into_inner();
        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = SystemStatePacket::read_le(&mut cursor).unwrap();
        assert_eq!(system_state, deserialized);
    }
}

#[cfg(test)]
#[path = "tests/state.rs"]
mod state_length_tests;