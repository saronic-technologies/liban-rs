//! # liban - Advanced Navigation Packet Protocol Library
//!
//! A Rust library for communicating with Advanced Navigation devices using the
//! Advanced Navigation Packet Protocol (ANPP), with specific support for the Boreas D90.
//!
//! ## Features
//!
//! - Full ANPP protocol implementation with CRC16-CCITT validation
//! - Asynchronous TCP communication
//! - Type-safe packet handling
//! - Comprehensive error handling
//! - Built-in timeout and connection management
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use liban::{BoreasInterface, PacketId};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut interface = BoreasInterface::new(
//!         "192.168.1.100",
//!         16718,
//!         Duration::from_secs(5)
//!     ).await?;
//!
//!     let device_info = interface.get_device_information().await?;
//!     println!("Device ID: {}", device_info.device_id);
//!
//!     interface.disconnect().await?;
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod interface;
pub mod packet;
pub mod protocol;

pub use error::{AnError, Result};
pub use interface::BoreasInterface;
pub use packet::{
    AcknowledgePacket,
    // Core traits
    AnppPacket,
    BootModePacket,

    DeviceInformation,
    FilterOptionsPacket,
    FilterStatus,
    FormattedTimePacket,

    InstallationAlignmentPacket,
    IpConfigurationPacket,
    IpDataportEntry,
    IpDataportMode,
    IpDataportsConfigurationPacket,
    OdometerConfigurationPacket,
    OffsetVector,
    // Core types
    PacketId,
    PacketPeriodEntry,
    // Configuration packets (180-203)
    PacketTimerPeriodPacket,
    PacketsPeriodPacket,
    ReferencePointOffsetsPacket,
    // System packets (0-14)
    RequestPacket,
    ResetPacket,
    RestoreFactorySettingsPacket,
    SerialPortPassthroughPacket,
    SetZeroOrientationAlignmentPacket,
    StatusPacket,

    SubcomponentInformationPacket,
    SystemState,
    SystemStatus,
    // Time packets (20-23)
    UnixTimePacket,
    VehicleType,
};
pub use protocol::AnppProtocol;
