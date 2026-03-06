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
pub use packet::{Packet, PacketKind, HasPacketId};
pub use parser::{AnppParser, parse_datagram, DatagramError};

// Re-export all public types from packet modules
pub use packet::system::{
    Acknowledge, AcknowledgeResult, Request, BootMode, DeviceInformation, DeviceType,
    RestoreFactorySettings, Reset, IpConfiguration,
};

pub use packet::state::{
    SystemStatus, FilterStatus, GnssFixType, SystemState, UnixTime, Status,
    PositionStdDev, VelocityStdDev,
    EulerOrientationStdDev, QuaternionOrientationStdDev,
    RawSensors, RawGnss, RawGnssStatus,
    Satellites,
    GeodeticPosition, EcefPosition, UtmPosition, NedVelocity, BodyVelocity,
    Acceleration, BodyAcceleration, EulerOrientation, QuaternionOrientation,
    DcmOrientation, AngularVelocity, AngularAcceleration,
    ExternalPositionVelocity, ExternalPosition, ExternalVelocity,
    ExternalBodyVelocity, ExternalHeading,
    RunningTime, ExternalTime, GeoidHeight, RtcmCorrections,
    Heave, RawDvlData, DvlStatus,
    GnssReceiverInformation, GnssManufacturer, GnssReceiverModel,
    SensorTemperature,
    GnssPositionVelocityTime, GnssOrientation,
    GnssPvtStatus, GnssOrientationStatus, SpoofingStatus, InterferenceStatus,
};

pub use packet::config::{
    PacketPeriod, PacketTimerPeriod, PacketsPeriod, OffsetVector,
    InstallationAlignment, VehicleType, FilterOptions, OdometerConfiguration,
    SetZeroOrientationAlignment, ReferencePointOffsets, UserData,
    IpDataportMode, IpDataport, IpDataportsConfiguration,
};
