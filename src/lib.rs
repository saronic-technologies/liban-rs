//! # liban - Advanced Navigation Packet Protocol Library (Sans-IO)
//!
//! A sans-io Rust library for working with Advanced Navigation devices using the
//! Advanced Navigation Packet Protocol (ANPP), with specific support for the Boreas D90.

pub mod error;
pub mod packet;
pub mod parser;
pub mod protocol;
pub mod reader;
pub mod types;

pub use error::{AnError, Result};
pub use packet::PacketKind;
pub use parser::AnppParser;

// Re-export clean API types as the primary public interface
pub use types::*;
