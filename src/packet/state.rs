use super::{
    receiver::{GnssManufacturer, GnssReceiverData},
    satellite::{EphemerisData, ExtendedSatelliteEntry, RawSatelliteEntry, SatelliteSystem},
};
use binrw::{binrw, BinRead, BinWrite};
use bitflags::{bitflags, parser::WriteHex};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! hex_debug {
    ($t:ty) => {
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}(0x", stringify!($t))?;
                self.bits().write_hex(&mut *f)?;
                write!(f, ")")
            }
        }
    };
}

// ===========================================================================
// Enums and Status Types
// ===========================================================================

/// GNSS fix type enumeration
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
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

/// Spoofing status for GNSS packets
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SpoofingStatus {
    #[default]
    Unknown = 0,
    None = 1,
    DetectedAndMitigated = 2,
    DetectedAndUnmitigated = 3,
}

/// Interference status for GNSS packets
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum InterferenceStatus {
    #[default]
    Unknown = 0,
    None = 1,
    DetectedAndMitigated = 2,
    DetectedAndUnmitigated = 3,
}

bitflags! {
    /// System status flags
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        const INTERNAL_DATA_LOGGING_ERROR = 1 << 12;
        const HIGH_VOLTAGE_ALARM = 1 << 13;
        const GNSS_ANTENNA_DISCONNECTED = 1 << 14;
        const DATA_OUTPUT_OVERFLOW_ALARM = 1 << 15;
    }
}

bitflags! {
    /// Filter status flags
    #[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FilterStatus: u16 {
        const ORIENTATION_FILTER_INITIALISED = 1 << 0;
        const NAVIGATION_FILTER_INITIALISED = 1 << 1;
        const HEADING_INITIALISED = 1 << 2;
        const UTC_TIME_INITIALISED = 1 << 3;
        /// Mask for the GNSS fix type in bits 4-6, decoded by [`Self::gnss_fix_type`]
        const GNSS_FIX_TYPE_MASK = 0b0111 << Self::GNSS_FIX_TYPE_OFFSET;
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

hex_debug!(FilterStatus);

impl FilterStatus {
    const GNSS_FIX_TYPE_OFFSET: u32 = 4;

    pub fn gnss_fix_type(&self) -> GnssFixType { GnssFixType::from((self.intersection(Self::GNSS_FIX_TYPE_MASK).bits() >> Self::GNSS_FIX_TYPE_OFFSET) as u8) }
}

bitflags! {
    /// GNSS PVT status flags
    #[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GnssPvtStatus: u16 {
        /// Mask for the GNSS fix type in bits 0-2, decoded by [`Self::gnss_fix_status`]
        const GNSS_FIX_TYPE_MASK = 0b0111 << Self::GNSS_FIX_TYPE_OFFSET;
        /// Mask for the spoofing status in bits 3-5, decoded by [`Self::spoofing_status`]
        const SPOOFING_STATUS_MASK = 0b0111 << Self::SPOOFING_STATUS_OFFSET;
        /// Mask for the interference status in bits 6-8, decoded by [`Self::interference_status`]
        const INTERFERENCE_STATUS_MASK = 0b0111 << Self::INTERFERENCE_STATUS_OFFSET;
        const VELOCITY_VALID = 1 << 9;
        const TIME_VALID = 1 << 10;
        const ANTENNA_DISCONNECTED = 1 << 11;
        const ANTENNA_SHORT = 1 << 12;
        const GNSS_FAILURE = 1 << 13;
    }
}

hex_debug!(GnssPvtStatus);

impl GnssPvtStatus {
    const GNSS_FIX_TYPE_OFFSET: u32 = 0;
    const SPOOFING_STATUS_OFFSET: u32 = 3;
    const INTERFERENCE_STATUS_OFFSET: u32 = 6;

    pub fn from_gnss_fix(fix: GnssFixType) -> Self { Self::from_bits_retain((fix as u16) << Self::GNSS_FIX_TYPE_OFFSET) }
    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.intersection(Self::GNSS_FIX_TYPE_MASK).bits() >> Self::GNSS_FIX_TYPE_OFFSET) as u8) }
    pub fn spoofing_status(&self) -> SpoofingStatus { SpoofingStatus::from((self.intersection(Self::SPOOFING_STATUS_MASK).bits() >> Self::SPOOFING_STATUS_OFFSET) as u8) }
    pub fn interference_status(&self) -> InterferenceStatus { InterferenceStatus::from((self.intersection(Self::INTERFERENCE_STATUS_MASK).bits() >> Self::INTERFERENCE_STATUS_OFFSET) as u8) }
}

bitflags! {
    /// GNSS Orientation status flags
    #[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GnssOrientationStatus: u16 {
        /// Mask for the GNSS fix type in bits 0-2, decoded by [`Self::gnss_fix_status`]
        const GNSS_FIX_TYPE_MASK = 0b0111 << Self::GNSS_FIX_TYPE_OFFSET;
        const ANTENNA_DISCONNECTED = 1 << 3;
        const ANTENNA_SHORT = 1 << 4;
        const GNSS_FAILURE = 1 << 5;
        /// Mask for the spoofing status in bits 6-8, decoded by [`Self::spoofing_status`]
        const SPOOFING_STATUS_MASK = 0b0111 << Self::SPOOFING_STATUS_OFFSET;
        /// Mask for the interference status in bits 9-11, decoded by [`Self::interference_status`]
        const INTERFERENCE_STATUS_MASK = 0b0111 << Self::INTERFERENCE_STATUS_OFFSET;
    }
}

hex_debug!(GnssOrientationStatus);

impl GnssOrientationStatus {
    const GNSS_FIX_TYPE_OFFSET: u32 = 0;
    const SPOOFING_STATUS_OFFSET: u32 = 6;
    const INTERFERENCE_STATUS_OFFSET: u32 = 9;

    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.intersection(Self::GNSS_FIX_TYPE_MASK).bits() >> Self::GNSS_FIX_TYPE_OFFSET) as u8) }
    pub fn spoofing_status(&self) -> SpoofingStatus { SpoofingStatus::from((self.intersection(Self::SPOOFING_STATUS_MASK).bits() >> Self::SPOOFING_STATUS_OFFSET) as u8) }
    pub fn interference_status(&self) -> InterferenceStatus { InterferenceStatus::from((self.intersection(Self::INTERFERENCE_STATUS_MASK).bits() >> Self::INTERFERENCE_STATUS_OFFSET) as u8) }
}

// ===========================================================================
// Packet Structs
// ===========================================================================

/// System state packet (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct SystemState {
    #[br(map = SystemStatus::from_bits_retain)]
    #[bw(map = |x: &SystemStatus| x.bits())]
    pub system_status: SystemStatus,
    #[br(map = FilterStatus::from_bits_retain)]
    #[bw(map = |x: &FilterStatus| x.bits())]
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

/// Formatted time packet (Packet ID 22, Length 14) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct FormattedTime {
    pub microseconds: u32,
    pub year: u16,
    pub year_day: u16,
    pub month: u8,
    pub month_day: u8,
    pub week_day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl FormattedTime {
    /// Convert the time into a unix timestamp (ignoring micros).
    /// If `year_day` or `month_day` don't match the values derived
    /// from the date, return `None`.
    pub fn unix_time_seconds(&self) -> Option<i64> {
        // using our own impl since this is the only time conversion: not
        // worth adding a new dependency, and std does not have anything suitable.
        let year = self.year as i64 - (self.month <= 2) as i64;
        let era = (if year >= 0 { year } else { year - 399 }) / 400;
        let yoe = year - era * 400;
        let dom = (153 * (self.month as i64 + if self.month > 2 { -3 } else { 9 }) + 2) / 5 + self.month_day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + dom;
        let day = era * 146097 + doe - 719468;

        // Error if day of week does not match
        if (day + 4).rem_euclid(7) != self.week_day as i64 {
            return None;
        }

        // Error of day of year does not match. compute doy as diff from day of Jan 1 of self.year.
        let check_year = self.year as i64 - 1;
        let check_era = (if check_year >= 0 { check_year } else { check_year - 399 }) / 400;
        let check_yoe = check_year - check_era * 400;
        let check_day = check_era * 146097 + check_yoe * 365 + check_yoe / 4 - check_yoe / 100 - 719162;
        if day - check_day != self.year_day as i64 {
            return None;
        }

        Some(day * 86_400 + self.hour as i64 * 3_600 + self.minute as i64 * 60 + self.second as i64)
    }
}

/// Status packet (Packet ID 23, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Status {
    #[br(map = SystemStatus::from_bits_retain)]
    #[bw(map = |x: &SystemStatus| x.bits())]
    pub system_status: SystemStatus,
    #[br(map = FilterStatus::from_bits_retain)]
    #[bw(map = |x: &FilterStatus| x.bits())]
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

bitflags! {
    /// Raw GNSS status flags
    #[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RawGnssStatus: u16 {
        /// Mask for the GNSS fix type in bits 0-2, decoded by [`Self::gnss_fix_status`]
        const GNSS_FIX_TYPE_MASK = 0b0111 << Self::GNSS_FIX_TYPE_OFFSET;
        const DOPPLER_VELOCITY_VALID = 1 << 3;
        const TIME_VALID = 1 << 4;
        const EXTERNAL_GNSS = 1 << 5;
        const TILT_VALID = 1 << 6;
        const HEADING_VALID = 1 << 7;
        const FLOATING_AMBIGUITY_HEADING = 1 << 8;
        const ANTENNA_1_DISCONNECTED = 1 << 10;
        const ANTENNA_2_DISCONNECTED = 1 << 11;
        const ANTENNA_1_SHORT = 1 << 12;
        const ANTENNA_2_SHORT = 1 << 13;
        const GNSS1_FAILURE = 1 << 14;
        const GNSS2_FAILURE = 1 << 15;
    }
}

hex_debug!(RawGnssStatus);

impl RawGnssStatus {
    const GNSS_FIX_TYPE_OFFSET: u32 = 0;

    pub fn gnss_fix_status(&self) -> GnssFixType { GnssFixType::from((self.intersection(Self::GNSS_FIX_TYPE_MASK).bits() >> Self::GNSS_FIX_TYPE_OFFSET) as u8) }
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

/// Raw GNSS packet (Packet ID 29, Length 74) - Read/Write
///
/// This packet represents the raw data as it is received from the GNSS
/// receiver. The position is not corrected for antenna position offset and
/// the velocity is not compensated for the antenna lever arm offset.
#[deprecated(note = "superseded by GnssPositionVelocityTime and GnssOrientation")]
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
    #[br(map = RawGnssStatus::from_bits_retain)]
    #[bw(map = |x: &RawGnssStatus| x.bits())]
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

/// External body velocity packet (Packet ID 47, Length 16 or 24) - Write only
///
/// The 16-byte variant represents isotropic velocity error: all three standard deviations are equal.
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalBodyVelocity {
    /// Velocity X in m/s
    pub velocity_x: f32,
    /// Velocity Y in m/s
    pub velocity_y: f32,
    /// Velocity Z in m/s
    pub velocity_z: f32,
    /// Velocity X standard deviation in m/s
    pub velocity_x_std_dev: f32,
    /// Velocity Y standard deviation in m/s
    #[br(parse_with = |reader, endian, (fallback,): (f32,)| {
        f32::read_options(reader, endian, ()).or(Ok(fallback))
    }, args(velocity_x_std_dev))]
    pub velocity_y_std_dev: f32,
    /// Velocity Z standard deviation in m/s
    #[br(parse_with = |reader, endian, (fallback,): (f32,)| {
        f32::read_options(reader, endian, ()).or(Ok(fallback))
    }, args(velocity_x_std_dev))]
    pub velocity_z_std_dev: f32,
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

/// Odometer state packet (Packet ID 51, Length 20) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerState {
    pub pulse_count: i32,
    /// Distance in meters
    pub distance: f32,
    /// Speed in m/s
    pub speed: f32,
    /// Slip in meters
    pub slip: f32,
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub active: bool,
    #[br(temp)]
    #[bw(calc = [0u8; 3])]
    _reserved: [u8; 3],
}

/// External time packet (Packet ID 52, Length 8) - Write only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

/// External depth packet (Packet ID 53, Length 8) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalDepth {
    /// Depth below mean sea level in meters
    pub depth: f32,
    /// Depth standard deviation in meters
    pub depth_std_dev: f32,
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

/// Wind packet (Packet ID 57, Length 12) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Wind {
    /// Wind velocity north in m/s
    pub velocity_north: f32,
    /// Wind velocity east in m/s
    pub velocity_east: f32,
    /// Wind velocity standard deviation in m/s
    pub velocity_std_dev: f32,
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

/// Raw satellite data packet (Packet ID 60, Variable length) - Read only
#[deprecated(note = "broken due to lossy signal codes; unusable without detailed receiver information, which may not always be available")]
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSatelliteData {
    /// Unix timestamp (seconds)
    pub unix_time: u32,
    /// Nanoseconds part of timestamp
    pub nanoseconds: u32,
    /// Receiver clock offset (nanoseconds)
    pub receiver_clock_offset: i32,
    /// Receiver number
    pub receiver_number: u8,
    /// Packet number (range 1 to Total)
    pub packet_number: u8,
    /// Total packets
    pub total_packets: u8,
    #[br(temp)]
    #[bw(calc = satellites.len() as u8)]
    num_satellites: u8,
    /// Per-satellite measurements
    #[br(count = num_satellites)]
    pub satellites: Vec<RawSatelliteEntry>,
}

/// Raw satellite ephemeris packet (Packet ID 61, Length 132 GPS / 94 GLONASS) - Read only
#[deprecated(note = "broken due to lossy signal codes; unusable without detailed receiver information, which may not always be available")]
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSatelliteEphemeris {
    /// Unix timestamp (seconds)
    pub unix_time: u32,
    /// Satellite system
    #[br(map = |x: u8| SatelliteSystem::from(x))]
    #[bw(map = |x: &SatelliteSystem| *x as u8)]
    pub satellite_system: SatelliteSystem,
    /// Satellite number (PRN)
    pub prn: u8,
    /// System-specific ephemeris data
    #[br(args(satellite_system))]
    pub data: EphemerisData,
}

bitflags! {
    /// DVL status flags
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DvlStatus: u32 {
        const BOTTOM_VELOCITY_VALID = 1 << 0;
        const WATER_VELOCITY_VALID = 1 << 1;
        const TEMPERATURE_VALID = 1 << 2;
        const DEPTH_VALID = 1 << 3;
        const ALTITUDE_VALID = 1 << 4;
    }
}

/// External odometer packet (Packet ID 67, Length 13) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalOdometer {
    /// Estimated measurement delay in seconds
    pub estimated_delay: f32,
    /// Speed in m/s
    pub speed: f32,
    #[br(temp)]
    #[bw(calc = 0f32)]
    _reserved: f32,
    /// Whether the odometer supports reversing detection
    #[br(map = |x: u8| x != 0)]
    #[bw(map = |x: &bool| *x as u8)]
    pub reversing_detection_supported: bool,
}

bitflags! {
    /// Air data flags for ExternalAirData
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AirDataFlags: u8 {
        const BAROMETRIC_ALTITUDE_VALID = 1 << 0;
        const AIRSPEED_VALID = 1 << 1;
        const BAROMETRIC_ALTITUDE_OVER_RANGE = 1 << 2;
        const AIRSPEED_OVER_RANGE = 1 << 3;
        const BAROMETRIC_ALTITUDE_SENSOR_FAILURE = 1 << 4;
        const AIRSPEED_SENSOR_FAILURE = 1 << 5;
    }
}

/// External air data packet (Packet ID 68, Length 25) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalAirData {
    /// Barometric altitude measurement delay in seconds
    pub barometric_altitude_delay: f32,
    /// Airspeed measurement delay in seconds
    pub airspeed_delay: f32,
    /// Barometric altitude in meters
    pub barometric_altitude: f32,
    /// True airspeed in m/s
    pub airspeed: f32,
    /// Barometric altitude standard deviation in meters
    pub barometric_altitude_std_dev: f32,
    /// Airspeed standard deviation in m/s
    pub airspeed_std_dev: f32,
    /// Validity and sensor status flags
    #[br(map = AirDataFlags::from_bits_retain)]
    #[bw(map = |x: &AirDataFlags| x.bits())]
    pub flags: AirDataFlags,
}

/// GNSS receiver information packet (Packet ID 69, Length 48 or 68) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssReceiverInformation {
    /// Manufacturer and receiver model (2-byte header)
    #[br(parse_with = GnssManufacturer::parse)]
    #[bw(write_with = GnssManufacturer::write_to)]
    pub manufacturer: GnssManufacturer,
    /// Receiver-specific payload, layout determined by manufacturer and model
    #[br(args(manufacturer.receiver_type()))]
    pub data: GnssReceiverData,
}

/// Raw DVL data packet (Packet ID 70, Length 60) - Read/Write
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RawDvlData {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
    #[br(map = DvlStatus::from_bits_retain)]
    #[bw(map = |x: &DvlStatus| x.bits())]
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

bitflags! {
    /// North seeking initialisation status flags
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct NorthSeekingFlags: u16 {
        const INITIALISATION_COMPLETE = 1 << 0;
        const CANNOT_START_POSITION_UNKNOWN = 1 << 1;
        const SOLUTION_OUT_OF_RANGE = 1 << 2;
        const SOLUTION_NON_ORTHOGONAL = 1 << 3;
        const RESTARTED_EXCESSIVE_MOVEMENT = 1 << 4;
        const RESTARTED_LATITUDE_CHANGE = 1 << 5;
        const RESTARTED_LEVER_ARM_CHANGE = 1 << 6;
        const LATITUDE_CHECK_FAILED = 1 << 7;
    }
}

/// North seeking initialisation status packet (Packet ID 71, Length 28) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NorthSeekingInitialisationStatus {
    /// Initialisation status flags
    #[br(map = NorthSeekingFlags::from_bits_retain)]
    #[bw(map = |x: &NorthSeekingFlags| x.bits())]
    pub flags: NorthSeekingFlags,
    /// Firmware version
    pub version: u16,
    /// Initialisation progress as a percentage
    pub progress: u8,
    /// Number of alignment attempts
    pub alignment_attempts: u8,
    #[br(temp)]
    #[bw(calc = [0u8; 2])]
    _reserved1: [u8; 2],
    /// Coarse alignment heading in radians
    pub coarse_alignment_heading: f32,
    /// Predicted heading accuracy in radians
    pub predicted_accuracy: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 12])]
    _reserved2: [u8; 12],
}

/// Gimbal state packet (Packet ID 72, Length 8) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalState {
    /// Current gimbal angle in radians
    pub angle: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 4])]
    _reserved: [u8; 4],
}

/// Automotive packet (Packet ID 73, Length 24) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Automotive {
    /// Virtual odometer distance in meters
    pub virtual_odometer_distance: f32,
    /// Slip angle in radians
    pub slip_angle: f32,
    /// Velocity X in m/s
    pub velocity_x: f32,
    /// Velocity Y in m/s
    pub velocity_y: f32,
    /// Distance standard deviation in meters
    pub distance_std_dev: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 4])]
    _reserved: [u8; 4],
}

bitflags! {
    /// External magnetometers flags
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExternalMagnetometersFlags: u8 {
        const FAILURE = 1 << 0;
        const OVER_RANGE = 1 << 1;
    }
}

/// External magnetometers packet (Packet ID 75, Length 17) - Read/Write
///
/// External magnetometers need to be calibrated before feeding into the
/// device; the 2D, 3D, and automatic magnetic calibration of the device
/// cannot be used to calibrate the external magnetometer values. For Boreas
/// units, a magnetic heading provided in this packet is accepted without
/// error, but it is not used, because the Boreas fiber optic gyroscopes are
/// more accurate than magnetometers.
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExternalMagnetometers {
    /// Delay in seconds
    pub delay: f32,
    /// Magnetometer X in milligauss
    pub magnetometer_x: f32,
    /// Magnetometer Y in milligauss
    pub magnetometer_y: f32,
    /// Magnetometer Z in milligauss
    pub magnetometer_z: f32,
    /// External magnetometer flags
    #[br(map = ExternalMagnetometersFlags::from_bits_retain)]
    #[bw(map = |x: &ExternalMagnetometersFlags| x.bits())]
    pub flags: ExternalMagnetometersFlags,
}

/// Zero angular velocity packet (Packet ID 83, Length 8) - Write only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZeroAngularVelocity {
    /// Duration the unit has been stationary about the heading axis in seconds
    pub duration: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 4])]
    _reserved: [u8; 4],
}

/// Extended satellites packet (Packet ID 84, Variable length) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExtendedSatellites {
    /// Total number of extended satellites packets
    pub total_packets: u8,
    /// Packet number (range 1 to Total)
    pub packet_number: u8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = super::write_vec)]
    pub satellites: Vec<ExtendedSatelliteEntry>,
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

/// System temperature packet (Packet ID 86, Length 64) - Read only
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemTemperature {
    /// System temperature in degrees Celsius
    pub temperature: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 60])]
    _reserved: [u8; 60],
}

/// Vessel motion packet (Packet ID 89, Length 48) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct VesselMotion {
    /// Surge at reference point 1 in meters
    pub surge_point_1: f32,
    /// Surge at reference point 2 in meters
    pub surge_point_2: f32,
    /// Surge at reference point 3 in meters
    pub surge_point_3: f32,
    /// Surge at reference point 4 in meters
    pub surge_point_4: f32,
    /// Sway at reference point 1 in meters
    pub sway_point_1: f32,
    /// Sway at reference point 2 in meters
    pub sway_point_2: f32,
    /// Sway at reference point 3 in meters
    pub sway_point_3: f32,
    /// Sway at reference point 4 in meters
    pub sway_point_4: f32,
    /// Heave at reference point 1 in meters
    pub heave_point_1: f32,
    /// Heave at reference point 2 in meters
    pub heave_point_2: f32,
    /// Heave at reference point 3 in meters
    pub heave_point_3: f32,
    /// Heave at reference point 4 in meters
    pub heave_point_4: f32,
}

/// Automatic magnetic calibration status method
#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum MagneticCalibrationMethod {
    Disabled = 0,
    GnssAided = 1,
    Online = 2,
}

bitflags! {
    /// Automatic magnetic calibration status flags
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MagneticCalibrationFlags: u8 {
        const READY = 1 << 0;
        const COARSE_COMPLETE = 1 << 1;
        const EXISTING_CALIBRATION = 1 << 2;
        const AWAITING_FILTERS = 1 << 3;
        const MANUAL_CALIBRATION_IN_PROGRESS = 1 << 4;
        const INVALID_VEHICLE_TYPE = 1 << 5;
        const MAGNETIC_HEADING_DISABLED = 1 << 6;
    }
}

/// Automatic magnetic calibration status packet (Packet ID 90, Length 78) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AutomaticMagneticCalibrationStatus {
    /// Method
    pub method: MagneticCalibrationMethod,
    /// Flags
    #[br(map = MagneticCalibrationFlags::from_bits_retain)]
    #[bw(map = |x: &MagneticCalibrationFlags| x.bits())]
    pub flags: MagneticCalibrationFlags,
    pub convergence: f32,
    pub scale_factor_x: f32,
    pub scale_factor_y: f32,
    pub scale_factor_z: f32,
    pub soft_iron_x: f32,
    pub soft_iron_y: f32,
    pub soft_iron_z: f32,
    pub hard_iron_x: f32,
    pub hard_iron_y: f32,
    pub hard_iron_z: f32,
    pub scale_factor_std_dev_x: f32,
    pub scale_factor_std_dev_y: f32,
    pub scale_factor_std_dev_z: f32,
    pub soft_iron_std_dev_x: f32,
    pub soft_iron_std_dev_y: f32,
    pub soft_iron_std_dev_z: f32,
    pub hard_iron_std_dev_x: f32,
    pub hard_iron_std_dev_y: f32,
    pub hard_iron_std_dev_z: f32,
}

/// External SVS packet (Packet ID 91, Length 28) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalSvs {
    /// Pressure in dBar
    pub pressure: f32,
    /// Temperature in degrees Celsius
    pub temperature: f32,
    /// Sound velocity in m/s
    pub sound_velocity: f32,
    /// Salinity in ppt
    pub salinity: f32,
    /// Density in kg/m³
    pub density: f32,
    #[br(temp)]
    #[bw(calc = [0u8; 8])]
    _reserved: [u8; 8],
}

/// GNSS Position Velocity Time packet (Packet ID 92, Length 76) - Read/Write
///
/// This packet provides the raw Position, Velocity, and Time (PVT) data as it
/// is received from the GNSS receiver. The position and velocity describe the
/// location of the GNSS primary antenna, not the vehicle reference point.
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssPositionVelocityTime {
    pub gnss_id: u8,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
    #[br(map = GnssPvtStatus::from_bits_retain)]
    #[bw(map = |x: &GnssPvtStatus| x.bits())]
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

/// GNSS Orientation packet (Packet ID 93, Length 36) - Read/Write
///
/// This packet represents the raw orientation data as it is received from the
/// GNSS receiver, and applies only to dual antenna installations. The
/// orientation is that of the line between the two antennas, not of the
/// vehicle body, and is described in the NED frame.
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssOrientation {
    pub gnss_id: u8,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
    #[br(map = GnssOrientationStatus::from_bits_retain)]
    #[bw(map = |x: &GnssOrientationStatus| x.bits())]
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

/// Origin of an aiding source, decoded from bits 10-15 of an
/// [`AidingSourceStatusField`]
#[derive(Debug, Clone, Copy, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum AidingSourceOrigin {
    Internal = 0,
    PrimaryPort = 1,
    AuxPort = 2,
    Gpio = 3,
    DataStream1 = 4,
    DataStream2 = 5,
    DataStream3 = 6,
    DataStream4 = 7,
    /// Origin value not defined by the protocol
    #[num_enum(default)]
    Unknown = 63,
}

/// Status of a single aiding source within [`AidingSourceStatus`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct AidingSourceStatusField(pub u16);

impl AidingSourceStatusField {
    const ORIGIN_OFFSET: u32 = 10;

    /// Communicating with device
    pub fn online(&self) -> bool { self.0 & (1 << 0) != 0 }

    /// Providing valid filter information
    pub fn valid(&self) -> bool { self.0 & (1 << 1) != 0 }

    /// Reporting a fault
    pub fn fault(&self) -> bool { self.0 & (1 << 2) != 0 }

    /// Aiding source origin
    pub fn origin(&self) -> AidingSourceOrigin { AidingSourceOrigin::from((self.0 >> Self::ORIGIN_OFFSET) as u8) }
}

/// Aiding source status packet (Packet ID 95, Length 64) - Read/Write
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AidingSourceStatus {
    pub internal_gnss_pvt: AidingSourceStatusField,
    pub internal_gnss_orientation: AidingSourceStatusField,
    pub internal_magnetometers: AidingSourceStatusField,
    pub internal_pressure: AidingSourceStatusField,
    pub external_gnss_pvt: AidingSourceStatusField,
    pub external_gnss_orientation: AidingSourceStatusField,
    pub external_position: AidingSourceStatusField,
    pub external_odometer: AidingSourceStatusField,
    pub external_heading: AidingSourceStatusField,
    pub external_pressure: AidingSourceStatusField,
    pub external_velocity: AidingSourceStatusField,
    pub external_position_velocity: AidingSourceStatusField,
    pub external_body_velocity: AidingSourceStatusField,
    pub external_air_data: AidingSourceStatusField,
    pub external_magnetometers: AidingSourceStatusField,
    pub external_lvs: AidingSourceStatusField,
    pub internal_depth_sensor: AidingSourceStatusField,
    pub external_subsonus: AidingSourceStatusField,
    pub external_dvl_data: AidingSourceStatusField,
    pub external_depth: AidingSourceStatusField,
    pub external_usbl: AidingSourceStatusField,
    #[br(temp)]
    #[bw(calc = [0u8; 22])]
    _reserved: [u8; 22],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_accessors() {
        let status = SystemStatus::from_bits_retain(0b0100_0000_0010_0001u16); // bits 0, 5, 14
        assert!(status.contains(SystemStatus::SYSTEM_FAILURE));
        assert!(status.contains(SystemStatus::GNSS_FAILURE));
        assert!(status.contains(SystemStatus::GNSS_ANTENNA_DISCONNECTED));
        assert!(!status.contains(SystemStatus::ACCELEROMETER_SENSOR_FAILURE));
        assert_eq!(status, SystemStatus::from_bits_retain(status.bits()));
    }

    #[test]
    fn test_filter_status_accessors() {
        // bits 0, 2, 9 + gnss_fix_type = 7 (bits 4-6)
        let status = FilterStatus::from_bits_retain(0b0000_0010_0111_0101u16);
        assert!(status.contains(FilterStatus::ORIENTATION_FILTER_INITIALISED));
        assert!(status.contains(FilterStatus::HEADING_INITIALISED));
        assert!(status.contains(FilterStatus::INTERNAL_GNSS_ENABLED));
        assert_eq!(status.gnss_fix_type(), GnssFixType::RtkFixed);
        assert_eq!(status, FilterStatus::from_bits_retain(status.bits()));
    }

    #[test]
    fn test_gnss_pvt_status_accessors() {
        let status = GnssPvtStatus::from_bits_retain(0u16);
        assert_eq!(status.gnss_fix_status(), GnssFixType::NoFix);
        assert!(!status.contains(GnssPvtStatus::VELOCITY_VALID));
        let status = GnssPvtStatus::from_bits_retain(0b0000_0110_0000_1010u16); // fix=2, spoofing=1, velocity_valid
        assert_eq!(status.gnss_fix_status(), GnssFixType::Fix3D);
        assert_eq!(status.spoofing_status(), SpoofingStatus::None);
        assert!(status.contains(GnssPvtStatus::VELOCITY_VALID));
        assert_eq!(status, GnssPvtStatus::from_bits_retain(status.bits()));
    }

    #[test]
    fn test_gnss_orientation_status_accessors() {
        let status = GnssOrientationStatus::from_bits_retain(0b0000_0000_0010_0110u16); // fix=6, gnss_failure
        assert_eq!(status.gnss_fix_status(), GnssFixType::RtkFloat);
        assert!(!status.contains(GnssOrientationStatus::ANTENNA_DISCONNECTED)); // bit 3 not set
        assert!(status.contains(GnssOrientationStatus::GNSS_FAILURE)); // bit 5
        assert_eq!(status, GnssOrientationStatus::from_bits_retain(status.bits()));
    }

    #[test]
    fn test_gnss_orientation_status_serde_round_trip() {
        // ANTENNA_SHORT | GNSS_FAILURE, GnssFixType::PppFix, SpoofingStatus::DetectedAndMitigated
        let status = GnssOrientationStatus::from_bits_retain(0x00F5u16);
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"ANTENNA_SHORT | GNSS_FAILURE | 0xc5\"");
        let deserialized: GnssOrientationStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, status);

        // ANTENNA_SHORT | GNSS_FAILURE, GnssFixType::RtkFixed, SpoofingStatus::DetectedAndMitigated
        let status = GnssOrientationStatus::from_bits_retain(0x00F7u16);
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"GNSS_FIX_TYPE_MASK | ANTENNA_SHORT | GNSS_FAILURE | 0xc0\"");
        let deserialized: GnssOrientationStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, status);
    }

    #[test]
    fn test_gnss_orientation_status_debug() {
        let status = GnssOrientationStatus::from_bits_retain(0x00F5u16); // PppFix
        let debug = format!("{status:?}");
        assert_eq!(debug, "GnssOrientationStatus(0xf5)");
    }

    #[test]
    fn test_system_state_serialization() {
        use std::f64::consts::PI;

        let system_state = SystemState {
            system_status: SystemStatus::default(),
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
