use binrw::binrw;
use bitflags::bitflags;

bitflags! {
    /// System status bitmask for SystemStatePacket
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[binrw]
    #[br(map = |x: u16| SystemStatusFlags::from_bits_truncate(x))]
    #[bw(map = |x: &SystemStatusFlags| x.bits())]
    pub struct SystemStatusFlags: u16 {
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
    /// Filter status bitmask for SystemStatePacket
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[binrw]
    #[br(map = |x: u16| FilterStatusFlags::from_bits_truncate(x))]
    #[bw(map = |x: &FilterStatusFlags| x.bits())]
    pub struct FilterStatusFlags: u16 {
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

impl FilterStatusFlags {
    /// Get the GNSS fix type from the filter status
    pub fn gnss_fix_type(self) -> u8 {
        ((self.bits() & Self::GNSS_FIX_TYPE_MASK.bits()) >> 4) as u8
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_status_bitflags() {
        // Test individual flags and combinations
        let filter_status = FilterStatusFlags::NAVIGATION_FILTER_INITIALISED
                          | FilterStatusFlags::UTC_TIME_INITIALISED
                          | FilterStatusFlags::INTERNAL_GNSS_ENABLED;

        assert!(filter_status.contains(FilterStatusFlags::NAVIGATION_FILTER_INITIALISED));
        assert!(filter_status.contains(FilterStatusFlags::UTC_TIME_INITIALISED));
        assert!(filter_status.contains(FilterStatusFlags::INTERNAL_GNSS_ENABLED));
        assert!(!filter_status.contains(FilterStatusFlags::HEADING_INITIALISED));

        // Test GNSS fix type extraction (bits 4-6)
        let filter_with_gnss_fix = FilterStatusFlags::from_bits_truncate(0b0110000); // Fix type 3
        assert_eq!(filter_with_gnss_fix.gnss_fix_type(), 3);

        let filter_with_different_fix = FilterStatusFlags::from_bits_truncate(0b0010000); // Fix type 1
        assert_eq!(filter_with_different_fix.gnss_fix_type(), 1);
    }

    #[test]
    fn test_bitflags_operations() {
        let status1 = SystemStatusFlags::SYSTEM_FAILURE | SystemStatusFlags::GNSS_FAILURE;
        let status2 = SystemStatusFlags::GNSS_FAILURE | SystemStatusFlags::LOW_VOLTAGE_ALARM;

        // Test union (|)
        let union = status1 | status2;
        assert!(union.contains(SystemStatusFlags::SYSTEM_FAILURE));
        assert!(union.contains(SystemStatusFlags::GNSS_FAILURE));
        assert!(union.contains(SystemStatusFlags::LOW_VOLTAGE_ALARM));

        // Test intersection (&)
        let intersection = status1 & status2;
        assert!(!intersection.contains(SystemStatusFlags::SYSTEM_FAILURE));
        assert!(intersection.contains(SystemStatusFlags::GNSS_FAILURE));
        assert!(!intersection.contains(SystemStatusFlags::LOW_VOLTAGE_ALARM));

        // Test difference (-)
        let difference = status1 - status2;
        assert!(difference.contains(SystemStatusFlags::SYSTEM_FAILURE));
        assert!(!difference.contains(SystemStatusFlags::GNSS_FAILURE));
        assert!(!difference.contains(SystemStatusFlags::LOW_VOLTAGE_ALARM));
    }
}