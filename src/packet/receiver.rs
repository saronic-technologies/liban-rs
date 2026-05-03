use binrw::{BinRead, BinResult, BinWrite, Endian, binrw};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

/// Trimble receiver model identifier
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrimbleModel {
    #[default]
    Unknown = 0,
    /// Trimble MB-Two
    MbTwo = 5,
    /// Trimble BD992
    Bd992 = 7,
}

impl TrimbleModel {
    /// Wire layout to use when parsing the receiver-specific payload
    pub fn receiver_type(self) -> ReceiverType {
        match self {
            Self::Bd992 => ReceiverType::TrimbleBd992,
            _ => ReceiverType::Generic,
        }
    }
}

impl From<u8> for TrimbleModel {
    fn from(v: u8) -> Self {
        match v {
            5 => Self::MbTwo,
            7 => Self::Bd992,
            _ => Self::Unknown,
        }
    }
}

/// u-blox receiver model identifier
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UBloxModel {
    #[default]
    Unknown = 0,
    /// u-blox NEO-F9P
    NeoF9P = 5,
}

impl From<u8> for UBloxModel {
    fn from(v: u8) -> Self {
        match v {
            5 => Self::NeoF9P,
            _ => Self::Unknown,
        }
    }
}

/// Advanced Navigation receiver model identifier
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdvancedNavigationModel {
    #[default]
    Unknown = 0,
    /// Aries GNSS receiver
    Aries = 1,
    /// Aries GC2 GNSS receiver
    AriesGc2 = 2,
}

impl From<u8> for AdvancedNavigationModel {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Aries,
            2 => Self::AriesGc2,
            _ => Self::Unknown,
        }
    }
}

/// GNSS manufacturer and receiver model, encoded as a 2-byte header on the wire
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssManufacturer {
    Unknown { manufacturer: u8, model: u8 },
    Trimble(TrimbleModel),
    UBlox(UBloxModel),
    AdvancedNavigation(AdvancedNavigationModel),
}

impl GnssManufacturer {
    /// Wire layout to use when parsing the receiver-specific payload
    pub fn receiver_type(&self) -> ReceiverType {
        match self {
            Self::Trimble(m) => m.receiver_type(),
            _ => ReceiverType::Generic,
        }
    }

    pub(crate) fn parse<R: Read + Seek>(reader: &mut R, endian: Endian, _: ()) -> BinResult<Self> {
        let manufacturer = u8::read_options(reader, endian, ())?;
        let model = u8::read_options(reader, endian, ())?;
        Ok(match manufacturer {
            1 => Self::Trimble(TrimbleModel::from(model)),
            2 => Self::UBlox(UBloxModel::from(model)),
            3 => Self::AdvancedNavigation(AdvancedNavigationModel::from(model)),
            _ => Self::Unknown { manufacturer, model },
        })
    }

    pub(crate) fn write_to<W: Write + Seek>(
        val: &Self,
        writer: &mut W,
        endian: Endian,
        _: (),
    ) -> BinResult<()> {
        let (manufacturer, model) = match val {
            &Self::Unknown { manufacturer, model } => (manufacturer, model),
            Self::Trimble(m) => (1, *m as u8),
            Self::UBlox(m) => (2, *m as u8),
            Self::AdvancedNavigation(m) => (3, *m as u8),
        };
        manufacturer.write_options(writer, endian, ())?;
        model.write_options(writer, endian, ())
    }
}

/// Selects the wire layout used when parsing the receiver-specific payload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiverType {
    TrimbleBd992,
    Generic,
}

/// Omnistar differential correction engine mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum OmnistarEngineMode {
    #[default]
    NotActive = 0,
    Hp = 1,
    Xp = 2,
    G2 = 3,
    HpG2 = 4,
    HpXp = 5,
}

/// RTK software license accuracy level
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum RtkLicenseAccuracy {
    #[default]
    Unknown = 0,
    /// 0.3 m horizontal accuracy, 0.3 m vertical accuracy
    H30cmV30cm = 1,
    /// 0.1 m horizontal accuracy, 0.1 m vertical accuracy
    H10cmV10cm = 2,
    /// 0.1 m horizontal accuracy, 0.02 m vertical accuracy
    H10cmV2cm = 3,
    /// 0.008 m horizontal accuracy, 0.1 m vertical accuracy
    H8mmV10cm = 4,
    /// 0.008 m horizontal accuracy, 0.02 m vertical accuracy
    H8mmV2cm = 5,
}

/// Trimble BD992 receiver-specific payload (46 bytes)
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrimbleBd992ReceiverData {
    /// Serial number as ASCII string (10 bytes)
    pub serial_number: [u8; 10],
    /// Firmware version
    pub firmware_version: u32,
    /// Software license code
    pub software_license_code: [u32; 3],
    /// Omnistar serial number
    pub omnistar_serial_number: u32,
    /// Subscription start time (Unix seconds)
    pub subscription_start_time: u32,
    /// Subscription expiry time (Unix seconds)
    pub subscription_expiry_time: u32,
    /// Omnistar differential correction engine mode
    pub omnistar_engine_mode: OmnistarEngineMode,
    /// RTK software license accuracy level
    pub rtk_license_accuracy: RtkLicenseAccuracy,
    #[br(temp)]
    #[bw(calc = [0u8; 6])]
    _reserved: [u8; 6],
}

/// Generic receiver payload used for all non-Trimble-BD992 models (66 bytes)
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericReceiverData {
    /// Serial number as ASCII string (24 bytes)
    pub serial_number: [u8; 24],
    /// Firmware version
    pub firmware_version: u32,
    /// Hardware version
    pub hardware_version: u32,
    #[br(temp)]
    #[bw(calc = [0u8; 34])]
    _reserved: [u8; 34],
}

/// Receiver-specific payload within GnssReceiverInformation
#[derive(Debug, Clone, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[br(import(receiver_type: ReceiverType))]
pub enum GnssReceiverData {
    #[br(pre_assert(receiver_type == ReceiverType::TrimbleBd992))]
    TrimbleBd992(TrimbleBd992ReceiverData),
    Generic(GenericReceiverData),
}
