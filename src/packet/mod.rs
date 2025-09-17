
pub mod system;
pub mod state;
pub mod config;
pub mod flags;

/// ANPP packet identifiers for Advanced Navigation devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PacketId {
    // System Packets
    Acknowledge = 0,
    Request = 1,
    BootMode = 2,
    DeviceInformation = 3,
    RestoreFactorySettings = 4,
    Reset = 5,
    IpConfiguration = 11,
    // State Packets
    SystemState = 20,
    UnixTime = 21,
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
            11 => Some(Self::IpConfiguration),
            20 => Some(Self::SystemState),
            21 => Some(Self::UnixTime),
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


/// Macro to implement TryFrom<&[u8]>, TryFrom<Vec<u8>>, and TryInto<Vec<u8>> for ANPP packet types using binrw
macro_rules! impl_binrw_packet_conversions {
    ($packet_type:ty) => {
        impl TryFrom<&[u8]> for $packet_type {
            type Error = AnError;

            fn try_from(data: &[u8]) -> Result<Self> {
                use binrw::BinRead;
                let mut cursor = std::io::Cursor::new(data);
                Self::read_le(&mut cursor)
                    .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize {}: {}", stringify!($packet_type), e)))
            }
        }

        impl TryFrom<Vec<u8>> for $packet_type {
            type Error = AnError;

            fn try_from(data: Vec<u8>) -> Result<Self> {
                Self::try_from(data.as_slice())
            }
        }

        impl TryInto<Vec<u8>> for $packet_type {
            type Error = AnError;

            fn try_into(self) -> Result<Vec<u8>> {
                use binrw::BinWrite;
                let mut cursor = std::io::Cursor::new(Vec::new());
                self.write_le(&mut cursor)
                    .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize {}: {}", stringify!($packet_type), e)))?;
                Ok(cursor.into_inner())
            }
        }
    };
}

pub(crate) use impl_binrw_packet_conversions;