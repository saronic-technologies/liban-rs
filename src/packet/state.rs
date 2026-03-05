use binrw::{binrw, BinRead, BinWrite};
use serde::{Serialize, Deserialize};

// ===========================================================================
// Enums and Status Types
// ===========================================================================

/// GNSS fix type enumeration
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssFixType {
    #[default]
    NoFix = 0,
    Fix2D = 1,
    Fix3D = 2,
    SbassFix = 3,
    DifferentialFix = 4,
    OmnistarFix = 5,
    RtkFloat = 6,
    RtkFixed = 7,
}

impl GnssFixType {
    fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::NoFix,
            1 => Self::Fix2D,
            2 => Self::Fix3D,
            3 => Self::SbassFix,
            4 => Self::DifferentialFix,
            5 => Self::OmnistarFix,
            6 => Self::RtkFloat,
            7 => Self::RtkFixed,
            _ => Self::NoFix,
        }
    }
}

/// Spoofing status for GNSS packets
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpoofingStatus {
    #[default]
    Unknown = 0,
    None = 1,
    DetectedAndMitigated = 2,
    DetectedAndUnmitigated = 3,
}

impl SpoofingStatus {
    fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::DetectedAndMitigated,
            3 => Self::DetectedAndUnmitigated,
            _ => Self::Unknown,
        }
    }
}

/// Interference status for GNSS packets
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceStatus {
    #[default]
    Unknown = 0,
    None = 1,
    DetectedAndMitigated = 2,
    DetectedAndUnmitigated = 3,
}

impl InterferenceStatus {
    fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::DetectedAndMitigated,
            3 => Self::DetectedAndUnmitigated,
            _ => Self::Unknown,
        }
    }
}

/// System status - individual boolean fields
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemStatus {
    pub system_failure: bool,
    pub accelerometer_sensor_failure: bool,
    pub gyroscope_sensor_failure: bool,
    pub magnetometer_sensor_failure: bool,
    pub pressure_sensor_failure: bool,
    pub gnss_failure: bool,
    pub accelerometer_over_range: bool,
    pub gyroscope_over_range: bool,
    pub magnetometer_over_range: bool,
    pub pressure_over_range: bool,
    pub minimum_temperature_alarm: bool,
    pub maximum_temperature_alarm: bool,
    pub low_voltage_alarm: bool,
    pub high_voltage_alarm: bool,
    pub gnss_antenna_disconnected: bool,
    pub serial_port_overflow_alarm: bool,
}

impl SystemStatus {
    pub fn from_raw(bits: u16) -> Self {
        Self {
            system_failure: bits & (1 << 0) != 0,
            accelerometer_sensor_failure: bits & (1 << 1) != 0,
            gyroscope_sensor_failure: bits & (1 << 2) != 0,
            magnetometer_sensor_failure: bits & (1 << 3) != 0,
            pressure_sensor_failure: bits & (1 << 4) != 0,
            gnss_failure: bits & (1 << 5) != 0,
            accelerometer_over_range: bits & (1 << 6) != 0,
            gyroscope_over_range: bits & (1 << 7) != 0,
            magnetometer_over_range: bits & (1 << 8) != 0,
            pressure_over_range: bits & (1 << 9) != 0,
            minimum_temperature_alarm: bits & (1 << 10) != 0,
            maximum_temperature_alarm: bits & (1 << 11) != 0,
            low_voltage_alarm: bits & (1 << 12) != 0,
            high_voltage_alarm: bits & (1 << 13) != 0,
            gnss_antenna_disconnected: bits & (1 << 14) != 0,
            serial_port_overflow_alarm: bits & (1 << 15) != 0,
        }
    }

    pub fn to_raw(&self) -> u16 {
        let mut bits = 0u16;
        if self.system_failure { bits |= 1 << 0; }
        if self.accelerometer_sensor_failure { bits |= 1 << 1; }
        if self.gyroscope_sensor_failure { bits |= 1 << 2; }
        if self.magnetometer_sensor_failure { bits |= 1 << 3; }
        if self.pressure_sensor_failure { bits |= 1 << 4; }
        if self.gnss_failure { bits |= 1 << 5; }
        if self.accelerometer_over_range { bits |= 1 << 6; }
        if self.gyroscope_over_range { bits |= 1 << 7; }
        if self.magnetometer_over_range { bits |= 1 << 8; }
        if self.pressure_over_range { bits |= 1 << 9; }
        if self.minimum_temperature_alarm { bits |= 1 << 10; }
        if self.maximum_temperature_alarm { bits |= 1 << 11; }
        if self.low_voltage_alarm { bits |= 1 << 12; }
        if self.high_voltage_alarm { bits |= 1 << 13; }
        if self.gnss_antenna_disconnected { bits |= 1 << 14; }
        if self.serial_port_overflow_alarm { bits |= 1 << 15; }
        bits
    }
}

/// Filter status
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterStatus {
    pub orientation_filter_initialised: bool,
    pub navigation_filter_initialised: bool,
    pub heading_initialised: bool,
    pub utc_time_initialised: bool,
    pub gnss_fix_type: GnssFixType,
    pub event1_flag: bool,
    pub event2_flag: bool,
    pub internal_gnss_enabled: bool,
    pub dual_antenna_heading_active: bool,
    pub velocity_heading_enabled: bool,
    pub atmospheric_altitude_enabled: bool,
    pub external_position_active: bool,
    pub external_velocity_active: bool,
    pub external_heading_active: bool,
}

impl FilterStatus {
    pub fn from_raw(bits: u16) -> Self {
        Self {
            orientation_filter_initialised: bits & (1 << 0) != 0,
            navigation_filter_initialised: bits & (1 << 1) != 0,
            heading_initialised: bits & (1 << 2) != 0,
            utc_time_initialised: bits & (1 << 3) != 0,
            gnss_fix_type: GnssFixType::from_bits(((bits >> 4) & 0x07) as u8),
            event1_flag: bits & (1 << 7) != 0,
            event2_flag: bits & (1 << 8) != 0,
            internal_gnss_enabled: bits & (1 << 9) != 0,
            dual_antenna_heading_active: bits & (1 << 10) != 0,
            velocity_heading_enabled: bits & (1 << 11) != 0,
            atmospheric_altitude_enabled: bits & (1 << 12) != 0,
            external_position_active: bits & (1 << 13) != 0,
            external_velocity_active: bits & (1 << 14) != 0,
            external_heading_active: bits & (1 << 15) != 0,
        }
    }

    pub fn to_raw(&self) -> u16 {
        let mut bits = 0u16;
        if self.orientation_filter_initialised { bits |= 1 << 0; }
        if self.navigation_filter_initialised { bits |= 1 << 1; }
        if self.heading_initialised { bits |= 1 << 2; }
        if self.utc_time_initialised { bits |= 1 << 3; }
        bits |= (self.gnss_fix_type as u16) << 4;
        if self.event1_flag { bits |= 1 << 7; }
        if self.event2_flag { bits |= 1 << 8; }
        if self.internal_gnss_enabled { bits |= 1 << 9; }
        if self.dual_antenna_heading_active { bits |= 1 << 10; }
        if self.velocity_heading_enabled { bits |= 1 << 11; }
        if self.atmospheric_altitude_enabled { bits |= 1 << 12; }
        if self.external_position_active { bits |= 1 << 13; }
        if self.external_velocity_active { bits |= 1 << 14; }
        if self.external_heading_active { bits |= 1 << 15; }
        bits
    }
}

/// GNSS PVT status
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssPvtStatus {
    pub gnss_fix_status: GnssFixType,
    pub spoofing_status: SpoofingStatus,
    pub interference_status: InterferenceStatus,
    pub velocity_valid: bool,
    pub time_valid: bool,
    pub antenna_disconnected: bool,
    pub antenna_short: bool,
    pub gnss_failure: bool,
}

impl GnssPvtStatus {
    pub fn from_raw(bits: u16) -> Self {
        Self {
            gnss_fix_status: GnssFixType::from_bits((bits & 0x07) as u8),
            spoofing_status: SpoofingStatus::from_bits(((bits >> 3) & 0x07) as u8),
            interference_status: InterferenceStatus::from_bits(((bits >> 6) & 0x07) as u8),
            velocity_valid: bits & (1 << 9) != 0,
            time_valid: bits & (1 << 10) != 0,
            antenna_disconnected: bits & (1 << 11) != 0,
            antenna_short: bits & (1 << 12) != 0,
            gnss_failure: bits & (1 << 13) != 0,
        }
    }

    pub fn to_raw(&self) -> u16 {
        let mut bits = 0u16;
        bits |= self.gnss_fix_status as u16;
        bits |= (self.spoofing_status as u16) << 3;
        bits |= (self.interference_status as u16) << 6;
        if self.velocity_valid { bits |= 1 << 9; }
        if self.time_valid { bits |= 1 << 10; }
        if self.antenna_disconnected { bits |= 1 << 11; }
        if self.antenna_short { bits |= 1 << 12; }
        if self.gnss_failure { bits |= 1 << 13; }
        bits
    }
}

/// GNSS Orientation status
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssOrientationStatus {
    pub gnss_fix_status: GnssFixType,
    pub antenna_disconnected: bool,
    pub antenna_short: bool,
    pub gnss_failure: bool,
    pub spoofing_status: SpoofingStatus,
    pub interference_status: InterferenceStatus,
}

impl GnssOrientationStatus {
    pub fn from_raw(bits: u16) -> Self {
        Self {
            gnss_fix_status: GnssFixType::from_bits((bits & 0x07) as u8),
            antenna_disconnected: bits & (1 << 3) != 0,
            antenna_short: bits & (1 << 4) != 0,
            gnss_failure: bits & (1 << 5) != 0,
            spoofing_status: SpoofingStatus::from_bits(((bits >> 6) & 0x07) as u8),
            interference_status: InterferenceStatus::from_bits(((bits >> 9) & 0x07) as u8),
        }
    }

    pub fn to_raw(&self) -> u16 {
        let mut bits = 0u16;
        bits |= self.gnss_fix_status as u16;
        if self.antenna_disconnected { bits |= 1 << 3; }
        if self.antenna_short { bits |= 1 << 4; }
        if self.gnss_failure { bits |= 1 << 5; }
        bits |= (self.spoofing_status as u16) << 6;
        bits |= (self.interference_status as u16) << 9;
        bits
    }
}

// ===========================================================================
// Packet Structs
// ===========================================================================

/// System state packet (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SystemState {
    #[br(map = |x: u16| SystemStatus::from_raw(x))]
    #[bw(map = |x: &SystemStatus| x.to_raw())]
    pub system_status: SystemStatus,
    #[br(map = |x: u16| FilterStatus::from_raw(x))]
    #[bw(map = |x: &FilterStatus| x.to_raw())]
    pub filter_status: FilterStatus,
    pub unix_time_seconds: u32,
    pub microseconds: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
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
    pub latitude_std_dev: f32,
    pub longitude_std_dev: f32,
    pub height_std_dev: f32,
}

/// Unix time packet (Packet ID 21, Length 8) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct UnixTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

/// Status packet (Packet ID 23, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Status {
    #[br(map = |x: u16| SystemStatus::from_raw(x))]
    #[bw(map = |x: &SystemStatus| x.to_raw())]
    pub system_status: SystemStatus,
    #[br(map = |x: u16| FilterStatus::from_raw(x))]
    #[bw(map = |x: &FilterStatus| x.to_raw())]
    pub filter_status: FilterStatus,
}

/// Position standard deviation packet (Packet ID 24, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PositionStdDev {
    /// Latitude standard deviation in meters
    pub latitude_std_dev: f32,
    /// Longitude standard deviation in meters
    pub longitude_std_dev: f32,
    /// Height standard deviation in meters
    pub height_std_dev: f32,
}

/// Velocity standard deviation packet (Packet ID 25, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct VelocityStdDev {
    /// Velocity north standard deviation in m/s
    pub velocity_north_std_dev: f32,
    /// Velocity east standard deviation in m/s
    pub velocity_east_std_dev: f32,
    /// Velocity down standard deviation in m/s
    pub velocity_down_std_dev: f32,
}

/// Euler orientation standard deviation packet (Packet ID 26, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct EulerOrientationStdDev {
    pub roll_std_dev: f32,
    pub pitch_std_dev: f32,
    pub heading_std_dev: f32,
}

/// Raw sensors packet (Packet ID 28, Length 48) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSensors {
    pub accelerometer_x: f32,
    pub accelerometer_y: f32,
    pub accelerometer_z: f32,
    pub gyroscope_x: f32,
    pub gyroscope_y: f32,
    pub gyroscope_z: f32,
    #[br(temp)]
    #[bw(calc = 0.0f32)]
    _reserved1: f32,
    #[br(temp)]
    #[bw(calc = 0.0f32)]
    _reserved2: f32,
    #[br(temp)]
    #[bw(calc = 0.0f32)]
    _reserved3: f32,
    pub imu_temperature: f32,
    pub pressure: f32,
    pub pressure_temperature: f32,
}

/// Satellites packet (Packet ID 30, Length 13) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Satellites {
    pub hdop: f32,
    pub vdop: f32,
    pub gps_satellites: u8,
    pub glonass_satellites: u8,
    pub beidou_satellites: u8,
    pub galileo_satellites: u8,
    pub sbas_satellites: u8,
}

/// External time packet (Packet ID 52, Length 8) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

/// Heave packet (Packet ID 58, Length 16) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Heave {
    pub heave_point_1: f32,
    pub heave_point_2: f32,
    pub heave_point_3: f32,
    pub heave_point_4: f32,
}

/// Sensor temperature packet (Packet ID 85, Length 32) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorTemperature {
    pub accelerometer_temp_0: f32,
    pub accelerometer_temp_1: f32,
    pub accelerometer_temp_2: f32,
    pub gyroscope_temp_0: f32,
    pub gyroscope_temp_1: f32,
    pub gyroscope_temp_2: f32,
    #[br(temp)]
    #[bw(calc = 0.0f32)]
    _reserved: f32,
    pub pressure_sensor_temp: f32,
}

/// GNSS Position Velocity Time packet (Packet ID 92, Length 76) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssPositionVelocityTime {
    pub gnss_id: u8,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
    #[br(map = |x: u16| GnssPvtStatus::from_raw(x))]
    #[bw(map = |x: &GnssPvtStatus| x.to_raw())]
    pub status: GnssPvtStatus,
    pub posix_time_seconds: u32,
    pub posix_time_microseconds: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub position_std_dev_north: f32,
    pub position_std_dev_east: f32,
    pub position_std_dev_down: f32,
    pub velocity_north: f32,
    pub velocity_east: f32,
    pub velocity_down: f32,
    pub velocity_std_dev_north: f32,
    pub velocity_std_dev_east: f32,
    pub velocity_std_dev_down: f32,
    pub latency: u32,
}

/// GNSS Orientation packet (Packet ID 93, Length 36) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssOrientation {
    pub gnss_id: u8,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
    #[br(map = |x: u16| GnssOrientationStatus::from_raw(x))]
    #[bw(map = |x: &GnssOrientationStatus| x.to_raw())]
    pub status: GnssOrientationStatus,
    pub posix_time_seconds: u32,
    pub posix_time_microseconds: u32,
    pub azimuth: f32,
    pub azimuth_std_dev: f32,
    pub tilt: f32,
    pub tilt_std_dev: f32,
    pub baseline_length: f32,
    pub latency: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_round_trip() {
        let status = SystemStatus {
            system_failure: true,
            gnss_failure: true,
            gnss_antenna_disconnected: true,
            ..Default::default()
        };
        let raw = status.to_raw();
        let back = SystemStatus::from_raw(raw);
        assert_eq!(status, back);
    }

    #[test]
    fn test_filter_status_round_trip() {
        let status = FilterStatus {
            orientation_filter_initialised: true,
            heading_initialised: true,
            gnss_fix_type: GnssFixType::RtkFixed,
            internal_gnss_enabled: true,
            ..Default::default()
        };
        let raw = status.to_raw();
        let back = FilterStatus::from_raw(raw);
        assert_eq!(status, back);
    }

    #[test]
    fn test_gnss_pvt_status_round_trip() {
        let status = GnssPvtStatus {
            gnss_fix_status: GnssFixType::Fix3D,
            spoofing_status: SpoofingStatus::None,
            interference_status: InterferenceStatus::DetectedAndMitigated,
            velocity_valid: true,
            time_valid: true,
            ..Default::default()
        };
        let raw = status.to_raw();
        let back = GnssPvtStatus::from_raw(raw);
        assert_eq!(status, back);
    }

    #[test]
    fn test_gnss_orientation_status_round_trip() {
        let status = GnssOrientationStatus {
            gnss_fix_status: GnssFixType::RtkFloat,
            antenna_disconnected: true,
            spoofing_status: SpoofingStatus::DetectedAndUnmitigated,
            interference_status: InterferenceStatus::None,
            ..Default::default()
        };
        let raw = status.to_raw();
        let back = GnssOrientationStatus::from_raw(raw);
        assert_eq!(status, back);
    }

    #[test]
    fn test_system_state_serialization() {
        use std::f64::consts::PI;

        let system_state = SystemState {
            system_status: SystemStatus::default(),
            filter_status: FilterStatus {
                orientation_filter_initialised: true,
                navigation_filter_initialised: true,
                heading_initialised: true,
                ..Default::default()
            },
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

        let mut cursor = std::io::Cursor::new(Vec::new());
        system_state.write_le(&mut cursor).unwrap();
        let serialized = cursor.into_inner();
        assert_eq!(serialized.len(), 100);

        let mut cursor = std::io::Cursor::new(&serialized);
        let deserialized = SystemState::read_le(&mut cursor).unwrap();
        assert_eq!(system_state, deserialized);
    }
}

#[cfg(test)]
#[path = "tests/state.rs"]
mod state_length_tests;
