//! Clean Rust API types for state packets

use serde::{Serialize, Deserialize};
use crate::packet::{state, flags};

/// System status - individual boolean fields instead of bitflags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl From<flags::SystemStatusFlags> for SystemStatus {
    fn from(flags: flags::SystemStatusFlags) -> Self {
        Self {
            system_failure: flags.contains(flags::SystemStatusFlags::SYSTEM_FAILURE),
            accelerometer_sensor_failure: flags.contains(flags::SystemStatusFlags::ACCELEROMETER_SENSOR_FAILURE),
            gyroscope_sensor_failure: flags.contains(flags::SystemStatusFlags::GYROSCOPE_SENSOR_FAILURE),
            magnetometer_sensor_failure: flags.contains(flags::SystemStatusFlags::MAGNETOMETER_SENSOR_FAILURE),
            pressure_sensor_failure: flags.contains(flags::SystemStatusFlags::PRESSURE_SENSOR_FAILURE),
            gnss_failure: flags.contains(flags::SystemStatusFlags::GNSS_FAILURE),
            accelerometer_over_range: flags.contains(flags::SystemStatusFlags::ACCELEROMETER_OVER_RANGE),
            gyroscope_over_range: flags.contains(flags::SystemStatusFlags::GYROSCOPE_OVER_RANGE),
            magnetometer_over_range: flags.contains(flags::SystemStatusFlags::MAGNETOMETER_OVER_RANGE),
            pressure_over_range: flags.contains(flags::SystemStatusFlags::PRESSURE_OVER_RANGE),
            minimum_temperature_alarm: flags.contains(flags::SystemStatusFlags::MINIMUM_TEMPERATURE_ALARM),
            maximum_temperature_alarm: flags.contains(flags::SystemStatusFlags::MAXIMUM_TEMPERATURE_ALARM),
            low_voltage_alarm: flags.contains(flags::SystemStatusFlags::LOW_VOLTAGE_ALARM),
            high_voltage_alarm: flags.contains(flags::SystemStatusFlags::HIGH_VOLTAGE_ALARM),
            gnss_antenna_disconnected: flags.contains(flags::SystemStatusFlags::GNSS_ANTENNA_DISCONNECTED),
            serial_port_overflow_alarm: flags.contains(flags::SystemStatusFlags::SERIAL_PORT_OVERFLOW_ALARM),
        }
    }
}

impl From<SystemStatus> for flags::SystemStatusFlags {
    fn from(status: SystemStatus) -> Self {
        let mut flags = flags::SystemStatusFlags::empty();
        if status.system_failure { flags |= flags::SystemStatusFlags::SYSTEM_FAILURE; }
        if status.accelerometer_sensor_failure { flags |= flags::SystemStatusFlags::ACCELEROMETER_SENSOR_FAILURE; }
        if status.gyroscope_sensor_failure { flags |= flags::SystemStatusFlags::GYROSCOPE_SENSOR_FAILURE; }
        if status.magnetometer_sensor_failure { flags |= flags::SystemStatusFlags::MAGNETOMETER_SENSOR_FAILURE; }
        if status.pressure_sensor_failure { flags |= flags::SystemStatusFlags::PRESSURE_SENSOR_FAILURE; }
        if status.gnss_failure { flags |= flags::SystemStatusFlags::GNSS_FAILURE; }
        if status.accelerometer_over_range { flags |= flags::SystemStatusFlags::ACCELEROMETER_OVER_RANGE; }
        if status.gyroscope_over_range { flags |= flags::SystemStatusFlags::GYROSCOPE_OVER_RANGE; }
        if status.magnetometer_over_range { flags |= flags::SystemStatusFlags::MAGNETOMETER_OVER_RANGE; }
        if status.pressure_over_range { flags |= flags::SystemStatusFlags::PRESSURE_OVER_RANGE; }
        if status.minimum_temperature_alarm { flags |= flags::SystemStatusFlags::MINIMUM_TEMPERATURE_ALARM; }
        if status.maximum_temperature_alarm { flags |= flags::SystemStatusFlags::MAXIMUM_TEMPERATURE_ALARM; }
        if status.low_voltage_alarm { flags |= flags::SystemStatusFlags::LOW_VOLTAGE_ALARM; }
        if status.high_voltage_alarm { flags |= flags::SystemStatusFlags::HIGH_VOLTAGE_ALARM; }
        if status.gnss_antenna_disconnected { flags |= flags::SystemStatusFlags::GNSS_ANTENNA_DISCONNECTED; }
        if status.serial_port_overflow_alarm { flags |= flags::SystemStatusFlags::SERIAL_PORT_OVERFLOW_ALARM; }
        flags
    }
}

/// GNSS fix type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssFixType {
    NoFix = 0,
    Fix2D = 1,
    Fix3D = 2,
    SbassFix = 3,
    DifferentialFix = 4,
    OmnistarFix = 5,
    RtkFloat = 6,
    RtkFixed = 7,
}

/// Filter status - individual fields instead of bitflags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl From<flags::FilterStatusFlags> for FilterStatus {
    fn from(flags: flags::FilterStatusFlags) -> Self {
        let fix_type = match flags.gnss_fix_type() {
            0 => GnssFixType::NoFix,
            1 => GnssFixType::Fix2D,
            2 => GnssFixType::Fix3D,
            3 => GnssFixType::SbassFix,
            4 => GnssFixType::DifferentialFix,
            5 => GnssFixType::OmnistarFix,
            6 => GnssFixType::RtkFloat,
            7 => GnssFixType::RtkFixed,
            _ => GnssFixType::NoFix,
        };

        Self {
            orientation_filter_initialised: flags.contains(flags::FilterStatusFlags::ORIENTATION_FILTER_INITIALISED),
            navigation_filter_initialised: flags.contains(flags::FilterStatusFlags::NAVIGATION_FILTER_INITIALISED),
            heading_initialised: flags.contains(flags::FilterStatusFlags::HEADING_INITIALISED),
            utc_time_initialised: flags.contains(flags::FilterStatusFlags::UTC_TIME_INITIALISED),
            gnss_fix_type: fix_type,
            event1_flag: flags.contains(flags::FilterStatusFlags::EVENT1_FLAG),
            event2_flag: flags.contains(flags::FilterStatusFlags::EVENT2_FLAG),
            internal_gnss_enabled: flags.contains(flags::FilterStatusFlags::INTERNAL_GNSS_ENABLED),
            dual_antenna_heading_active: flags.contains(flags::FilterStatusFlags::DUAL_ANTENNA_HEADING_ACTIVE),
            velocity_heading_enabled: flags.contains(flags::FilterStatusFlags::VELOCITY_HEADING_ENABLED),
            atmospheric_altitude_enabled: flags.contains(flags::FilterStatusFlags::ATMOSPHERIC_ALTITUDE_ENABLED),
            external_position_active: flags.contains(flags::FilterStatusFlags::EXTERNAL_POSITION_ACTIVE),
            external_velocity_active: flags.contains(flags::FilterStatusFlags::EXTERNAL_VELOCITY_ACTIVE),
            external_heading_active: flags.contains(flags::FilterStatusFlags::EXTERNAL_HEADING_ACTIVE),
        }
    }
}

impl From<FilterStatus> for flags::FilterStatusFlags {
    fn from(status: FilterStatus) -> Self {
        // Set the GNSS fix type bits (4-6)
        let fix_bits = (status.gnss_fix_type as u16) << 4;
        let mut flags = flags::FilterStatusFlags::from_bits_truncate(fix_bits);

        if status.orientation_filter_initialised { flags |= flags::FilterStatusFlags::ORIENTATION_FILTER_INITIALISED; }
        if status.navigation_filter_initialised { flags |= flags::FilterStatusFlags::NAVIGATION_FILTER_INITIALISED; }
        if status.heading_initialised { flags |= flags::FilterStatusFlags::HEADING_INITIALISED; }
        if status.utc_time_initialised { flags |= flags::FilterStatusFlags::UTC_TIME_INITIALISED; }
        if status.event1_flag { flags |= flags::FilterStatusFlags::EVENT1_FLAG; }
        if status.event2_flag { flags |= flags::FilterStatusFlags::EVENT2_FLAG; }
        if status.internal_gnss_enabled { flags |= flags::FilterStatusFlags::INTERNAL_GNSS_ENABLED; }
        if status.dual_antenna_heading_active { flags |= flags::FilterStatusFlags::DUAL_ANTENNA_HEADING_ACTIVE; }
        if status.velocity_heading_enabled { flags |= flags::FilterStatusFlags::VELOCITY_HEADING_ENABLED; }
        if status.atmospheric_altitude_enabled { flags |= flags::FilterStatusFlags::ATMOSPHERIC_ALTITUDE_ENABLED; }
        if status.external_position_active { flags |= flags::FilterStatusFlags::EXTERNAL_POSITION_ACTIVE; }
        if status.external_velocity_active { flags |= flags::FilterStatusFlags::EXTERNAL_VELOCITY_ACTIVE; }
        if status.external_heading_active { flags |= flags::FilterStatusFlags::EXTERNAL_HEADING_ACTIVE; }
        flags
    }
}

/// System state - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl From<state::SystemStatePacket> for SystemState {
    fn from(p: state::SystemStatePacket) -> Self {
        Self {
            system_status: p.system_status.into(),
            filter_status: p.filter_status.into(),
            unix_time_seconds: p.unix_time_seconds,
            microseconds: p.microseconds,
            latitude: p.latitude,
            longitude: p.longitude,
            height: p.height,
            velocity_north: p.velocity_north,
            velocity_east: p.velocity_east,
            velocity_down: p.velocity_down,
            body_acceleration_x: p.body_acceleration_x,
            body_acceleration_y: p.body_acceleration_y,
            body_acceleration_z: p.body_acceleration_z,
            g_force: p.g_force,
            roll: p.roll,
            pitch: p.pitch,
            heading: p.heading,
            angular_velocity_x: p.angular_velocity_x,
            angular_velocity_y: p.angular_velocity_y,
            angular_velocity_z: p.angular_velocity_z,
            latitude_std_dev: p.latitude_std_dev,
            longitude_std_dev: p.longitude_std_dev,
            height_std_dev: p.height_std_dev,
        }
    }
}

impl From<SystemState> for state::SystemStatePacket {
    fn from(s: SystemState) -> Self {
        Self {
            system_status: s.system_status.into(),
            filter_status: s.filter_status.into(),
            unix_time_seconds: s.unix_time_seconds,
            microseconds: s.microseconds,
            latitude: s.latitude,
            longitude: s.longitude,
            height: s.height,
            velocity_north: s.velocity_north,
            velocity_east: s.velocity_east,
            velocity_down: s.velocity_down,
            body_acceleration_x: s.body_acceleration_x,
            body_acceleration_y: s.body_acceleration_y,
            body_acceleration_z: s.body_acceleration_z,
            g_force: s.g_force,
            roll: s.roll,
            pitch: s.pitch,
            heading: s.heading,
            angular_velocity_x: s.angular_velocity_x,
            angular_velocity_y: s.angular_velocity_y,
            angular_velocity_z: s.angular_velocity_z,
            latitude_std_dev: s.latitude_std_dev,
            longitude_std_dev: s.longitude_std_dev,
            height_std_dev: s.height_std_dev,
        }
    }
}

/// Unix time - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnixTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

impl From<state::UnixTimePacket> for UnixTime {
    fn from(p: state::UnixTimePacket) -> Self {
        Self {
            unix_time_seconds: p.unix_time_seconds,
            microseconds: p.microseconds,
        }
    }
}

impl From<UnixTime> for state::UnixTimePacket {
    fn from(t: UnixTime) -> Self {
        Self {
            unix_time_seconds: t.unix_time_seconds,
            microseconds: t.microseconds,
        }
    }
}

/// Status - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub system_status: SystemStatus,
    pub filter_status: FilterStatus,
}

impl From<state::StatusPacket> for Status {
    fn from(p: state::StatusPacket) -> Self {
        Self {
            system_status: p.system_status.into(),
            filter_status: p.filter_status.into(),
        }
    }
}

impl From<Status> for state::StatusPacket {
    fn from(s: Status) -> Self {
        Self {
            system_status: s.system_status.into(),
            filter_status: s.filter_status.into(),
        }
    }
}

/// Euler orientation standard deviation - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EulerOrientationStdDev {
    pub roll_std_dev: f32,
    pub pitch_std_dev: f32,
    pub heading_std_dev: f32,
}

impl From<state::EulerOrientationStdDevPacket> for EulerOrientationStdDev {
    fn from(p: state::EulerOrientationStdDevPacket) -> Self {
        Self {
            roll_std_dev: p.roll_std_dev,
            pitch_std_dev: p.pitch_std_dev,
            heading_std_dev: p.heading_std_dev,
        }
    }
}

impl From<EulerOrientationStdDev> for state::EulerOrientationStdDevPacket {
    fn from(e: EulerOrientationStdDev) -> Self {
        Self {
            roll_std_dev: e.roll_std_dev,
            pitch_std_dev: e.pitch_std_dev,
            heading_std_dev: e.heading_std_dev,
        }
    }
}

/// External time - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalTime {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

impl From<state::ExternalTimePacket> for ExternalTime {
    fn from(p: state::ExternalTimePacket) -> Self {
        Self {
            unix_time_seconds: p.unix_time_seconds,
            microseconds: p.microseconds,
        }
    }
}

impl From<ExternalTime> for state::ExternalTimePacket {
    fn from(t: ExternalTime) -> Self {
        Self {
            unix_time_seconds: t.unix_time_seconds,
            microseconds: t.microseconds,
        }
    }
}

/// Satellites - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Satellites {
    pub hdop: f32,
    pub vdop: f32,
    pub gps_satellites: u8,
    pub glonass_satellites: u8,
    pub beidou_satellites: u8,
    pub galileo_satellites: u8,
    pub sbas_satellites: u8,
}

impl From<state::SatellitesPacket> for Satellites {
    fn from(p: state::SatellitesPacket) -> Self {
        Self {
            hdop: p.hdop,
            vdop: p.vdop,
            gps_satellites: p.gps_satellites,
            glonass_satellites: p.glonass_satellites,
            beidou_satellites: p.beidou_satellites,
            galileo_satellites: p.galileo_satellites,
            sbas_satellites: p.sbas_satellites,
        }
    }
}

impl From<Satellites> for state::SatellitesPacket {
    fn from(s: Satellites) -> Self {
        Self {
            hdop: s.hdop,
            vdop: s.vdop,
            gps_satellites: s.gps_satellites,
            glonass_satellites: s.glonass_satellites,
            beidou_satellites: s.beidou_satellites,
            galileo_satellites: s.galileo_satellites,
            sbas_satellites: s.sbas_satellites,
        }
    }
}

/// Heave - clean API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heave {
    pub heave_point_1: f32,
    pub heave_point_2: f32,
    pub heave_point_3: f32,
    pub heave_point_4: f32,
}

impl From<state::HeavePacket> for Heave {
    fn from(p: state::HeavePacket) -> Self {
        Self {
            heave_point_1: p.heave_point_1,
            heave_point_2: p.heave_point_2,
            heave_point_3: p.heave_point_3,
            heave_point_4: p.heave_point_4,
        }
    }
}

impl From<Heave> for state::HeavePacket {
    fn from(h: Heave) -> Self {
        Self {
            heave_point_1: h.heave_point_1,
            heave_point_2: h.heave_point_2,
            heave_point_3: h.heave_point_3,
            heave_point_4: h.heave_point_4,
        }
    }
}

/// Sensor temperature - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorTemperature {
    pub accelerometer_temp_0: f32,
    pub accelerometer_temp_1: f32,
    pub accelerometer_temp_2: f32,
    pub gyroscope_temp_0: f32,
    pub gyroscope_temp_1: f32,
    pub gyroscope_temp_2: f32,
    pub pressure_sensor_temp: f32,
}

impl From<state::SensorTemperaturePacket> for SensorTemperature {
    fn from(p: state::SensorTemperaturePacket) -> Self {
        Self {
            accelerometer_temp_0: p.accelerometer_temp_0,
            accelerometer_temp_1: p.accelerometer_temp_1,
            accelerometer_temp_2: p.accelerometer_temp_2,
            gyroscope_temp_0: p.gyroscope_temp_0,
            gyroscope_temp_1: p.gyroscope_temp_1,
            gyroscope_temp_2: p.gyroscope_temp_2,
            pressure_sensor_temp: p.pressure_sensor_temp,
        }
    }
}

impl From<SensorTemperature> for state::SensorTemperaturePacket {
    fn from(t: SensorTemperature) -> Self {
        Self {
            accelerometer_temp_0: t.accelerometer_temp_0,
            accelerometer_temp_1: t.accelerometer_temp_1,
            accelerometer_temp_2: t.accelerometer_temp_2,
            gyroscope_temp_0: t.gyroscope_temp_0,
            gyroscope_temp_1: t.gyroscope_temp_1,
            gyroscope_temp_2: t.gyroscope_temp_2,
            reserved: 0.0,  // Auto-filled
            pressure_sensor_temp: t.pressure_sensor_temp,
        }
    }
}

/// Raw sensors - clean API (no reserved fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSensors {
    pub accelerometer_x: f32,
    pub accelerometer_y: f32,
    pub accelerometer_z: f32,
    pub gyroscope_x: f32,
    pub gyroscope_y: f32,
    pub gyroscope_z: f32,
    pub imu_temperature: f32,
    pub pressure: f32,
    pub pressure_temperature: f32,
}

impl From<state::RawSensorsPacket> for RawSensors {
    fn from(p: state::RawSensorsPacket) -> Self {
        Self {
            accelerometer_x: p.accelerometer_x,
            accelerometer_y: p.accelerometer_y,
            accelerometer_z: p.accelerometer_z,
            gyroscope_x: p.gyroscope_x,
            gyroscope_y: p.gyroscope_y,
            gyroscope_z: p.gyroscope_z,
            imu_temperature: p.imu_temperature,
            pressure: p.pressure,
            pressure_temperature: p.pressure_temperature,
        }
    }
}

impl From<RawSensors> for state::RawSensorsPacket {
    fn from(r: RawSensors) -> Self {
        Self {
            accelerometer_x: r.accelerometer_x,
            accelerometer_y: r.accelerometer_y,
            accelerometer_z: r.accelerometer_z,
            gyroscope_x: r.gyroscope_x,
            gyroscope_y: r.gyroscope_y,
            gyroscope_z: r.gyroscope_z,
            reserved1: 0.0,  // Auto-filled
            reserved2: 0.0,  // Auto-filled
            reserved3: 0.0,  // Auto-filled
            imu_temperature: r.imu_temperature,
            pressure: r.pressure,
            pressure_temperature: r.pressure_temperature,
        }
    }
}
