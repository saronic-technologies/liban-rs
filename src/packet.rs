use crate::error::{AnError, Result};
use serde::{Deserialize, Serialize};

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
    SerialPortPassthrough = 10,
    IpConfiguration = 11,
    SubcomponentInformation = 14,
    // State Packets
    SystemState = 20,
    UnixTime = 21,
    FormattedTime = 22,
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
            10 => Some(Self::SerialPortPassthrough),
            11 => Some(Self::IpConfiguration),
            14 => Some(Self::SubcomponentInformation),
            20 => Some(Self::SystemState),
            21 => Some(Self::UnixTime),
            22 => Some(Self::FormattedTime),
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

/// Trait for ANPP packet serialization/deserialization
pub trait AnppPacket: Serialize + for<'de> Deserialize<'de> + Sized {
    /// Parse packet from raw bytes using little-endian format
    fn from_bytes(data: &[u8]) -> Result<Self> {
        // Use bincode with little-endian configuration for ANPP protocol
        bincode::deserialize(data)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize packet: {}", e)))
    }

    /// Serialize packet to bytes using little-endian format  
    fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize packet: {}", e)))
    }
}

/// System Packets (0-14)

/// Acknowledge packet structure (Packet ID 0, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgePacket {
    pub packet_id: u8,
    pub packet_crc: u16,
    pub result: u8,
}

impl AnppPacket for AcknowledgePacket {}

/// Request packet structure (Packet ID 1, Variable length) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestPacket {
    pub packet_id: u8,
}

impl AnppPacket for RequestPacket {}

/// Boot mode packet structure (Packet ID 2, Length 1) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootModePacket {
    pub boot_mode: u8,
}

impl AnppPacket for BootModePacket {}

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

impl AnppPacket for DeviceInformation {}

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

impl AnppPacket for RestoreFactorySettingsPacket {}

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

impl AnppPacket for ResetPacket {}

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

/// Serial port passthrough packet structure (Packet ID 10, Variable length) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerialPortPassthroughPacket {
    pub data: Vec<u8>,
}

impl AnppPacket for SerialPortPassthroughPacket {}

/// IP configuration packet structure (Packet ID 11, Length 30) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpConfigurationPacket {
    pub permanent: u8,
    pub dhcp_mode: u8,
    pub ip_address: u32,
    pub ip_netmask: u32,
    pub ip_gateway: u32,
    pub dns_server: u32,
    pub serial_number_1: u32,
    pub serial_number_2: u32,
    pub serial_number_3: u32,
}

impl AnppPacket for IpConfigurationPacket {}

impl IpConfigurationPacket {
    /// Convert IP address from u32 to dotted decimal notation
    pub fn get_ip_address(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::from(self.ip_address.to_le_bytes())
    }

    /// Convert netmask from u32 to dotted decimal notation
    pub fn get_ip_netmask(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::from(self.ip_netmask.to_le_bytes())
    }

    /// Convert gateway from u32 to dotted decimal notation
    pub fn get_ip_gateway(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::from(self.ip_gateway.to_le_bytes())
    }

    /// Convert DNS server from u32 to dotted decimal notation
    pub fn get_dns_server(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::from(self.dns_server.to_le_bytes())
    }

    /// Get the complete serial number as a formatted string
    pub fn get_serial_number(&self) -> String {
        format!(
            "{}-{}-{}",
            self.serial_number_1, self.serial_number_2, self.serial_number_3
        )
    }

    /// Set IP address from dotted decimal notation
    pub fn set_ip_address(&mut self, addr: std::net::Ipv4Addr) {
        self.ip_address = u32::from_le_bytes(addr.octets());
    }

    /// Set netmask from dotted decimal notation
    pub fn set_ip_netmask(&mut self, addr: std::net::Ipv4Addr) {
        self.ip_netmask = u32::from_le_bytes(addr.octets());
    }

    /// Set gateway from dotted decimal notation
    pub fn set_ip_gateway(&mut self, addr: std::net::Ipv4Addr) {
        self.ip_gateway = u32::from_le_bytes(addr.octets());
    }

    /// Set DNS server from dotted decimal notation
    pub fn set_dns_server(&mut self, addr: std::net::Ipv4Addr) {
        self.dns_server = u32::from_le_bytes(addr.octets());
    }
}

/// Subcomponent information packet structure (Packet ID 14, Length 24) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubcomponentInformationPacket {
    pub software_version: u32,
    pub device_id: u32,
    pub hardware_revision: u32,
    pub serial_number: u32,
    pub hardware_id: u32,
    pub firmware_version: u32,
}

impl AnppPacket for SubcomponentInformationPacket {}

/// State Packets (20-23)

/// System state information (Packet ID 20, Length 100) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemState {
    pub system_status: u16,
    pub filter_status: u16,
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
    pub latitude_standard_deviation: f32,
    pub longitude_standard_deviation: f32,
    pub height_standard_deviation: f32,
}

impl AnppPacket for SystemState {}

impl SystemState {
    /// Get interpreted system status flags
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus::new(self.system_status)
    }

    /// Get interpreted filter status flags
    pub fn get_filter_status(&self) -> FilterStatus {
        FilterStatus::new(self.filter_status)
    }

    /// Get the maximum position standard deviation
    pub fn get_max_position_standard_deviation(&self) -> f32 {
        self.latitude_standard_deviation
            .max(self.longitude_standard_deviation)
            .max(self.height_standard_deviation)
    }
}

/// System status flags interpretation for System State packet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemStatus {
    pub raw_value: u16,
}

impl SystemStatus {
    /// Create a new SystemStatus from raw u16 value
    pub fn new(raw_value: u16) -> Self {
        Self { raw_value }
    }

    /// Check if system failure is indicated (bit 0)
    pub fn system_failure(&self) -> bool {
        (self.raw_value & 0x0001) != 0
    }

    /// Check if accelerometer sensor failure is indicated (bit 1)
    pub fn accelerometer_sensor_failure(&self) -> bool {
        (self.raw_value & 0x0002) != 0
    }

    /// Check if gyroscope sensor failure is indicated (bit 2)
    pub fn gyroscope_sensor_failure(&self) -> bool {
        (self.raw_value & 0x0004) != 0
    }

    /// Check if pressure sensor failure is indicated (bit 4)
    pub fn pressure_sensor_failure(&self) -> bool {
        (self.raw_value & 0x0010) != 0
    }

    /// Check if accelerometer over range is indicated (bit 6)
    pub fn accelerometer_over_range(&self) -> bool {
        (self.raw_value & 0x0040) != 0
    }

    /// Check if gyroscope over range is indicated (bit 7)
    pub fn gyroscope_over_range(&self) -> bool {
        (self.raw_value & 0x0080) != 0
    }

    /// Check if pressure over range is indicated (bit 9)
    pub fn pressure_over_range(&self) -> bool {
        (self.raw_value & 0x0200) != 0
    }

    /// Check if minimum temperature alarm is indicated (bit 10)
    pub fn minimum_temperature_alarm(&self) -> bool {
        (self.raw_value & 0x0400) != 0
    }

    /// Check if maximum temperature alarm is indicated (bit 11)
    pub fn maximum_temperature_alarm(&self) -> bool {
        (self.raw_value & 0x0800) != 0
    }

    /// Check if internal data logging error is indicated (bit 12)
    pub fn internal_data_logging_error(&self) -> bool {
        (self.raw_value & 0x1000) != 0
    }

    /// Check if high voltage alarm is indicated (bit 13)
    pub fn high_voltage_alarm(&self) -> bool {
        (self.raw_value & 0x2000) != 0
    }

    /// Check if data output overflow alarm is indicated (bit 15)
    pub fn data_output_overflow_alarm(&self) -> bool {
        (self.raw_value & 0x8000) != 0
    }

    /// Get a summary of all active alarms and failures
    pub fn get_active_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        if self.system_failure() {
            flags.push("System Failure".to_string());
        }
        if self.accelerometer_sensor_failure() {
            flags.push("Accelerometer Sensor Failure".to_string());
        }
        if self.gyroscope_sensor_failure() {
            flags.push("Gyroscope Sensor Failure".to_string());
        }
        if self.pressure_sensor_failure() {
            flags.push("Pressure Sensor Failure".to_string());
        }
        if self.accelerometer_over_range() {
            flags.push("Accelerometer Over Range".to_string());
        }
        if self.gyroscope_over_range() {
            flags.push("Gyroscope Over Range".to_string());
        }
        if self.pressure_over_range() {
            flags.push("Pressure Over Range".to_string());
        }
        if self.minimum_temperature_alarm() {
            flags.push("Minimum Temperature Alarm".to_string());
        }
        if self.maximum_temperature_alarm() {
            flags.push("Maximum Temperature Alarm".to_string());
        }
        if self.internal_data_logging_error() {
            flags.push("Internal Data Logging Error".to_string());
        }
        if self.high_voltage_alarm() {
            flags.push("High Voltage Alarm".to_string());
        }
        if self.data_output_overflow_alarm() {
            flags.push("Data Output Overflow Alarm".to_string());
        }

        flags
    }

    /// Check if system is healthy (no flags set)
    pub fn is_healthy(&self) -> bool {
        self.raw_value == 0
    }
}

impl From<u16> for SystemStatus {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for SystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_healthy() {
            write!(f, "System Status: Healthy (0x{:04X})", self.raw_value)
        } else {
            let flags = self.get_active_flags();
            write!(
                f,
                "System Status: {} (0x{:04X})",
                flags.join(", "),
                self.raw_value
            )
        }
    }
}

/// Filter status flags interpretation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterStatus {
    pub raw_value: u16,
}

impl FilterStatus {
    /// Create a new FilterStatus from raw u16 value
    pub fn new(raw_value: u16) -> Self {
        Self { raw_value }
    }

    /// Check if orientation filter is initialised (bit 0)
    pub fn orientation_filter_initialised(&self) -> bool {
        (self.raw_value & 0x0001) != 0
    }

    /// Check if navigation filter is initialised (bit 1)
    pub fn navigation_filter_initialised(&self) -> bool {
        (self.raw_value & 0x0002) != 0
    }

    /// Check if heading is initialised (bit 2)
    pub fn heading_initialised(&self) -> bool {
        (self.raw_value & 0x0004) != 0
    }

    /// Check if UTC time is initialised (bit 3)
    pub fn utc_time_initialised(&self) -> bool {
        (self.raw_value & 0x0008) != 0
    }

    /// Check GNSS fix status (bit 4)
    pub fn gnss_fix_status(&self) -> bool {
        (self.raw_value & 0x0010) != 0
    }

    /// Check if Event 1 occurred (bit 7)
    pub fn event_1_occurred(&self) -> bool {
        (self.raw_value & 0x0080) != 0
    }

    /// Check if Event 2 occurred (bit 8)
    pub fn event_2_occurred(&self) -> bool {
        (self.raw_value & 0x0100) != 0
    }

    /// Check if internal GNSS is enabled (bit 9)
    pub fn internal_gnss_enabled(&self) -> bool {
        (self.raw_value & 0x0200) != 0
    }

    /// Check if heading is active (bit 10)
    pub fn heading_active(&self) -> bool {
        (self.raw_value & 0x0400) != 0
    }

    /// Check if velocity heading is enabled (bit 11)
    pub fn velocity_heading_enabled(&self) -> bool {
        (self.raw_value & 0x0800) != 0
    }

    /// Check if atmospheric altitude is enabled (bit 12)
    pub fn atmospheric_altitude_enabled(&self) -> bool {
        (self.raw_value & 0x1000) != 0
    }

    /// Check if external position is active (bit 13)
    pub fn external_position_active(&self) -> bool {
        (self.raw_value & 0x2000) != 0
    }

    /// Check if external velocity is active (bit 14)
    pub fn external_velocity_active(&self) -> bool {
        (self.raw_value & 0x4000) != 0
    }

    /// Check if external heading is active (bit 15)
    pub fn external_heading_active(&self) -> bool {
        (self.raw_value & 0x8000) != 0
    }

    /// Get a summary of all active filter status flags
    pub fn get_active_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        if self.orientation_filter_initialised() {
            flags.push("Orientation Filter Initialised".to_string());
        }
        if self.navigation_filter_initialised() {
            flags.push("Navigation Filter Initialised".to_string());
        }
        if self.heading_initialised() {
            flags.push("Heading Initialised".to_string());
        }
        if self.utc_time_initialised() {
            flags.push("UTC Time Initialised".to_string());
        }
        if self.gnss_fix_status() {
            flags.push("GNSS Fix Status".to_string());
        }
        if self.event_1_occurred() {
            flags.push("Event 1 Occurred".to_string());
        }
        if self.event_2_occurred() {
            flags.push("Event 2 Occurred".to_string());
        }
        if self.internal_gnss_enabled() {
            flags.push("Internal GNSS Enabled".to_string());
        }
        if self.heading_active() {
            flags.push("Heading Active".to_string());
        }
        if self.velocity_heading_enabled() {
            flags.push("Velocity Heading Enabled".to_string());
        }
        if self.atmospheric_altitude_enabled() {
            flags.push("Atmospheric Altitude Enabled".to_string());
        }
        if self.external_position_active() {
            flags.push("External Position Active".to_string());
        }
        if self.external_velocity_active() {
            flags.push("External Velocity Active".to_string());
        }
        if self.external_heading_active() {
            flags.push("External Heading Active".to_string());
        }

        flags
    }

    /// Check if all critical filters are initialised
    pub fn is_fully_initialised(&self) -> bool {
        self.orientation_filter_initialised()
            && self.navigation_filter_initialised()
            && self.heading_initialised()
            && self.utc_time_initialised()
    }
}

impl From<u16> for FilterStatus {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for FilterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_fully_initialised() {
            let active_flags = self.get_active_flags();
            if active_flags.is_empty() {
                write!(
                    f,
                    "Filter Status: Fully Initialised (0x{:04X})",
                    self.raw_value
                )
            } else {
                write!(
                    f,
                    "Filter Status: {} (0x{:04X})",
                    active_flags.join(", "),
                    self.raw_value
                )
            }
        } else {
            let active_flags = self.get_active_flags();
            if active_flags.is_empty() {
                write!(
                    f,
                    "Filter Status: Not Initialised (0x{:04X})",
                    self.raw_value
                )
            } else {
                write!(
                    f,
                    "Filter Status: {} (0x{:04X})",
                    active_flags.join(", "),
                    self.raw_value
                )
            }
        }
    }
}

/// Unix time packet structure (Packet ID 21, Length 8) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnixTimePacket {
    pub unix_time_seconds: u32,
    pub microseconds: u32,
}

impl AnppPacket for UnixTimePacket {}

/// Formatted time packet structure (Packet ID 22, Length 14) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormattedTimePacket {
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

impl AnppPacket for FormattedTimePacket {}

/// Status packet structure (Packet ID 23, Length 4) - Read only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusPacket {
    pub system_status: u16,
    pub filter_status: u16,
}

impl AnppPacket for StatusPacket {}

impl StatusPacket {
    /// Get interpreted system status flags
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus::new(self.system_status)
    }

    /// Get interpreted filter status flags
    pub fn get_filter_status(&self) -> FilterStatus {
        FilterStatus::new(self.filter_status)
    }
}

/// Configuration Packets (180-203)

/// Packet timer period packet (Packet ID 180, Length 4) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketTimerPeriodPacket {
    pub permanent: u8,
    pub utc_synchronisation: u8,
    pub packet_timer_period: u16,
}

impl AnppPacket for PacketTimerPeriodPacket {}

impl PacketTimerPeriodPacket {
    /// Create a new packet timer period configuration
    ///
    /// # Arguments
    /// * `permanent` - Whether the setting is permanent (0 = no, 1 = yes)
    /// * `period_microseconds` - Timer period in microseconds (1000-65000, increments of 1000)
    /// * `enable_utc_sync` - Whether to enable UTC synchronization (if valid)
    pub fn new(permanent: bool, period_microseconds: u16, enable_utc_sync: bool) -> Result<Self> {
        // Validate period range and increment
        if period_microseconds < 1000 || period_microseconds > 65000 {
            return Err(AnError::ValidationFailed(
                "Packet timer period must be between 1000 and 65000 microseconds".to_string(),
            ));
        }

        if period_microseconds % 1000 != 0 {
            return Err(AnError::ValidationFailed(
                "Packet timer period must be in increments of 1000 microseconds".to_string(),
            ));
        }

        // Check UTC synchronization validity
        let utc_sync = if enable_utc_sync && Self::is_utc_sync_valid(period_microseconds) {
            1
        } else {
            0
        };

        Ok(Self {
            permanent: if permanent { 1 } else { 0 },
            utc_synchronisation: utc_sync,
            packet_timer_period: period_microseconds,
        })
    }

    /// Check if the packet timer period is valid for UTC synchronization
    /// UTC sync is valid when 1,000,000 divides evenly by the period
    pub fn is_utc_sync_valid(period_microseconds: u16) -> bool {
        1_000_000 % period_microseconds as u32 == 0
    }

    /// Get the packet timer rate in Hz
    pub fn get_rate_hz(&self) -> f64 {
        1_000_000.0 / self.packet_timer_period as f64
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Check if UTC synchronization is enabled
    pub fn is_utc_synchronisation_enabled(&self) -> bool {
        self.utc_synchronisation != 0
    }

    /// Get the period in microseconds
    pub fn get_period_microseconds(&self) -> u16 {
        self.packet_timer_period
    }

    /// Get the period in milliseconds
    pub fn get_period_milliseconds(&self) -> f64 {
        self.packet_timer_period as f64 / 1000.0
    }

    /// Validate current configuration
    pub fn validate(&self) -> Result<()> {
        if self.packet_timer_period < 1000 || self.packet_timer_period > 65000 {
            return Err(AnError::ValidationFailed(
                "Invalid packet timer period range".to_string(),
            ));
        }

        if self.packet_timer_period % 1000 != 0 {
            return Err(AnError::ValidationFailed(
                "Packet timer period must be in 1000 microsecond increments".to_string(),
            ));
        }

        if self.utc_synchronisation != 0 && !Self::is_utc_sync_valid(self.packet_timer_period) {
            return Err(AnError::ValidationFailed(
                "UTC synchronization is enabled but period is not valid for UTC sync".to_string(),
            ));
        }

        Ok(())
    }
}

/// Individual packet period entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketPeriodEntry {
    pub packet_id: u8,
    pub packet_period: u32,
}

/// Packets period packet (Packet ID 181, Variable length) - Read/Write
///
/// This packet allows configuration of state packets (packets 20 through 180) periods.
/// Note: This packet only affects the port from which it is sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketsPeriodPacket {
    pub permanent: u8,
    pub clear_existing_packet_periods: u8,
    pub packet_periods: Vec<PacketPeriodEntry>,
}

impl AnppPacket for PacketsPeriodPacket {}

impl PacketPeriodEntry {
    /// Create a new packet period entry
    pub fn new(packet_id: u8, packet_period: u32) -> Result<Self> {
        if packet_id < 20 || packet_id > 180 {
            return Err(AnError::ValidationFailed(
                "Packet ID must be between 20 and 180 for period configuration".to_string(),
            ));
        }

        Ok(Self {
            packet_id,
            packet_period,
        })
    }

    /// Calculate the packet rate in Hz given the packet timer period
    /// Formula: Packet Rate = 1,000,000 / (Packet Period × Packet Timer Period) Hz
    pub fn calculate_rate_hz(&self, packet_timer_period_us: u16) -> f64 {
        1_000_000.0 / (self.packet_period as f64 * packet_timer_period_us as f64)
    }

    /// Calculate the actual period in microseconds given the packet timer period
    pub fn calculate_period_us(&self, packet_timer_period_us: u16) -> u32 {
        self.packet_period * packet_timer_period_us as u32
    }
}

impl PacketsPeriodPacket {
    /// Create a new packets period packet
    pub fn new(permanent: bool, clear_existing: bool) -> Self {
        Self {
            permanent: if permanent { 1 } else { 0 },
            clear_existing_packet_periods: if clear_existing { 1 } else { 0 },
            packet_periods: Vec::new(),
        }
    }

    /// Add a packet period entry
    pub fn add_packet_period(&mut self, packet_id: u8, packet_period: u32) -> Result<()> {
        let entry = PacketPeriodEntry::new(packet_id, packet_period)?;

        // Remove existing entry for this packet ID if it exists
        self.packet_periods.retain(|p| p.packet_id != packet_id);

        self.packet_periods.push(entry);
        Ok(())
    }

    /// Remove a packet period entry by packet ID
    pub fn remove_packet_period(&mut self, packet_id: u8) {
        self.packet_periods.retain(|p| p.packet_id != packet_id);
    }

    /// Get packet period for a specific packet ID
    pub fn get_packet_period(&self, packet_id: u8) -> Option<u32> {
        self.packet_periods
            .iter()
            .find(|p| p.packet_id == packet_id)
            .map(|p| p.packet_period)
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Check if existing packet periods should be cleared
    pub fn should_clear_existing(&self) -> bool {
        self.clear_existing_packet_periods != 0
    }

    /// Get all configured packet IDs
    pub fn get_configured_packet_ids(&self) -> Vec<u8> {
        self.packet_periods.iter().map(|p| p.packet_id).collect()
    }

    /// Calculate rates for all packets given a packet timer period
    pub fn calculate_all_rates(&self, packet_timer_period_us: u16) -> Vec<(u8, f64)> {
        self.packet_periods
            .iter()
            .map(|entry| {
                (
                    entry.packet_id,
                    entry.calculate_rate_hz(packet_timer_period_us),
                )
            })
            .collect()
    }

    /// Clear all packet periods
    pub fn clear_all_periods(&mut self) {
        self.packet_periods.clear();
    }

    /// Get the expected packet length (2 + 5 × number of entries)
    pub fn get_expected_length(&self) -> usize {
        2 + (5 * self.packet_periods.len())
    }

    /// Validate the packet configuration
    pub fn validate(&self) -> Result<()> {
        for entry in &self.packet_periods {
            if entry.packet_id < 20 || entry.packet_id > 180 {
                return Err(AnError::ValidationFailed(format!(
                    "Invalid packet ID {}: must be between 20 and 180",
                    entry.packet_id
                )));
            }
        }

        // Check for duplicate packet IDs
        let mut seen_ids = std::collections::HashSet::new();
        for entry in &self.packet_periods {
            if !seen_ids.insert(entry.packet_id) {
                return Err(AnError::ValidationFailed(format!(
                    "Duplicate packet ID {} found",
                    entry.packet_id
                )));
            }
        }

        Ok(())
    }
}

/// Installation alignment packet (Packet ID 185, Length 73) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationAlignmentPacket {
    pub alignment_dcm: [f32; 9],
    pub gnss_antenna_offset: [f32; 3],
    pub dual_antenna_offset: [f32; 3],
    pub odometer_offset: [f32; 3],
    pub external_data_offset: [f32; 3],
    pub alignment_dcm_uncertainty: [f32; 3],
    pub gnss_antenna_offset_uncertainty: [f32; 3],
    pub dual_antenna_offset_uncertainty: [f32; 3],
    pub odometer_offset_uncertainty: [f32; 3],
    pub external_data_offset_uncertainty: [f32; 3],
}

impl AnppPacket for InstallationAlignmentPacket {}

/// Vehicle type options for filter configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VehicleType {
    Unlimited = 0,
    BicycleOrMotorcycle = 1,
    Car = 2,
    Hovercraft = 3,
    Submarine = 4,
    ThreeDUnderwaterVehicle = 5,
    FixedWingPlane = 6,
    ThreeDAircraft = 7,
    Human = 8,
    Boat = 9,
    LargeShip = 10,
    Stationary = 11,
    StuntPlane = 12,
    RaceCar = 13,
    Train = 14,
}

impl VehicleType {
    /// Convert a u8 value to a VehicleType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unlimited),
            1 => Some(Self::BicycleOrMotorcycle),
            2 => Some(Self::Car),
            3 => Some(Self::Hovercraft),
            4 => Some(Self::Submarine),
            5 => Some(Self::ThreeDUnderwaterVehicle),
            6 => Some(Self::FixedWingPlane),
            7 => Some(Self::ThreeDAircraft),
            8 => Some(Self::Human),
            9 => Some(Self::Boat),
            10 => Some(Self::LargeShip),
            11 => Some(Self::Stationary),
            12 => Some(Self::StuntPlane),
            13 => Some(Self::RaceCar),
            14 => Some(Self::Train),
            _ => None,
        }
    }

    /// Get the u8 value of the VehicleType
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get a human-readable description of the vehicle type
    pub fn description(&self) -> &'static str {
        match self {
            Self::Unlimited => "Unlimited",
            Self::BicycleOrMotorcycle => "Bicycle or Motorcycle",
            Self::Car => "Car",
            Self::Hovercraft => "Hovercraft",
            Self::Submarine => "Submarine",
            Self::ThreeDUnderwaterVehicle => "3D Underwater Vehicle",
            Self::FixedWingPlane => "Fixed Wing Plane",
            Self::ThreeDAircraft => "3D Aircraft",
            Self::Human => "Human",
            Self::Boat => "Boat",
            Self::LargeShip => "Large Ship",
            Self::Stationary => "Stationary",
            Self::StuntPlane => "Stunt Plane",
            Self::RaceCar => "Race Car",
            Self::Train => "Train",
        }
    }
}

impl std::fmt::Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Filter options packet (Packet ID 186, Length 17) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptionsPacket {
    pub permanent: u8,
    pub vehicle_type: u8,
    pub internal_gnss_enabled: u8,
    pub reserved_1: u8,
    pub atmospheric_altitude_enabled: u8,
    pub velocity_heading_enabled: u8,
    pub reversing_detection_enabled: u8,
    pub motion_analysis_enabled: u8,
    pub reserved_2: u8,
    pub reserved_3: [u8; 8],
}

impl AnppPacket for FilterOptionsPacket {}

impl FilterOptionsPacket {
    /// Create a new filter options packet
    pub fn new(permanent: bool, vehicle_type: VehicleType) -> Self {
        Self {
            permanent: if permanent { 1 } else { 0 },
            vehicle_type: vehicle_type.as_u8(),
            internal_gnss_enabled: 0,
            reserved_1: 0,
            atmospheric_altitude_enabled: 0,
            velocity_heading_enabled: 0,
            reversing_detection_enabled: 0,
            motion_analysis_enabled: 0,
            reserved_2: 0,
            reserved_3: [0; 8],
        }
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Get the vehicle type as an enum
    pub fn get_vehicle_type(&self) -> Option<VehicleType> {
        VehicleType::from_u8(self.vehicle_type)
    }

    /// Set the vehicle type
    pub fn set_vehicle_type(&mut self, vehicle_type: VehicleType) {
        self.vehicle_type = vehicle_type.as_u8();
    }

    /// Check if internal GNSS is enabled
    pub fn is_internal_gnss_enabled(&self) -> bool {
        self.internal_gnss_enabled != 0
    }

    /// Enable or disable internal GNSS
    pub fn set_internal_gnss_enabled(&mut self, enabled: bool) {
        self.internal_gnss_enabled = if enabled { 1 } else { 0 };
    }

    /// Check if atmospheric altitude is enabled
    pub fn is_atmospheric_altitude_enabled(&self) -> bool {
        self.atmospheric_altitude_enabled != 0
    }

    /// Enable or disable atmospheric altitude
    pub fn set_atmospheric_altitude_enabled(&mut self, enabled: bool) {
        self.atmospheric_altitude_enabled = if enabled { 1 } else { 0 };
    }

    /// Check if velocity heading is enabled
    pub fn is_velocity_heading_enabled(&self) -> bool {
        self.velocity_heading_enabled != 0
    }

    /// Enable or disable velocity heading
    pub fn set_velocity_heading_enabled(&mut self, enabled: bool) {
        self.velocity_heading_enabled = if enabled { 1 } else { 0 };
    }

    /// Check if reversing detection is enabled
    pub fn is_reversing_detection_enabled(&self) -> bool {
        self.reversing_detection_enabled != 0
    }

    /// Enable or disable reversing detection
    pub fn set_reversing_detection_enabled(&mut self, enabled: bool) {
        self.reversing_detection_enabled = if enabled { 1 } else { 0 };
    }

    /// Check if motion analysis is enabled
    pub fn is_motion_analysis_enabled(&self) -> bool {
        self.motion_analysis_enabled != 0
    }

    /// Enable or disable motion analysis
    pub fn set_motion_analysis_enabled(&mut self, enabled: bool) {
        self.motion_analysis_enabled = if enabled { 1 } else { 0 };
    }

    /// Get a summary of enabled features
    pub fn get_enabled_features(&self) -> Vec<String> {
        let mut features = Vec::new();

        if let Some(vehicle_type) = self.get_vehicle_type() {
            features.push(format!("Vehicle Type: {}", vehicle_type));
        }

        if self.is_internal_gnss_enabled() {
            features.push("Internal GNSS".to_string());
        }
        if self.is_atmospheric_altitude_enabled() {
            features.push("Atmospheric Altitude".to_string());
        }
        if self.is_velocity_heading_enabled() {
            features.push("Velocity Heading".to_string());
        }
        if self.is_reversing_detection_enabled() {
            features.push("Reversing Detection".to_string());
        }
        if self.is_motion_analysis_enabled() {
            features.push("Motion Analysis".to_string());
        }

        features
    }

    /// Validate the packet configuration
    pub fn validate(&self) -> Result<()> {
        if self.get_vehicle_type().is_none() {
            return Err(AnError::ValidationFailed(format!(
                "Invalid vehicle type: {}",
                self.vehicle_type
            )));
        }

        // Validate reserved fields are zero
        if self.reserved_1 != 0 {
            return Err(AnError::ValidationFailed(
                "Reserved field 1 must be zero".to_string(),
            ));
        }
        if self.reserved_2 != 0 {
            return Err(AnError::ValidationFailed(
                "Reserved field 2 must be zero".to_string(),
            ));
        }
        if self.reserved_3 != [0; 8] {
            return Err(AnError::ValidationFailed(
                "Reserved field 3 must be all zeros".to_string(),
            ));
        }

        Ok(())
    }
}

/// Odometer configuration packet (Packet ID 192, Length 8) - Read/Write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometerConfigurationPacket {
    pub permanent: u8,
    pub automatic_pulse_measurement_active: u8,
    pub reserved: [u8; 2],
    pub pulse_length: f32,
}

impl AnppPacket for OdometerConfigurationPacket {}

impl OdometerConfigurationPacket {
    /// Create a new odometer configuration packet
    pub fn new(permanent: bool, pulse_length_meters: f32, auto_measurement: bool) -> Result<Self> {
        if pulse_length_meters <= 0.0 {
            return Err(AnError::ValidationFailed(
                "Pulse length must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            permanent: if permanent { 1 } else { 0 },
            automatic_pulse_measurement_active: if auto_measurement { 1 } else { 0 },
            reserved: [0; 2],
            pulse_length: pulse_length_meters,
        })
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Set permanent flag
    pub fn set_permanent(&mut self, permanent: bool) {
        self.permanent = if permanent { 1 } else { 0 };
    }

    /// Check if automatic pulse measurement is active
    pub fn is_automatic_pulse_measurement_active(&self) -> bool {
        self.automatic_pulse_measurement_active != 0
    }

    /// Enable or disable automatic pulse measurement
    pub fn set_automatic_pulse_measurement_active(&mut self, active: bool) {
        self.automatic_pulse_measurement_active = if active { 1 } else { 0 };
    }

    /// Get pulse length in meters
    pub fn get_pulse_length_meters(&self) -> f32 {
        self.pulse_length
    }

    /// Set pulse length in meters
    pub fn set_pulse_length_meters(&mut self, length_meters: f32) -> Result<()> {
        if length_meters <= 0.0 {
            return Err(AnError::ValidationFailed(
                "Pulse length must be greater than zero".to_string(),
            ));
        }
        self.pulse_length = length_meters;
        Ok(())
    }

    /// Get pulse length in millimeters
    pub fn get_pulse_length_millimeters(&self) -> f32 {
        self.pulse_length * 1000.0
    }

    /// Set pulse length from millimeters
    pub fn set_pulse_length_millimeters(&mut self, length_mm: f32) -> Result<()> {
        self.set_pulse_length_meters(length_mm / 1000.0)
    }

    /// Get pulse length in centimeters
    pub fn get_pulse_length_centimeters(&self) -> f32 {
        self.pulse_length * 100.0
    }

    /// Set pulse length from centimeters
    pub fn set_pulse_length_centimeters(&mut self, length_cm: f32) -> Result<()> {
        self.set_pulse_length_meters(length_cm / 100.0)
    }

    /// Validate the packet configuration
    pub fn validate(&self) -> Result<()> {
        if self.pulse_length <= 0.0 {
            return Err(AnError::ValidationFailed(
                "Pulse length must be greater than zero".to_string(),
            ));
        }

        // Validate reserved fields are zero
        if self.reserved != [0; 2] {
            return Err(AnError::ValidationFailed(
                "Reserved fields must be zero".to_string(),
            ));
        }

        Ok(())
    }

    /// Get a human-readable summary of the configuration
    pub fn get_summary(&self) -> String {
        format!(
            "Odometer Config: {:.3}m pulse length, auto measurement: {}, permanent: {}",
            self.pulse_length,
            if self.is_automatic_pulse_measurement_active() {
                "enabled"
            } else {
                "disabled"
            },
            if self.is_permanent() { "yes" } else { "no" }
        )
    }
}

/// Set zero orientation alignment packet (Packet ID 193, Length 5) - Write only
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetZeroOrientationAlignmentPacket {
    pub permanent: u8,
    pub verification: u32, // Verification sequence (must be 0x9A4E8055)
}

impl AnppPacket for SetZeroOrientationAlignmentPacket {}

impl SetZeroOrientationAlignmentPacket {
    /// Create a new set zero orientation alignment packet with the correct verification sequence
    pub fn new(permanent: bool) -> Self {
        Self {
            permanent: if permanent { 1 } else { 0 },
            verification: 0x9A4E8055,
        }
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Set permanent flag
    pub fn set_permanent(&mut self, permanent: bool) {
        self.permanent = if permanent { 1 } else { 0 };
    }

    /// Validate the packet has the correct verification sequence
    pub fn validate(&self) -> Result<()> {
        if self.verification != 0x9A4E8055 {
            return Err(AnError::ValidationFailed(format!(
                "Invalid verification sequence: expected 0x9A4E8055, got 0x{:08X}",
                self.verification
            )));
        }
        Ok(())
    }

    /// Get a human-readable summary of the packet
    pub fn get_summary(&self) -> String {
        format!(
            "Set Zero Orientation: permanent: {}, verification: 0x{:08X}",
            if self.is_permanent() { "yes" } else { "no" },
            self.verification
        )
    }
}

impl Default for SetZeroOrientationAlignmentPacket {
    fn default() -> Self {
        Self::new(false)
    }
}

/// 3D offset vector in body coordinate frame
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OffsetVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl OffsetVector {
    /// Create a new offset vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a zero offset vector
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Check if this is a zero offset
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    /// Get magnitude of the offset vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Default for OffsetVector {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for OffsetVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})m", self.x, self.y, self.z)
    }
}

/// Reference point offsets packet (Packet ID 194, Length 49) - Read/Write
///
/// Used to adjust the measurement point that all data is referenced to.
/// By default all values are zero and the measurement point is the centre of the Boreas unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferencePointOffsetsPacket {
    pub permanent: u8,
    pub heave_point_1_offset_x: f32, // Primary reference point offset X (m)
    pub heave_point_1_offset_y: f32, // Primary reference point offset Y (m)
    pub heave_point_1_offset_z: f32, // Primary reference point offset Z (m)
    pub heave_point_2_offset_x: f32, // Used for COG Lever Arm Offset X (m)
    pub heave_point_2_offset_y: f32, // Used for COG Lever Arm Offset Y (m)
    pub heave_point_2_offset_z: f32, // Used for COG Lever Arm Offset Z (m)
    pub heave_point_3_offset_x: f32, // Heave point 3 offset X (m)
    pub heave_point_3_offset_y: f32, // Heave point 3 offset Y (m)
    pub heave_point_3_offset_z: f32, // Heave point 3 offset Z (m)
    pub heave_point_4_offset_x: f32, // Heave point 4 offset X (m)
    pub heave_point_4_offset_y: f32, // Heave point 4 offset Y (m)
    pub heave_point_4_offset_z: f32, // Heave point 4 offset Z (m)
}

impl AnppPacket for ReferencePointOffsetsPacket {}

impl ReferencePointOffsetsPacket {
    /// Create a new reference point offsets packet with all offsets at zero
    pub fn new(permanent: bool) -> Self {
        Self {
            permanent: if permanent { 1 } else { 0 },
            heave_point_1_offset_x: 0.0,
            heave_point_1_offset_y: 0.0,
            heave_point_1_offset_z: 0.0,
            heave_point_2_offset_x: 0.0,
            heave_point_2_offset_y: 0.0,
            heave_point_2_offset_z: 0.0,
            heave_point_3_offset_x: 0.0,
            heave_point_3_offset_y: 0.0,
            heave_point_3_offset_z: 0.0,
            heave_point_4_offset_x: 0.0,
            heave_point_4_offset_y: 0.0,
            heave_point_4_offset_z: 0.0,
        }
    }

    /// Check if permanent setting is enabled
    pub fn is_permanent(&self) -> bool {
        self.permanent != 0
    }

    /// Set permanent flag
    pub fn set_permanent(&mut self, permanent: bool) {
        self.permanent = if permanent { 1 } else { 0 };
    }

    /// Get the primary reference point offset (heave point 1)
    /// This affects all data output from the device
    pub fn get_primary_reference_offset(&self) -> OffsetVector {
        OffsetVector::new(
            self.heave_point_1_offset_x,
            self.heave_point_1_offset_y,
            self.heave_point_1_offset_z,
        )
    }

    /// Set the primary reference point offset (heave point 1)
    pub fn set_primary_reference_offset(&mut self, offset: OffsetVector) {
        self.heave_point_1_offset_x = offset.x;
        self.heave_point_1_offset_y = offset.y;
        self.heave_point_1_offset_z = offset.z;
    }

    /// Get the COG (Center of Gravity) lever arm offset (heave point 2)
    /// This is used for heading alignment
    pub fn get_cog_lever_arm_offset(&self) -> OffsetVector {
        OffsetVector::new(
            self.heave_point_2_offset_x,
            self.heave_point_2_offset_y,
            self.heave_point_2_offset_z,
        )
    }

    /// Set the COG lever arm offset (heave point 2)
    pub fn set_cog_lever_arm_offset(&mut self, offset: OffsetVector) {
        self.heave_point_2_offset_x = offset.x;
        self.heave_point_2_offset_y = offset.y;
        self.heave_point_2_offset_z = offset.z;
    }

    /// Get heave point 3 offset
    pub fn get_heave_point_3_offset(&self) -> OffsetVector {
        OffsetVector::new(
            self.heave_point_3_offset_x,
            self.heave_point_3_offset_y,
            self.heave_point_3_offset_z,
        )
    }

    /// Set heave point 3 offset
    pub fn set_heave_point_3_offset(&mut self, offset: OffsetVector) {
        self.heave_point_3_offset_x = offset.x;
        self.heave_point_3_offset_y = offset.y;
        self.heave_point_3_offset_z = offset.z;
    }

    /// Get heave point 4 offset
    pub fn get_heave_point_4_offset(&self) -> OffsetVector {
        OffsetVector::new(
            self.heave_point_4_offset_x,
            self.heave_point_4_offset_y,
            self.heave_point_4_offset_z,
        )
    }

    /// Set heave point 4 offset
    pub fn set_heave_point_4_offset(&mut self, offset: OffsetVector) {
        self.heave_point_4_offset_x = offset.x;
        self.heave_point_4_offset_y = offset.y;
        self.heave_point_4_offset_z = offset.z;
    }

    /// Check if this is using default configuration (all offsets zero)
    pub fn is_default_configuration(&self) -> bool {
        self.get_primary_reference_offset().is_zero()
            && self.get_cog_lever_arm_offset().is_zero()
            && self.get_heave_point_3_offset().is_zero()
            && self.get_heave_point_4_offset().is_zero()
    }

    /// Get a summary of all configured offsets
    pub fn get_summary(&self) -> String {
        if self.is_default_configuration() {
            format!(
                "All reference point offsets at default (0,0,0), permanent: {}",
                if self.is_permanent() { "yes" } else { "no" }
            )
        } else {
            let mut active_offsets = Vec::new();

            if !self.get_primary_reference_offset().is_zero() {
                active_offsets.push(format!("Primary: {}", self.get_primary_reference_offset()));
            }
            if !self.get_cog_lever_arm_offset().is_zero() {
                active_offsets.push(format!("COG: {}", self.get_cog_lever_arm_offset()));
            }
            if !self.get_heave_point_3_offset().is_zero() {
                active_offsets.push(format!("Heave3: {}", self.get_heave_point_3_offset()));
            }
            if !self.get_heave_point_4_offset().is_zero() {
                active_offsets.push(format!("Heave4: {}", self.get_heave_point_4_offset()));
            }

            format!(
                "Reference offsets: {} (permanent: {})",
                active_offsets.join(", "),
                if self.is_permanent() { "yes" } else { "no" }
            )
        }
    }
}

impl Default for ReferencePointOffsetsPacket {
    fn default() -> Self {
        Self::new(false)
    }
}

/// IP dataport mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum IpDataportMode {
    ModeNone = 0,
    Reserved = 1,
    ModeTcpServer = 2,
    ModeTcpClient = 3,
    ModeUdp = 4,
}

impl IpDataportMode {
    /// Convert a u8 value to an IpDataportMode
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::ModeNone),
            1 => Some(Self::Reserved),
            2 => Some(Self::ModeTcpServer),
            3 => Some(Self::ModeTcpClient),
            4 => Some(Self::ModeUdp),
            _ => None,
        }
    }

    /// Get the u8 value of the IpDataportMode
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get a human-readable description of the IP dataport mode
    pub fn description(&self) -> &'static str {
        match self {
            Self::ModeNone => "MODE_NONE",
            Self::Reserved => "Reserved",
            Self::ModeTcpServer => "MODE_TCP_SERVER",
            Self::ModeTcpClient => "MODE_TCP_CLIENT",
            Self::ModeUdp => "MODE_UDP",
        }
    }
}

impl std::fmt::Display for IpDataportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Individual IP dataport configuration (7 bytes each)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportEntry {
    pub tcp_udp_ip_address: u32, // TCP/UDP IP Address (little-endian)
    pub tcp_udp_port: u16,       // TCP/UDP Port
    pub ip_dataport_mode: u8,    // IP Dataport Mode (0-4)
}

impl IpDataportEntry {
    /// Create a new IP dataport entry
    pub fn new(ip_address: std::net::Ipv4Addr, port: u16, mode: IpDataportMode) -> Self {
        Self {
            tcp_udp_ip_address: u32::from_le_bytes(ip_address.octets()),
            tcp_udp_port: port,
            ip_dataport_mode: mode.as_u8(),
        }
    }

    /// Create an inactive IP dataport entry
    pub fn inactive() -> Self {
        Self::new(std::net::Ipv4Addr::UNSPECIFIED, 0, IpDataportMode::ModeNone)
    }

    /// Get the IP address as a readable format
    pub fn get_ip_address(&self) -> std::net::Ipv4Addr {
        std::net::Ipv4Addr::from(self.tcp_udp_ip_address.to_le_bytes())
    }

    /// Set the IP address
    pub fn set_ip_address(&mut self, addr: std::net::Ipv4Addr) {
        self.tcp_udp_ip_address = u32::from_le_bytes(addr.octets());
    }

    /// Get the port number
    pub fn get_port(&self) -> u16 {
        self.tcp_udp_port
    }

    /// Set the port number
    pub fn set_port(&mut self, port: u16) {
        self.tcp_udp_port = port;
    }

    /// Get the IP dataport mode as an enum
    pub fn get_mode(&self) -> Option<IpDataportMode> {
        IpDataportMode::from_u8(self.ip_dataport_mode)
    }

    /// Set the IP dataport mode
    pub fn set_mode(&mut self, mode: IpDataportMode) {
        self.ip_dataport_mode = mode.as_u8();
    }

    /// Check if this dataport is active (not inactive mode)
    pub fn is_active(&self) -> bool {
        self.ip_dataport_mode != IpDataportMode::ModeNone.as_u8()
    }

    /// Validate the dataport entry
    pub fn validate(&self) -> Result<()> {
        if self.get_mode().is_none() {
            return Err(AnError::ValidationFailed(format!(
                "Invalid IP dataport mode: {}",
                self.ip_dataport_mode
            )));
        }

        Ok(())
    }
}

impl Default for IpDataportEntry {
    fn default() -> Self {
        Self::inactive()
    }
}

impl std::fmt::Display for IpDataportEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(mode) = self.get_mode() {
            if mode == IpDataportMode::ModeNone {
                write!(f, "MODE_NONE")
            } else {
                write!(
                    f,
                    "{} {}:{}",
                    mode,
                    self.get_ip_address(),
                    self.tcp_udp_port
                )
            }
        } else {
            write!(f, "Invalid mode ({})", self.ip_dataport_mode)
        }
    }
}

/// IP dataports configuration packet (Packet ID 202, Length 30) - Read/Write
///
/// Configures up to 4 IP data ports for TCP/UDP communication.
/// Each data port can be configured as TCP Server, TCP Client, UDP Server, UDP Client, or Inactive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpDataportsConfigurationPacket {
    pub reserved: u16,               // Reserved field (bytes 0-1)
    pub dataport_1: IpDataportEntry, // IP Dataport 1 configuration (bytes 2-8)
    pub dataport_2: IpDataportEntry, // IP Dataport 2 configuration (bytes 9-15)
    pub dataport_3: IpDataportEntry, // IP Dataport 3 configuration (bytes 16-22)
    pub dataport_4: IpDataportEntry, // IP Dataport 4 configuration (bytes 23-29)
}

impl AnppPacket for IpDataportsConfigurationPacket {}

impl IpDataportsConfigurationPacket {
    /// Create a new IP dataports configuration packet with all ports inactive
    pub fn new() -> Self {
        Self {
            reserved: 0,
            dataport_1: IpDataportEntry::inactive(),
            dataport_2: IpDataportEntry::inactive(),
            dataport_3: IpDataportEntry::inactive(),
            dataport_4: IpDataportEntry::inactive(),
        }
    }

    /// Get dataport entry by index (0-3)
    pub fn get_dataport(&self, index: usize) -> Option<&IpDataportEntry> {
        match index {
            0 => Some(&self.dataport_1),
            1 => Some(&self.dataport_2),
            2 => Some(&self.dataport_3),
            3 => Some(&self.dataport_4),
            _ => None,
        }
    }

    /// Get mutable dataport entry by index (0-3)
    pub fn get_dataport_mut(&mut self, index: usize) -> Option<&mut IpDataportEntry> {
        match index {
            0 => Some(&mut self.dataport_1),
            1 => Some(&mut self.dataport_2),
            2 => Some(&mut self.dataport_3),
            3 => Some(&mut self.dataport_4),
            _ => None,
        }
    }

    /// Set dataport configuration by index (0-3)
    pub fn set_dataport(&mut self, index: usize, dataport: IpDataportEntry) -> Result<()> {
        match index {
            0 => self.dataport_1 = dataport,
            1 => self.dataport_2 = dataport,
            2 => self.dataport_3 = dataport,
            3 => self.dataport_4 = dataport,
            _ => {
                return Err(AnError::ValidationFailed(
                    "Dataport index must be 0-3".to_string(),
                ))
            }
        }
        Ok(())
    }

    /// Configure a dataport as TCP server
    pub fn set_tcp_server(&mut self, index: usize, port: u16) -> Result<()> {
        let dataport = IpDataportEntry::new(
            std::net::Ipv4Addr::UNSPECIFIED,
            port,
            IpDataportMode::ModeTcpServer,
        );
        self.set_dataport(index, dataport)
    }

    /// Configure a dataport as TCP client
    pub fn set_tcp_client(
        &mut self,
        index: usize,
        ip_address: std::net::Ipv4Addr,
        port: u16,
    ) -> Result<()> {
        let dataport = IpDataportEntry::new(ip_address, port, IpDataportMode::ModeTcpClient);
        self.set_dataport(index, dataport)
    }

    /// Configure a dataport as UDP
    pub fn set_udp(
        &mut self,
        index: usize,
        ip_address: std::net::Ipv4Addr,
        port: u16,
    ) -> Result<()> {
        let dataport = IpDataportEntry::new(ip_address, port, IpDataportMode::ModeUdp);
        self.set_dataport(index, dataport)
    }

    /// Deactivate a dataport
    pub fn set_inactive(&mut self, index: usize) -> Result<()> {
        let dataport = IpDataportEntry::inactive();
        self.set_dataport(index, dataport)
    }

    /// Get all active dataports
    pub fn get_active_dataports(&self) -> Vec<(usize, &IpDataportEntry)> {
        [
            (0, &self.dataport_1),
            (1, &self.dataport_2),
            (2, &self.dataport_3),
            (3, &self.dataport_4),
        ]
        .iter()
        .filter(|(_, entry)| entry.is_active())
        .map(|(i, entry)| (*i, *entry))
        .collect()
    }

    /// Get count of active dataports
    pub fn get_active_count(&self) -> usize {
        self.get_active_dataports().len()
    }

    /// Check if any dataports are configured
    pub fn has_active_dataports(&self) -> bool {
        self.get_active_count() > 0
    }

    /// Validate the entire packet configuration
    pub fn validate(&self) -> Result<()> {
        // Validate each dataport
        self.dataport_1.validate()?;
        self.dataport_2.validate()?;
        self.dataport_3.validate()?;
        self.dataport_4.validate()?;

        // Validate reserved field
        if self.reserved != 0 {
            return Err(AnError::ValidationFailed(
                "Reserved field must be zero".to_string(),
            ));
        }

        // Check for port conflicts on the same modes
        let mut server_ports = std::collections::HashSet::new();

        for (_index, dataport) in [
            (1, &self.dataport_1),
            (2, &self.dataport_2),
            (3, &self.dataport_3),
            (4, &self.dataport_4),
        ] {
            if let Some(mode) = dataport.get_mode() {
                match mode {
                    IpDataportMode::ModeTcpServer => {
                        if dataport.tcp_udp_port != 0
                            && !server_ports.insert((mode, dataport.tcp_udp_port))
                        {
                            return Err(AnError::ValidationFailed(format!(
                                "Port {} is already used by another {} dataport",
                                dataport.tcp_udp_port, mode
                            )));
                        }
                    }
                    IpDataportMode::Reserved => {
                        return Err(AnError::ValidationFailed(
                            "Reserved IP dataport mode cannot be used".to_string(),
                        ));
                    }
                    _ => {} // Client modes, UDP, and ModeNone don't conflict on ports
                }
            }
        }

        Ok(())
    }

    /// Get a summary of all dataport configurations
    pub fn get_summary(&self) -> String {
        let active_dataports = self.get_active_dataports();

        if active_dataports.is_empty() {
            "All IP dataports inactive".to_string()
        } else {
            let summaries: Vec<String> = active_dataports
                .iter()
                .map(|(index, entry)| format!("Port {}: {}", index + 1, entry))
                .collect();

            format!(
                "IP Dataports ({}): {}",
                active_dataports.len(),
                summaries.join(", ")
            )
        }
    }
}

impl Default for IpDataportsConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}
