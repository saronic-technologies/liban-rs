//! # liban - Advanced Navigation Packet Protocol Library (Sans-IO)
//!
//! A sans-io Rust library for working with Advanced Navigation devices using the
//! Advanced Navigation Packet Protocol (ANPP).

pub mod error;
pub mod packet;
pub mod parser;
pub mod protocol;
pub mod reader;

pub use error::{AnError, Result};
pub use packet::{HasPacketId, Packet, PacketKind};
pub use parser::{AnppParser, DatagramError, parse_datagram};

// Re-export all public types from packet modules
pub use packet::system::{
    Acknowledge, AcknowledgeResult, BootMode, DeviceInformation, DeviceType, IpConfiguration,
    Request, Reset, ResetMode, RestoreFactorySettings,
};

pub use packet::state::{
    Acceleration, AirDataFlags, AngularAcceleration, AngularVelocity, Automotive, BodyAcceleration,
    BodyVelocity, DcmOrientation, DvlStatus, EcefPosition, EulerOrientation,
    EulerOrientationStdDev, ExtendedSatellites, ExternalAirData, ExternalBodyVelocity,
    ExternalDepth, ExternalHeading, ExternalOdometer, ExternalPosition, ExternalPositionVelocity,
    ExternalTime, ExternalVelocity, FilterStatus, FormattedTime, GeodeticPosition, GeoidHeight,
    GimbalState, GnssFixType, GnssOrientation, GnssOrientationStatus, GnssPositionVelocityTime,
    GnssPvtStatus, GnssReceiverInformation, Heave, InterferenceStatus, NedVelocity,
    NorthSeekingFlags, NorthSeekingInitialisationStatus, OdometerState, PositionStdDev,
    QuaternionOrientation, QuaternionOrientationStdDev, RawDvlData, RawGnss, RawGnssStatus,
    RawSatelliteData, RawSatelliteEphemeris, RawSensors, RtcmCorrections, RunningTime, Satellites,
    SensorTemperature, SpoofingStatus, Status, SystemState, SystemStatus, SystemTemperature,
    UnixTime, UtmPosition, VelocityStdDev, VesselMotion, Wind, ZeroAngularVelocity,
};

pub use packet::receiver::{
    AdvancedNavigationModel, GenericReceiverData, GnssManufacturer, GnssReceiverData,
    OmnistarEngineMode, ReceiverType, RtkLicenseAccuracy, TrimbleBd992ReceiverData, TrimbleModel,
    UBloxModel,
};

pub use packet::config::{
    FilterOptions, InstallationAlignment, IpDataport, IpDataportMode, IpDataportsConfiguration,
    OdometerConfiguration, OffsetVector, PacketPeriod, PacketTimerPeriod, PacketsPeriod,
    ReferencePointOffsets, SetZeroOrientationAlignment, UserData, VehicleType,
};
