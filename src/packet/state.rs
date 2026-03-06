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
    PppFix = 5,
    RtkFloat = 6,
    RtkFixed = 7,
}

impl From<u8> for GnssFixType {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NoFix,
            1 => Self::Fix2D,
            2 => Self::Fix3D,
            3 => Self::SbassFix,
            4 => Self::DifferentialFix,
            5 => Self::PppFix,
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

impl From<u8> for SpoofingStatus {
    fn from(v: u8) -> Self {
        match v {
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

impl From<u8> for InterferenceStatus {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::DetectedAndMitigated,
            3 => Self::DetectedAndUnmitigated,
            _ => Self::Unknown,
        }
    }
}

/// System status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SystemStatus(u16);

impl SystemStatus {
    pub fn raw(&self) -> u16 { self.0 }
    pub fn system_failure(&self) -> bool { self.0 & (1 << 0) != 0 }
    pub fn accelerometer_sensor_failure(&self) -> bool { self.0 & (1 << 1) != 0 }
    pub fn gyroscope_sensor_failure(&self) -> bool { self.0 & (1 << 2) != 0 }
    pub fn magnetometer_sensor_failure(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn pressure_sensor_failure(&self) -> bool { self.0 & (1 << 4) != 0 }
    pub fn gnss_failure(&self) -> bool { self.0 & (1 << 5) != 0 }
    pub fn accelerometer_over_range(&self) -> bool { self.0 & (1 << 6) != 0 }
    pub fn gyroscope_over_range(&self) -> bool { self.0 & (1 << 7) != 0 }
    pub fn magnetometer_over_range(&self) -> bool { self.0 & (1 << 8) != 0 }
    pub fn pressure_over_range(&self) -> bool { self.0 & (1 << 9) != 0 }
    pub fn minimum_temperature_alarm(&self) -> bool { self.0 & (1 << 10) != 0 }
    pub fn maximum_temperature_alarm(&self) -> bool { self.0 & (1 << 11) != 0 }
    pub fn internal_data_logging_error(&self) -> bool { self.0 & (1 << 12) != 0 }
    pub fn high_voltage_alarm(&self) -> bool { self.0 & (1 << 13) != 0 }
    pub fn gnss_antenna_disconnected(&self) -> bool { self.0 & (1 << 14) != 0 }
    pub fn data_output_overflow_alarm(&self) -> bool { self.0 & (1 << 15) != 0 }
}

impl From<u16> for SystemStatus {
    fn from(v: u16) -> Self { Self(v) }
}

/// Filter status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct FilterStatus(u16);

impl FilterStatus {
    pub fn raw(&self) -> u16 { self.0 }
    pub fn orientation_filter_initialised(&self) -> bool { self.0 & (1 << 0) != 0 }
    pub fn navigation_filter_initialised(&self) -> bool { self.0 & (1 << 1) != 0 }
    pub fn heading_initialised(&self) -> bool { self.0 & (1 << 2) != 0 }
    pub fn utc_time_initialised(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn gnss_fix_type(&self) -> GnssFixType { GnssFixType::from(((self.0 >> 4) & 0x07) as u8) }
    pub fn event1_flag(&self) -> bool { self.0 & (1 << 7) != 0 }
    pub fn event2_flag(&self) -> bool { self.0 & (1 << 8) != 0 }
    pub fn internal_gnss_enabled(&self) -> bool { self.0 & (1 << 9) != 0 }
    pub fn dual_antenna_heading_active(&self) -> bool { self.0 & (1 << 10) != 0 }
    pub fn velocity_heading_enabled(&self) -> bool { self.0 & (1 << 11) != 0 }
    pub fn atmospheric_altitude_enabled(&self) -> bool { self.0 & (1 << 12) != 0 }
    pub fn external_position_active(&self) -> bool { self.0 & (1 << 13) != 0 }
    pub fn external_velocity_active(&self) -> bool { self.0 & (1 << 14) != 0 }
    pub fn external_heading_active(&self) -> bool { self.0 & (1 << 15) != 0 }
}

impl From<u16> for FilterStatus {
    fn from(v: u16) -> Self { Self(v) }
}

/// GNSS PVT status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct GnssPvtStatus(u16);

impl GnssPvtStatus {
    pub fn raw(&self) -> u16 { self.0 }
    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.0 & 0x07) as u8) }
    pub fn spoofing_status(&self) -> SpoofingStatus { SpoofingStatus::from(((self.0 >> 3) & 0x07) as u8) }
    pub fn interference_status(&self) -> InterferenceStatus { InterferenceStatus::from(((self.0 >> 6) & 0x07) as u8) }
    pub fn velocity_valid(&self) -> bool { self.0 & (1 << 9) != 0 }
    pub fn time_valid(&self) -> bool { self.0 & (1 << 10) != 0 }
    pub fn antenna_disconnected(&self) -> bool { self.0 & (1 << 11) != 0 }
    pub fn antenna_short(&self) -> bool { self.0 & (1 << 12) != 0 }
    pub fn gnss_failure(&self) -> bool { self.0 & (1 << 13) != 0 }
}

impl From<u16> for GnssPvtStatus {
    fn from(v: u16) -> Self { Self(v) }
}

/// GNSS Orientation status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct GnssOrientationStatus(u16);

impl GnssOrientationStatus {
    pub fn raw(&self) -> u16 { self.0 }
    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.0 & 0x07) as u8) }
    pub fn antenna_disconnected(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn antenna_short(&self) -> bool { self.0 & (1 << 4) != 0 }
    pub fn gnss_failure(&self) -> bool { self.0 & (1 << 5) != 0 }
    pub fn spoofing_status(&self) -> SpoofingStatus { SpoofingStatus::from(((self.0 >> 6) & 0x07) as u8) }
    pub fn interference_status(&self) -> InterferenceStatus { InterferenceStatus::from(((self.0 >> 9) & 0x07) as u8) }
}

impl From<u16> for GnssOrientationStatus {
    fn from(v: u16) -> Self { Self(v) }
}

// ===========================================================================
// Packet Structs
// ===========================================================================

/// System state packet (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SystemState {
    pub system_status: SystemStatus,
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
    pub system_status: SystemStatus,
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

/// Quaternion orientation standard deviation packet (Packet ID 27, Length 16) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct QuaternionOrientationStdDev {
    pub q0_std_dev: f32,
    pub q1_std_dev: f32,
    pub q2_std_dev: f32,
    pub q3_std_dev: f32,
}

/// Raw GNSS status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RawGnssStatus(u16);

impl RawGnssStatus {
    pub fn raw(&self) -> u16 { self.0 }
    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.0 & 0x07) as u8) }
    pub fn doppler_velocity_valid(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn time_valid(&self) -> bool { self.0 & (1 << 4) != 0 }
    pub fn external_gnss(&self) -> bool { self.0 & (1 << 5) != 0 }
    pub fn tilt_valid(&self) -> bool { self.0 & (1 << 6) != 0 }
    pub fn heading_valid(&self) -> bool { self.0 & (1 << 7) != 0 }
    pub fn floating_ambiguity_heading(&self) -> bool { self.0 & (1 << 8) != 0 }
    pub fn antenna_1_disconnected(&self) -> bool { self.0 & (1 << 10) != 0 }
    pub fn antenna_2_disconnected(&self) -> bool { self.0 & (1 << 11) != 0 }
    pub fn antenna_1_short(&self) -> bool { self.0 & (1 << 12) != 0 }
    pub fn antenna_2_short(&self) -> bool { self.0 & (1 << 13) != 0 }
    pub fn gnss1_failure(&self) -> bool { self.0 & (1 << 14) != 0 }
    pub fn gnss2_failure(&self) -> bool { self.0 & (1 << 15) != 0 }
}

impl From<u16> for RawGnssStatus {
    fn from(v: u16) -> Self { Self(v) }
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

/// Raw GNSS packet (Packet ID 29, Length 74) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RawGnss {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
    /// Latitude in radians
    pub latitude: f64,
    /// Longitude in radians
    pub longitude: f64,
    /// Height in meters
    pub height: f64,
    /// Velocity north in m/s
    pub velocity_north: f32,
    /// Velocity east in m/s
    pub velocity_east: f32,
    /// Velocity down in m/s
    pub velocity_down: f32,
    /// Latitude standard deviation in meters
    pub latitude_std_dev: f32,
    /// Longitude standard deviation in meters
    pub longitude_std_dev: f32,
    /// Height standard deviation in meters
    pub height_std_dev: f32,
    /// Tilt in radians
    pub tilt: f32,
    /// Heading in radians
    pub heading: f32,
    /// Tilt standard deviation in radians
    pub tilt_std_dev: f32,
    /// Heading standard deviation in radians
    pub heading_std_dev: f32,
    pub status: RawGnssStatus,
}

/// Geodetic position packet (Packet ID 32, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct GeodeticPosition {
    /// Latitude in radians
    pub latitude: f64,
    /// Longitude in radians
    pub longitude: f64,
    /// Height in meters
    pub height: f64,
}

/// ECEF position packet (Packet ID 33, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct EcefPosition {
    /// ECEF X in meters
    pub x: f64,
    /// ECEF Y in meters
    pub y: f64,
    /// ECEF Z in meters
    pub z: f64,
}

/// UTM position packet (Packet ID 34, Length 26) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct UtmPosition {
    /// Northing in meters
    pub northing: f64,
    /// Easting in meters
    pub easting: f64,
    /// Height in meters
    pub height: f64,
    /// UTM zone number
    pub zone_number: u8,
    /// UTM zone character
    pub zone_char: i8,
}

/// NED velocity packet (Packet ID 35, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct NedVelocity {
    /// Velocity north in m/s
    pub velocity_north: f32,
    /// Velocity east in m/s
    pub velocity_east: f32,
    /// Velocity down in m/s
    pub velocity_down: f32,
}

/// Body velocity packet (Packet ID 36, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct BodyVelocity {
    /// Velocity X in m/s
    pub velocity_x: f32,
    /// Velocity Y in m/s
    pub velocity_y: f32,
    /// Velocity Z in m/s
    pub velocity_z: f32,
}

/// Acceleration packet (Packet ID 37, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Acceleration {
    /// Acceleration X in m/s²
    pub acceleration_x: f32,
    /// Acceleration Y in m/s²
    pub acceleration_y: f32,
    /// Acceleration Z in m/s²
    pub acceleration_z: f32,
}

/// Body acceleration packet (Packet ID 38, Length 16) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct BodyAcceleration {
    /// Body acceleration X in m/s²
    pub body_acceleration_x: f32,
    /// Body acceleration Y in m/s²
    pub body_acceleration_y: f32,
    /// Body acceleration Z in m/s²
    pub body_acceleration_z: f32,
    /// G force in g
    pub g_force: f32,
}

/// Euler orientation packet (Packet ID 39, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct EulerOrientation {
    /// Roll in radians
    pub roll: f32,
    /// Pitch in radians
    pub pitch: f32,
    /// Heading in radians
    pub heading: f32,
}

/// Quaternion orientation packet (Packet ID 40, Length 16) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct QuaternionOrientation {
    /// Scalar component
    pub q0: f32,
    /// X vector component
    pub q1: f32,
    /// Y vector component
    pub q2: f32,
    /// Z vector component
    pub q3: f32,
}

/// DCM orientation packet (Packet ID 41, Length 36) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DcmOrientation {
    pub dcm: [[f32; 3]; 3],
}

/// Angular velocity packet (Packet ID 42, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AngularVelocity {
    /// Angular velocity X in rad/s
    pub angular_velocity_x: f32,
    /// Angular velocity Y in rad/s
    pub angular_velocity_y: f32,
    /// Angular velocity Z in rad/s
    pub angular_velocity_z: f32,
}

/// Angular acceleration packet (Packet ID 43, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AngularAcceleration {
    /// Angular acceleration X in rad/s²
    pub angular_acceleration_x: f32,
    /// Angular acceleration Y in rad/s²
    pub angular_acceleration_y: f32,
    /// Angular acceleration Z in rad/s²
    pub angular_acceleration_z: f32,
}

/// External position and velocity packet (Packet ID 44, Length 60) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalPositionVelocity {
    /// Latitude in radians
    pub latitude: f64,
    /// Longitude in radians
    pub longitude: f64,
    /// Height in meters
    pub height: f64,
    /// Velocity north in m/s
    pub velocity_north: f32,
    /// Velocity east in m/s
    pub velocity_east: f32,
    /// Velocity down in m/s
    pub velocity_down: f32,
    /// Latitude standard deviation in meters
    pub latitude_std_dev: f32,
    /// Longitude standard deviation in meters
    pub longitude_std_dev: f32,
    /// Height standard deviation in meters
    pub height_std_dev: f32,
    /// Velocity north standard deviation in m/s
    pub velocity_north_std_dev: f32,
    /// Velocity east standard deviation in m/s
    pub velocity_east_std_dev: f32,
    /// Velocity down standard deviation in m/s
    pub velocity_down_std_dev: f32,
}

/// External position packet (Packet ID 45, Length 36) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalPosition {
    /// Latitude in radians
    pub latitude: f64,
    /// Longitude in radians
    pub longitude: f64,
    /// Height in meters
    pub height: f64,
    /// Latitude standard deviation in meters
    pub latitude_std_dev: f32,
    /// Longitude standard deviation in meters
    pub longitude_std_dev: f32,
    /// Height standard deviation in meters
    pub height_std_dev: f32,
}

/// External velocity packet (Packet ID 46, Length 24) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalVelocity {
    /// Velocity north in m/s
    pub velocity_north: f32,
    /// Velocity east in m/s
    pub velocity_east: f32,
    /// Velocity down in m/s
    pub velocity_down: f32,
    /// Velocity north standard deviation in m/s
    pub velocity_north_std_dev: f32,
    /// Velocity east standard deviation in m/s
    pub velocity_east_std_dev: f32,
    /// Velocity down standard deviation in m/s
    pub velocity_down_std_dev: f32,
}

/// External body velocity packet (Packet ID 47, Length 16) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalBodyVelocity {
    /// Velocity X in m/s
    pub velocity_x: f32,
    /// Velocity Y in m/s
    pub velocity_y: f32,
    /// Velocity Z in m/s
    pub velocity_z: f32,
    /// Standard deviation in m/s
    pub standard_deviation: f32,
}

/// External heading packet (Packet ID 48, Length 8) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalHeading {
    /// Heading in radians
    pub heading: f32,
    /// Standard deviation in radians
    pub standard_deviation: f32,
}

/// Running time packet (Packet ID 49, Length 8) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RunningTime {
    /// Running time in seconds since power on
    pub seconds: u32,
    /// Microseconds component
    pub microseconds: u32,
}

/// External time packet (Packet ID 52, Length 8) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

/// Geoid height packet (Packet ID 54, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct GeoidHeight {
    /// Geoid height in meters (offset between WGS84 ellipsoid and EGM96 geoid)
    pub geoid_height: f32,
}

/// RTCM corrections packet (Packet ID 55, Variable length) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RtcmCorrections {
    /// Raw RTCM v3 correction data
    #[br(parse_with = binrw::helpers::until_eof)]
    pub data: Vec<u8>,
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

/// DVL status flags bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DvlStatus(u32);

impl DvlStatus {
    pub fn raw(&self) -> u32 { self.0 }
    pub fn bottom_velocity_valid(&self) -> bool { self.0 & (1 << 0) != 0 }
    pub fn water_velocity_valid(&self) -> bool { self.0 & (1 << 1) != 0 }
    pub fn temperature_valid(&self) -> bool { self.0 & (1 << 2) != 0 }
    pub fn depth_valid(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn altitude_valid(&self) -> bool { self.0 & (1 << 4) != 0 }
}

impl From<u32> for DvlStatus {
    fn from(v: u32) -> Self { Self(v) }
}

/// Raw DVL data packet (Packet ID 70, Length 60) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RawDvlData {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
    pub status: DvlStatus,
    /// Bottom velocity X in m/s
    pub bottom_velocity_x: f32,
    /// Bottom velocity Y in m/s
    pub bottom_velocity_y: f32,
    /// Bottom velocity Z in m/s
    pub bottom_velocity_z: f32,
    /// Bottom velocity standard deviation in m/s
    pub bottom_velocity_std_dev: f32,
    /// Water velocity X in m/s
    pub water_velocity_x: f32,
    /// Water velocity Y in m/s
    pub water_velocity_y: f32,
    /// Water velocity Z in m/s
    pub water_velocity_z: f32,
    /// Water velocity standard deviation in m/s
    pub water_velocity_std_dev: f32,
    /// Water velocity layer depth in meters
    pub water_velocity_layer_depth: f32,
    /// Depth in meters
    pub depth: f32,
    /// Altitude in meters
    pub altitude: f32,
    /// Temperature in degrees Celsius
    pub temperature: f32,
}

/// GNSS manufacturer identifier
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssManufacturer {
    #[default]
    Unknown = 0,
    Trimble = 1,
    UBlox = 2,
    AdvancedNavigation = 3,
}

impl From<u8> for GnssManufacturer {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Unknown,
            1 => Self::Trimble,
            2 => Self::UBlox,
            3 => Self::AdvancedNavigation,
            _ => Self::Unknown,
        }
    }
}

/// GNSS receiver model (decoded from manufacturer + model ID)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssReceiverModel {
    #[default]
    Unknown,
    /// Trimble MB-Two
    TrimbleMbTwo,
    /// Trimble BD992
    TrimbleBd992,
    /// u-blox NEO-F9P
    UBloxNeoF9P,
    /// Advanced Navigation Aries
    Aries,
    /// Advanced Navigation Aries GC2
    AriesGc2,
}

impl From<(u8, u8)> for GnssReceiverModel {
    fn from((manufacturer, model): (u8, u8)) -> Self {
        match (manufacturer, model) {
            (1, 5) => Self::TrimbleMbTwo,
            (1, 7) => Self::TrimbleBd992,
            (2, 5) => Self::UBloxNeoF9P,
            (3, 1) => Self::Aries,
            (3, 2) => Self::AriesGc2,
            _ => Self::Unknown,
        }
    }
}

/// GNSS receiver information packet (Packet ID 69, Length 68) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssReceiverInformation {
    #[br(map = |x: u8| GnssManufacturer::from(x))]
    #[bw(map = |x: &GnssManufacturer| *x as u8)]
    pub manufacturer: GnssManufacturer,
    /// Raw receiver model byte (use `receiver_model()` to decode)
    pub receiver_model_id: u8,
    /// Serial number as ASCII string (24 bytes)
    pub serial_number: [u8; 24],
    pub firmware_version: u32,
    pub hardware_version: u32,
    #[br(temp)]
    #[bw(calc = [0u8; 34])]
    _reserved: [u8; 34],
}

impl GnssReceiverInformation {
    /// Decode the receiver model from manufacturer + model ID
    pub fn receiver_model(&self) -> GnssReceiverModel {
        GnssReceiverModel::from((self.manufacturer as u8, self.receiver_model_id))
    }

    /// Get serial number as a string
    pub fn serial_number_str(&self) -> &str {
        let len = self.serial_number.iter().position(|&b| b == 0).unwrap_or(24);
        std::str::from_utf8(&self.serial_number[..len]).unwrap_or("")
    }
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
    fn test_system_status_accessors() {
        let status = SystemStatus::from(0b0100_0000_0010_0001u16); // bits 0, 5, 14
        assert!(status.system_failure());
        assert!(status.gnss_failure());
        assert!(status.gnss_antenna_disconnected());
        assert!(!status.accelerometer_sensor_failure());
        assert_eq!(status, SystemStatus::from(status.raw()));
    }

    #[test]
    fn test_filter_status_accessors() {
        // bits 0, 2, 9 + gnss_fix_type = 7 (bits 4-6)
        let status = FilterStatus::from(0b0000_0010_0111_0101u16);
        assert!(status.orientation_filter_initialised());
        assert!(status.heading_initialised());
        assert!(status.internal_gnss_enabled());
        assert_eq!(status.gnss_fix_type(), GnssFixType::RtkFixed);
        assert_eq!(status, FilterStatus::from(status.raw()));
    }

    #[test]
    fn test_gnss_pvt_status_accessors() {
        let status = GnssPvtStatus::from(0u16);
        assert_eq!(status.gnss_fix_status(), GnssFixType::NoFix);
        assert!(!status.velocity_valid());
        let status = GnssPvtStatus::from(0b0000_0110_0000_1010u16); // fix=2, spoofing=1, velocity_valid
        assert_eq!(status.gnss_fix_status(), GnssFixType::Fix3D);
        assert_eq!(status.spoofing_status(), SpoofingStatus::None);
        assert!(status.velocity_valid());
        assert_eq!(status, GnssPvtStatus::from(status.raw()));
    }

    #[test]
    fn test_gnss_orientation_status_accessors() {
        let status = GnssOrientationStatus::from(0b0000_0000_0010_0110u16); // fix=6, antenna_disconnected
        assert_eq!(status.gnss_fix_status(), GnssFixType::RtkFloat);
        assert!(!status.antenna_disconnected()); // bit 3 not set
        assert!(status.gnss_failure()); // bit 5
        assert_eq!(status, GnssOrientationStatus::from(status.raw()));
    }

    #[test]
    fn test_system_state_serialization() {
        use std::f64::consts::PI;

        let system_state = SystemState {
            system_status: SystemStatus::default(),
            filter_status: FilterStatus::from(0b0000_0000_0000_0111u16), // bits 0,1,2
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
