use crate::error::{AnError, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use bitflags::bitflags;

/// ANPP packet identifiers for Advanced Navigation devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PacketId {
    // System Packets
    Acknowledge = 0,
    Request = 1,
    BootMode = 2,
    DeviceInformation = 3,
    RestoreFactorySettings = 4,
    Reset = 5,
    IpConfiguration = 11,
    // State Packets
    SystemState = 20,
    UnixTime = 21,
    Status = 23,
    // Configuration packets
    PacketTimerPeriod = 180,
    PacketsPeriod = 181,
    InstallationAlignment = 185,
    FilterOptions = 186,
    OdometerConfiguration = 192,
    SetZeroOrientationAlignment = 193,
    ReferencePointOffsets = 194,
    IpDataportsConfiguration = 202,
}

impl PacketId {
    /// Convert a u8 value to a PacketId
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Acknowledge),
            1 => Some(Self::Request),
            2 => Some(Self::BootMode),
            3 => Some(Self::DeviceInformation),
            4 => Some(Self::RestoreFactorySettings),
            5 => Some(Self::Reset),
            11 => Some(Self::IpConfiguration),
            20 => Some(Self::SystemState),
            21 => Some(Self::UnixTime),
            23 => Some(Self::Status),
            180 => Some(Self::PacketTimerPeriod),
            181 => Some(Self::PacketsPeriod),
            185 => Some(Self::InstallationAlignment),
            186 => Some(Self::FilterOptions),
            192 => Some(Self::OdometerConfiguration),
            193 => Some(Self::SetZeroOrientationAlignment),
            194 => Some(Self::ReferencePointOffsets),
            202 => Some(Self::IpDataportsConfiguration),
            _ => None,
        }
    }

    /// Get the u8 value of the PacketId
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}



/// Macro to implement TryFrom<&[u8]>, TryFrom<Vec<u8>>, and TryInto<Vec<u8>> for ANPP packet types
macro_rules! impl_packet_conversions {
    ($packet_type:ty) => {
        impl TryFrom<&[u8]> for $packet_type {
            type Error = AnError;

            fn try_from(data: &[u8]) -> Result<Self> {
                bincode::deserialize(data)
                    .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize {}: {}", stringify!($packet_type), e)))
            }
        }

        impl TryFrom<Vec<u8>> for $packet_type {
            type Error = AnError;

            fn try_from(data: Vec<u8>) -> Result<Self> {
                Self::try_from(data.as_slice())
            }
        }

        impl TryInto<Vec<u8>> for $packet_type {
            type Error = AnError;

            fn try_into(self) -> Result<Vec<u8>> {
                bincode::serialize(&self)
                    .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize {}: {}", stringify!($packet_type), e)))
            }
        }
    };
}

/// System Packets (0-14)

/// Acknowledge packet structure (Packet ID 0, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgePacket {
    pub packet_id: u8,
    pub packet_crc: u16,
    pub result: u8,
}

impl_packet_conversions!(AcknowledgePacket);

/// Request packet structure (Packet ID 1, Variable length) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestPacket {
    pub packet_id: u8,
}

impl_packet_conversions!(RequestPacket);

/// Boot mode packet structure (Packet ID 2, Length 1) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootModePacket {
    pub boot_mode: u8,
}

impl_packet_conversions!(BootModePacket);

/// Device information packet structure (Packet ID 3, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInformation {
    pub software_version: u32,
    pub device_id: u32,
    pub hardware_revision: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}

impl_packet_conversions!(DeviceInformation);

impl DeviceInformation {
    /// Get the complete serial number as a formatted string
    pub fn get_serial_number(&self) -> String {
        format!(
            "{}-{}-{}",
            self.serial_number_1, self.serial_number_2, self.serial_number_3
        )
    }
}

/// Restore factory settings packet structure (Packet ID 4, Length 4) - Write only
///
/// Note: A Factory Reset will re-enable the DHCP Client and lose any static IP address settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreFactorySettingsPacket {
    pub verification: u32, // Verification code (must be 0x85429E1C)
}

impl_packet_conversions!(RestoreFactorySettingsPacket);

impl RestoreFactorySettingsPacket {
    /// Create a new restore factory settings packet with the correct verification code
    pub fn new() -> Self {
        Self {
            verification: 0x85429E1C,
        }
    }
}

impl Default for RestoreFactorySettingsPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Reset packet structure (Packet ID 5, Length 4) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPacket {
    pub verification: u32, // Verification code (must be 0x21057A7E)
}

impl_packet_conversions!(ResetPacket);

impl ResetPacket {
    /// Create a new reset packet with the correct verification code
    pub fn new() -> Self {
        Self {
            verification: 0x21057A7E,
        }
    }
}

impl Default for ResetPacket {
    fn default() -> Self {
        Self::new()
    }
}


/// IP configuration packet structure (Packet ID 11, Length 30) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpConfigurationPacket {
    pub permanent: u8,
    pub dhcp_mode: u8,
    pub ip_address: u32,
    pub ip_netmask: u32,
    pub ip_gateway: u32,
    pub dns_server: u32,
    pub hostname: [u8; 16],
}

impl_packet_conversions!(IpConfigurationPacket);


/// State Packets (20-23)

/// System state packet structure (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemState {
    /// System Status (Field 1)
    pub system_status: SystemStatus,
    /// Filter Status (Field 2)
    pub filter_status: FilterStatus,
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

impl_packet_conversions!(SystemState);


bitflags! {
    /// System status bitmask for SystemState
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct SystemStatus: u16 {
        const SYSTEM_FAILURE = 1 << 0;
        const ACCELEROMETER_SENSOR_FAILURE = 1 << 1;
        const GYROSCOPE_SENSOR_FAILURE = 1 << 2;
        const MAGNETOMETER_SENSOR_FAILURE = 1 << 3;
        const PRESSURE_SENSOR_FAILURE = 1 << 4;
        const GNSS_FAILURE = 1 << 5;
        const ACCELEROMETER_OVER_RANGE = 1 << 6;
        const GYROSCOPE_OVER_RANGE = 1 << 7;
        const MAGNETOMETER_OVER_RANGE = 1 << 8;
        const PRESSURE_OVER_RANGE = 1 << 9;
        const MINIMUM_TEMPERATURE_ALARM = 1 << 10;
        const MAXIMUM_TEMPERATURE_ALARM = 1 << 11;
        const LOW_VOLTAGE_ALARM = 1 << 12;
        const HIGH_VOLTAGE_ALARM = 1 << 13;
        const GNSS_ANTENNA_DISCONNECTED = 1 << 14;
        const SERIAL_PORT_OVERFLOW_ALARM = 1 << 15;
    }
}

bitflags! {
    /// Filter status bitmask for SystemState
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct FilterStatus: u16 {
        const ORIENTATION_FILTER_INITIALISED = 1 << 0;
        const NAVIGATION_FILTER_INITIALISED = 1 << 1;
        const HEADING_INITIALISED = 1 << 2;
        const UTC_TIME_INITIALISED = 1 << 3;
        const GNSS_FIX_TYPE_MASK = 0x0070; // Bits 4-6
        const EVENT1_FLAG = 1 << 7;
        const EVENT2_FLAG = 1 << 8;
        const INTERNAL_GNSS_ENABLED = 1 << 9;
        const DUAL_ANTENNA_HEADING_ACTIVE = 1 << 10;
        const VELOCITY_HEADING_ENABLED = 1 << 11;
        const ATMOSPHERIC_ALTITUDE_ENABLED = 1 << 12;
        const EXTERNAL_POSITION_ACTIVE = 1 << 13;
        const EXTERNAL_VELOCITY_ACTIVE = 1 << 14;
        const EXTERNAL_HEADING_ACTIVE = 1 << 15;
    }
}

impl FilterStatus {
    /// Get the GNSS fix type from the filter status
    pub fn gnss_fix_type(self) -> u8 {
        ((self.bits() & Self::GNSS_FIX_TYPE_MASK.bits()) >> 4) as u8
    }
}

/// Unix time packet structure (Packet ID 21, Length 8) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnixTimePacket {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

impl_packet_conversions!(UnixTimePacket);


/// Status packet structure (Packet ID 23, Variable length) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusPacket {
    pub system_status: SystemStatus,
    pub filter_status: FilterStatus,
    pub unix_time_seconds: u32,
    pub microseconds: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub height: f32,
    pub velocity_north: f32,
    pub velocity_east: f32,
    pub velocity_down: f32,
    pub body_acceleration_x: f32,
    pub body_acceleration_y: f32,
    pub body_acceleration_z: f32,
    pub g_force: f32,
    pub roll: f32,
    pub pitch: f32,
    pub heading: f32,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub standard_deviation_latitude: f32,
    pub standard_deviation_longitude: f32,
    pub standard_deviation_height: f32,
}

impl_packet_conversions!(StatusPacket);

/// Configuration Packets (180-203)

/// Packet timer period packet structure (Packet ID 180, Length 20) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketTimerPeriodPacket {
    pub packet_timer_period: [u32; 5],
}

impl_packet_conversions!(PacketTimerPeriodPacket);

/// Packets period packet structure (Packet ID 181, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketsPeriodPacket {
    pub packet_periods: Vec<PacketPeriodEntry>,
}

impl_packet_conversions!(PacketsPeriodPacket);

/// Entry for packets period configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketPeriodEntry {
    pub packet_id: u8,
    pub period: u32,
}

/// Installation alignment packet structure (Packet ID 185, Length 48) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationAlignmentPacket {
    pub permanent: u8,
    pub alignment_dcm: [[f32; 3]; 3], // 3x3 Direction Cosine Matrix
    pub gnss_antenna_offset: OffsetVector,
    pub odometer_offset: OffsetVector,
    pub external_data_offset: OffsetVector,
}

impl_packet_conversions!(InstallationAlignmentPacket);

/// 3D offset vector for installation alignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Filter options packet structure (Packet ID 186, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptionsPacket {
    pub permanent: u8,
    pub vehicle_type: VehicleType,
    pub internal_gnss_enabled: u8,
    pub atmospheric_altitude_enabled: u8,
    pub velocity_heading_enabled: u8,
    pub reversing_detection_enabled: u8,
    pub motion_analysis_enabled: u8,
    pub automatic_magnetic_calibration_enabled: u8,
    pub magnetic_heading_enabled: u8,
    pub odometer_mode: u8,
    pub odometer_pulse_to_metre_ratio: f32,
    pub dual_antenna_heading_enabled: u8,
}

impl_packet_conversions!(FilterOptionsPacket);

/// Vehicle type enumeration for filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum VehicleType {
    Car = 0,
    Motorcycle = 1,
    Aircraft = 2,
    Boat = 3,
    PersonalWatercraft = 4,
    Ship = 5,
    Helicopter = 6,
    Train = 7,
    FixedWing = 8,
    MultiRotor = 9,
    Tank = 10,
    Truck = 11,
    ArmoredVehicle = 12,
    Bus = 13,
    Excavator = 14,
    Bulldozer = 15,
}

/// Odometer configuration packet structure (Packet ID 192, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerConfigurationPacket {
    pub permanent: u8,
    pub automatic_pulse_measurement: u8,
    pub pulse_length_filtering: u8,
    pub minimum_pulse_length: u32,
    pub maximum_pulse_length: u32,
    pub pulse_to_metre_ratio: f32,
}

impl_packet_conversions!(OdometerConfigurationPacket);

/// Set zero orientation alignment packet structure (Packet ID 193, Length 1) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetZeroOrientationAlignmentPacket {
    pub permanent: u8,
}

impl_packet_conversions!(SetZeroOrientationAlignmentPacket);

/// Reference point offsets packet structure (Packet ID 194, Length 13) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub offset: OffsetVector,
}

impl_packet_conversions!(ReferencePointOffsetsPacket);

/// IP dataports configuration packet structure (Packet ID 202, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfigurationPacket {
    pub permanent: u8,
    pub entries: Vec<IpDataportEntry>,
}

impl_packet_conversions!(IpDataportsConfigurationPacket);

/// IP dataport entry for IP dataports configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportEntry {
    pub dataport_id: u8,
    pub mode: IpDataportMode,
    pub ip_address: u32,
    pub port: u16,
}

/// IP dataport mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum IpDataportMode {
    Disabled = 0,
    TcpServer = 1,
    TcpClient = 2,
    UdpServer = 3,
    UdpClient = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_packet_try_from_try_into() {
        // Test serialization and deserialization using TryFrom/TryInto
        let original_packet = ResetPacket::new();

        // Serialize using TryInto
        let serialized: Vec<u8> = original_packet.clone().try_into().unwrap();

        // Deserialize using TryFrom
        let deserialized = ResetPacket::try_from(serialized.as_slice()).unwrap();

        assert_eq!(original_packet, deserialized);
        assert_eq!(deserialized.verification, 0x21057A7E);
    }

    #[test]
    fn test_device_information_try_from_vec() {
        // Create a mock device information
        let device_info = DeviceInformation {
            software_version: 123,
            device_id: 456,
            hardware_revision: 789,
            serial_number_1: 111,
            serial_number_2: 222,
            serial_number_3: 333,
        };

        // Serialize to Vec<u8>
        let serialized: Vec<u8> = device_info.clone().try_into().unwrap();

        // Test TryFrom<Vec<u8>>
        let deserialized = DeviceInformation::try_from(serialized).unwrap();

        assert_eq!(device_info, deserialized);
        assert_eq!(deserialized.get_serial_number(), "111-222-333");
    }

    #[test]
    fn test_boot_mode_packet_try_from_slice() {
        let boot_mode = BootModePacket { boot_mode: 42 };

        let serialized: Vec<u8> = boot_mode.clone().try_into().unwrap();

        // Test TryFrom<&[u8]>
        let deserialized = BootModePacket::try_from(serialized.as_slice()).unwrap();

        assert_eq!(boot_mode, deserialized);
        assert_eq!(deserialized.boot_mode, 42);
    }

    #[test]
    fn test_system_status_bitflags() {
        // Test individual flags
        let status = SystemStatus::SYSTEM_FAILURE | SystemStatus::GNSS_FAILURE;

        assert!(status.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(status.contains(SystemStatus::GNSS_FAILURE));
        assert!(!status.contains(SystemStatus::LOW_VOLTAGE_ALARM));

        // Test serialization/deserialization with minimal SystemState
        let system_state = SystemState {
            system_status: status,
            filter_status: FilterStatus::ORIENTATION_FILTER_INITIALISED | FilterStatus::HEADING_INITIALISED,
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

        let serialized: Vec<u8> = system_state.clone().try_into().unwrap();
        let deserialized = SystemState::try_from(serialized).unwrap();

        assert_eq!(system_state, deserialized);
        assert!(deserialized.system_status.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(deserialized.system_status.contains(SystemStatus::GNSS_FAILURE));
    }

    #[test]
    fn test_filter_status_bitflags() {
        // Test individual flags and combinations
        let filter_status = FilterStatus::NAVIGATION_FILTER_INITIALISED
                          | FilterStatus::UTC_TIME_INITIALISED
                          | FilterStatus::INTERNAL_GNSS_ENABLED;

        assert!(filter_status.contains(FilterStatus::NAVIGATION_FILTER_INITIALISED));
        assert!(filter_status.contains(FilterStatus::UTC_TIME_INITIALISED));
        assert!(filter_status.contains(FilterStatus::INTERNAL_GNSS_ENABLED));
        assert!(!filter_status.contains(FilterStatus::HEADING_INITIALISED));

        // Test GNSS fix type extraction (bits 4-6)
        let filter_with_gnss_fix = FilterStatus::from_bits_truncate(0b0110000); // Fix type 3
        assert_eq!(filter_with_gnss_fix.gnss_fix_type(), 3);

        let filter_with_different_fix = FilterStatus::from_bits_truncate(0b0010000); // Fix type 1
        assert_eq!(filter_with_different_fix.gnss_fix_type(), 1);
    }

    #[test]
    fn test_bitflags_operations() {
        let status1 = SystemStatus::SYSTEM_FAILURE | SystemStatus::GNSS_FAILURE;
        let status2 = SystemStatus::GNSS_FAILURE | SystemStatus::LOW_VOLTAGE_ALARM;

        // Test union (|)
        let union = status1 | status2;
        assert!(union.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(union.contains(SystemStatus::GNSS_FAILURE));
        assert!(union.contains(SystemStatus::LOW_VOLTAGE_ALARM));

        // Test intersection (&)
        let intersection = status1 & status2;
        assert!(!intersection.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(intersection.contains(SystemStatus::GNSS_FAILURE));
        assert!(!intersection.contains(SystemStatus::LOW_VOLTAGE_ALARM));

        // Test difference (-)
        let difference = status1 - status2;
        assert!(difference.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(!difference.contains(SystemStatus::GNSS_FAILURE));
        assert!(!difference.contains(SystemStatus::LOW_VOLTAGE_ALARM));
    }

    #[test]
    fn test_system_state_serialization() {
        use std::f64::consts::PI;

        // Create a SystemState packet for serialization testing
        let system_state = SystemState {
            system_status: SystemStatus::empty(),
            filter_status: FilterStatus::ORIENTATION_FILTER_INITIALISED
                         | FilterStatus::NAVIGATION_FILTER_INITIALISED
                         | FilterStatus::HEADING_INITIALISED,
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
        let serialized: Vec<u8> = system_state.clone().try_into().unwrap();
        let deserialized = SystemState::try_from(serialized).unwrap();
        assert_eq!(system_state, deserialized);
    }
}