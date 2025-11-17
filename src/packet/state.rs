use binrw::{BinRead, BinWrite};
use serde::{Serialize, Deserialize};
use super::flags::{SystemStatusFlags, FilterStatusFlags};
use crate::packet::flags;

/// State Packets (20-23)
/// System state packet structure (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct SystemStatePacket {
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
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct UnixTimePacket {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}


/// Status packet structure (Packet ID 23, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct StatusPacket {
    pub system_status: SystemStatusFlags,
    pub filter_status: FilterStatusFlags,
}


/// Euler Orientation Standard Deviation packet structure (Packet ID 26, Length 12) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct EulerOrientationStdDevPacket {
    /// Roll standard deviation in radians (Field 1)
    pub roll_std_dev: f32,
    /// Pitch standard deviation in radians (Field 2)
    pub pitch_std_dev: f32,
    /// Heading standard deviation in radians (Field 3)
    pub heading_std_dev: f32,
}


/// External Time packet structure (Packet ID 52, Length 8) - Write only
/// Used to send external time to the device when GNSS is unavailable
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct ExternalTimePacket {
    /// Unix time in seconds since epoch (Field 1)
    pub unix_time_seconds: u32,
    /// Microseconds component (Field 2)
    pub microseconds: u32,
}


/// Satellites packet structure (Packet ID 30, Length 13) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct SatellitesPacket {
    /// Horizontal Dilution of Precision (Field 1)
    pub hdop: f32,
    /// Vertical Dilution of Precision (Field 2)
    pub vdop: f32,
    /// GPS satellites (Field 3)
    pub gps_satellites: u8,
    /// GLONASS satellites (Field 4)
    pub glonass_satellites: u8,
    /// BeiDou satellites (Field 5)
    pub beidou_satellites: u8,
    /// Galileo satellites (Field 6)
    pub galileo_satellites: u8,
    /// SBAS satellites (Field 7)
    pub sbas_satellites: u8,
}


/// Heave packet structure (Packet ID 58, Length 16) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct HeavePacket {
    /// Heave point 1 in meters (Field 1)
    pub heave_point_1: f32,
    /// Heave point 2 in meters (Field 2)
    pub heave_point_2: f32,
    /// Heave point 3 in meters (Field 3)
    pub heave_point_3: f32,
    /// Heave point 4 in meters (Field 4)
    pub heave_point_4: f32,
}


/// Sensor temperature packet structure (Packet ID 85, Length 32) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct SensorTemperaturePacket {
    /// Accelerometer temperature - axis 0 in deg C (Field 1[0])
    pub accelerometer_temp_0: f32,
    /// Accelerometer temperature - axis 1 in deg C (Field 1[1])
    pub accelerometer_temp_1: f32,
    /// Accelerometer temperature - axis 2 in deg C (Field 1[2])
    pub accelerometer_temp_2: f32,
    /// Gyroscope temperature - axis 0 in deg C (Field 2[0])
    pub gyroscope_temp_0: f32,
    /// Gyroscope temperature - axis 1 in deg C (Field 2[1])
    pub gyroscope_temp_1: f32,
    /// Gyroscope temperature - axis 2 in deg C (Field 2[2])
    pub gyroscope_temp_2: f32,
    /// Reserved (Field 3)
    pub reserved: f32,
    /// Pressure sensor temperature in deg C (Field 4)
    pub pressure_sensor_temp: f32,
}


/// Raw sensors packet structure (Packet ID 28, Length 48) - Read only
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub(crate) struct RawSensorsPacket {
    /// Accelerometer X in m/s/s (Field 1)
    pub accelerometer_x: f32,
    /// Accelerometer Y in m/s/s (Field 2)
    pub accelerometer_y: f32,
    /// Accelerometer Z in m/s/s (Field 3)
    pub accelerometer_z: f32,
    /// Gyroscope X in rad/s (Field 4)
    pub gyroscope_x: f32,
    /// Gyroscope Y in rad/s (Field 5)
    pub gyroscope_y: f32,
    /// Gyroscope Z in rad/s (Field 6)
    pub gyroscope_z: f32,
    /// Reserved (Field 7)
    pub reserved1: f32,
    /// Reserved (Field 8)
    pub reserved2: f32,
    /// Reserved (Field 9)
    pub reserved3: f32,
    /// IMU Temperature in deg C (Field 10)
    pub imu_temperature: f32,
    /// Pressure in Pascals (Field 11)
    pub pressure: f32,
    /// Pressure Temperature in deg C (Field 12)
    pub pressure_temperature: f32,
}

// Public API Types

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

impl From<SystemStatePacket> for SystemState {
    fn from(p: SystemStatePacket) -> Self {
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

impl From<SystemState> for SystemStatePacket {
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

impl From<UnixTimePacket> for UnixTime {
    fn from(p: UnixTimePacket) -> Self {
        Self {
            unix_time_seconds: p.unix_time_seconds,
            microseconds: p.microseconds,
        }
    }
}

impl From<UnixTime> for UnixTimePacket {
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

impl From<StatusPacket> for Status {
    fn from(p: StatusPacket) -> Self {
        Self {
            system_status: p.system_status.into(),
            filter_status: p.filter_status.into(),
        }
    }
}

impl From<Status> for StatusPacket {
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

impl From<EulerOrientationStdDevPacket> for EulerOrientationStdDev {
    fn from(p: EulerOrientationStdDevPacket) -> Self {
        Self {
            roll_std_dev: p.roll_std_dev,
            pitch_std_dev: p.pitch_std_dev,
            heading_std_dev: p.heading_std_dev,
        }
    }
}

impl From<EulerOrientationStdDev> for EulerOrientationStdDevPacket {
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

impl From<ExternalTimePacket> for ExternalTime {
    fn from(p: ExternalTimePacket) -> Self {
        Self {
            unix_time_seconds: p.unix_time_seconds,
            microseconds: p.microseconds,
        }
    }
}

impl From<ExternalTime> for ExternalTimePacket {
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

impl From<SatellitesPacket> for Satellites {
    fn from(p: SatellitesPacket) -> Self {
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

impl From<Satellites> for SatellitesPacket {
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

impl From<HeavePacket> for Heave {
    fn from(p: HeavePacket) -> Self {
        Self {
            heave_point_1: p.heave_point_1,
            heave_point_2: p.heave_point_2,
            heave_point_3: p.heave_point_3,
            heave_point_4: p.heave_point_4,
        }
    }
}

impl From<Heave> for HeavePacket {
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

impl From<SensorTemperaturePacket> for SensorTemperature {
    fn from(p: SensorTemperaturePacket) -> Self {
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

impl From<SensorTemperature> for SensorTemperaturePacket {
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

impl From<RawSensorsPacket> for RawSensors {
    fn from(p: RawSensorsPacket) -> Self {
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

impl From<RawSensors> for RawSensorsPacket {
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