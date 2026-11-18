use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

/// Trimble receiver model identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum TrimbleModel {
    /// Trimble BD920
    Bd920 = 1,
    /// Trimble BD930
    Bd930 = 2,
    /// Trimble BD982
    Bd982 = 3,
    /// Trimble MB-One
    MbOne = 4,
    /// Trimble MB-Two
    MbTwo = 5,
    /// Trimble BD940
    Bd940 = 6,
    /// Trimble BD992
    Bd992 = 7,
    #[num_enum(catch_all)]
    Unknown(u8),
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

/// u-blox receiver model identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum UBloxModel {
    /// u-blox ZED-M8
    ZedM8 = 1,
    /// u-blox ZED-M8T
    ZedM8T = 2,
    /// u-blox ZED-M8P
    ZedM8P = 3,
    /// u-blox ZED-F9P
    ZedF9P = 4,
    /// u-blox NEO-F9P
    NeoF9P = 5,
    /// u-blox ZED-X20P
    ZedX20P = 6,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Advanced Navigation receiver model identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum AdvancedNavigationModel {
    /// Original Aries GNSS receiver, a.k.a. Aries F9P
    Aries = 1,
    /// Aries GC2 GNSS receiver
    AriesGc2 = 2,
    /// Aries SkyTraq Orion B16 GNSS receiver
    AriesSkyTraqOrionB16 = 3,
    /// Aries SkyTraq PX1172RH GNSS receiver
    AriesSkyTraqPx1172Rh = 4,
    /// Aries X20P GNSS receiver
    AriesX20P = 5,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Septentrio receiver model identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SeptentrioModel {
    /// Septentrio mosaic-X5
    MosaicX5 = 1,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// GNSS manufacturer identifier, the first byte of the 2-byte header
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum ManufacturerId {
    Trimble = 1,
    UBlox = 2,
    AdvancedNavigation = 3,
    Septentrio = 6,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// GNSS manufacturer and receiver model, encoded as a 2-byte header on the wire
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GnssManufacturer {
    Unknown { manufacturer: u8, model: u8 },
    Trimble(TrimbleModel),
    UBlox(UBloxModel),
    AdvancedNavigation(AdvancedNavigationModel),
    Septentrio(SeptentrioModel),
}

impl GnssManufacturer {
    /// Wire layout to use when parsing the receiver-specific payload
    pub fn receiver_type(&self) -> ReceiverType {
        match self {
            Self::Trimble(m) => m.receiver_type(),
            _ => ReceiverType::Generic,
        }
    }

    pub(crate) fn parse<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: (),
    ) -> BinResult<Self> {
        let manufacturer = u8::read_options(reader, endian, ())?;
        let model = u8::read_options(reader, endian, ())?;
        Ok(match ManufacturerId::from(manufacturer) {
            ManufacturerId::Trimble => Self::Trimble(TrimbleModel::from(model)),
            ManufacturerId::UBlox => Self::UBlox(UBloxModel::from(model)),
            ManufacturerId::AdvancedNavigation => {
                Self::AdvancedNavigation(AdvancedNavigationModel::from(model))
            }
            ManufacturerId::Septentrio => Self::Septentrio(SeptentrioModel::from(model)),
            ManufacturerId::Unknown(_) => Self::Unknown { manufacturer, model },
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
            Self::Trimble(m) => (u8::from(ManufacturerId::Trimble), u8::from(*m)),
            Self::UBlox(m) => (u8::from(ManufacturerId::UBlox), u8::from(*m)),
            Self::AdvancedNavigation(m) => {
                (u8::from(ManufacturerId::AdvancedNavigation), u8::from(*m))
            }
            Self::Septentrio(m) => (u8::from(ManufacturerId::Septentrio), u8::from(*m)),
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
