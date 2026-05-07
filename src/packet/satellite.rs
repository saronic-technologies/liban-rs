use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};

/// Satellite navigation system
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SatelliteSystem {
    #[default]
    Unknown = 0,
    Gps = 1,
    Glonass = 2,
    BeiDou = 3,
    Galileo = 4,
    Sbas = 5,
    Qzss = 6,
    Omnistar = 8,
    NavIc = 10,
}

impl From<u8> for SatelliteSystem {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Gps,
            2 => Self::Glonass,
            3 => Self::BeiDou,
            4 => Self::Galileo,
            5 => Self::Sbas,
            6 => Self::Qzss,
            8 => Self::Omnistar,
            10 => Self::NavIc,
            _ => Self::Unknown,
        }
    }
}

/// Satellite frequency tracking status bitfield
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct TrackingStatus(u8);

impl TrackingStatus {
    pub fn raw(&self) -> u8 { self.0 }
    pub fn is_valid(&self) -> bool { self.0 == 0 }
    pub fn carrier_phase_valid(&self) -> bool { self.0 & (1 << 0) != 0 }
    pub fn carrier_phase_cycle_slip(&self) -> bool { self.0 & (1 << 1) != 0 }
    pub fn carrier_phase_half_cycle_ambiguity(&self) -> bool { self.0 & (1 << 2) != 0 }
    pub fn pseudo_range_valid(&self) -> bool { self.0 & (1 << 3) != 0 }
    pub fn doppler_valid(&self) -> bool { self.0 & (1 << 4) != 0 }
    pub fn snr_valid(&self) -> bool { self.0 & (1 << 5) != 0 }
}

impl From<u8> for TrackingStatus {
    fn from(v: u8) -> Self { Self(v) }
}

/// Per-frequency measurement within a RawSatelliteEntry
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct RawSatelliteFrequency {
    /// Satellite frequency code (system-dependent)
    pub frequency: u8,
    /// Tracking status flags for carrier/pseudo range/doppler validity
    pub tracking_status: TrackingStatus,
    /// Carrier phase (cycles)
    pub carrier_phase: f64,
    /// Pseudo range (m)
    pub pseudo_range: f64,
    /// Doppler frequency (Hz)
    pub doppler_frequency: f32,
    /// Signal to noise ratio (dB-Hz)
    pub snr: f32,
}

/// Per-satellite entry within RawSatelliteData
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSatelliteEntry {
    /// Satellite system
    #[br(map = |x: u8| SatelliteSystem::from(x))]
    #[bw(map = |x: &SatelliteSystem| *x as u8)]
    pub satellite_system: SatelliteSystem,
    /// PRN or satellite number
    pub prn: u8,
    /// Elevation (deg)
    pub elevation: u8,
    /// Azimuth (deg)
    pub azimuth: u16,
    #[br(temp)]
    #[bw(calc = frequencies.len() as u8)]
    num_frequencies: u8,
    /// Per-frequency measurements
    #[br(count = num_frequencies)]
    pub frequencies: Vec<RawSatelliteFrequency>,
}

/// GPS satellite ephemeris data
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct GpsEphemeris {
    /// Time of ephemeris (s)
    pub toe: u32,
    /// Issue of Data Clock (s)
    pub iodc: u16,
    /// Issue of Data Ephemeris (s)
    pub iode: u16,
    /// Satellite clock bias af0 (s)
    pub af0: f32,
    /// Satellite clock drift af1 (s/s)
    pub af1: f32,
    /// Satellite clock drift rate af2 (s/s/s)
    pub af2: f32,
    /// Crs (m)
    pub crs: f32,
    /// Delta N (rad/s)
    pub delta_n: f32,
    /// M0 (rad)
    pub m0: f64,
    /// Cuc (rad)
    pub cuc: f32,
    /// Eccentricity
    pub eccentricity: f64,
    /// Cus (rad)
    pub cus: f32,
    /// Square root of semi-major axis (sqrt(m))
    pub sqrt_semi_major_axis: f64,
    /// Cic (rad)
    pub cic: f32,
    /// OMEGA0 (rad)
    pub omega0: f64,
    /// Cis (rad)
    pub cis: f32,
    /// i0 (rad)
    pub i0: f64,
    /// Crc (m)
    pub crc: f32,
    /// Omega (rad)
    pub omega: f64,
    /// Omega dot (rad/s)
    pub omega_dot: f64,
    /// IDOT (rad/s)
    pub idot: f64,
    /// TGD (s)
    pub tgd: f32,
    /// Ephemeris week number
    pub week: u16,
    /// Transmission time (s)
    pub transmission_time: u32,
    /// User range accuracy (m)
    pub ura: u16,
    /// GPS status flags
    pub status: u16,
}

/// GLONASS satellite ephemeris data
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlonassEphemeris {
    /// Satellite clock bias (s)
    pub clock_bias: f32,
    /// Satellite frequency bias gamma
    pub frequency_bias: f32,
    /// Satellite position X (m)
    pub x_position: f64,
    /// Satellite position Y (m)
    pub y_position: f64,
    /// Satellite position Z (m)
    pub z_position: f64,
    /// Satellite velocity X (m/s)
    pub x_velocity: f64,
    /// Satellite velocity Y (m/s)
    pub y_velocity: f64,
    /// Satellite velocity Z (m/s)
    pub z_velocity: f64,
    /// Satellite acceleration X (m/s/s)
    pub x_acceleration: f64,
    /// Satellite acceleration Y (m/s/s)
    pub y_acceleration: f64,
    /// Satellite acceleration Z (m/s/s)
    pub z_acceleration: f64,
    /// Message frame start time (s)
    pub frame_start_time: u32,
    /// Age of operational information (days)
    pub age: u8,
    /// Frequency slot number
    pub frequency_slot: i8,
    /// Satellite health
    pub health: u8,
    #[br(temp)]
    #[bw(calc = 0u8)]
    _reserved: u8,
}

/// System-specific ephemeris data within RawSatelliteEphemeris
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[br(import(system: SatelliteSystem))]
pub enum EphemerisData {
    #[br(pre_assert(matches!(system, SatelliteSystem::Gps | SatelliteSystem::Qzss)))]
    Gps(GpsEphemeris),
    #[br(pre_assert(system == SatelliteSystem::Glonass))]
    Glonass(GlonassEphemeris),
    Unknown,
}

/// Per-satellite entry within ExtendedSatellites
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ExtendedSatelliteEntry {
    /// Satellite system
    #[br(map = |x: u8| SatelliteSystem::from(x))]
    #[bw(map = |x: &SatelliteSystem| *x as u8)]
    pub satellite_system: SatelliteSystem,
    /// Satellite number (PRN)
    pub prn: u8,
    /// Satellite frequencies indicator
    pub frequencies: u8,
    /// Elevation (deg)
    pub elevation: u8,
    /// Azimuth (deg)
    pub azimuth: u16,
    /// SNR for receiver 1
    pub snr_receiver_1: u8,
    /// SNR for receiver 2
    pub snr_receiver_2: u8,
    /// Status flags
    pub status: u8,
}
