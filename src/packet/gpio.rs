use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

/// Which GPIO pins support a given function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioFunctionType {
    Gpio1,
    Gpio2,
    Both,
}

/// Which auxiliary port directions support a given function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioAuxFunctionType {
    Transmit,
    Receive,
    Both,
}

/// GPIO pin function selection
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum GpioFunction {
    #[default]
    Inactive = 0,
    Pps1Output = 1,
    GnssFixOutput = 2,
    OdometerInput = 3,
    ZeroVelocityInput = 4,
    PitotTubeInput = 5,
    NmeaInput = 6,
    NmeaOutput = 7,
    NovatelGnssInput = 8,
    TopconGnssInput = 9,
    AnppInput = 11,
    AnppOutput = 12,
    DisableMagnetometers = 13,
    DisableGnss = 14,
    DisablePressure = 15,
    SetZeroOrientationAlignment = 16,
    SystemStatePacketTrigger = 17,
    RawSensorsPacketTrigger = 18,
    RtcmCorrectionsInput = 19,
    TrimbleGnssInput = 20,
    UBloxGnssInput = 21,
    HemisphereGnssInput = 22,
    TeledyneDvlInput = 23,
    TritechUsblInput = 24,
    LinkquestDvlInput = 25,
    PressureDepthTransducer = 26,
    LeftWheelSpeedSensor = 27,
    RightWheelSpeedSensor = 28,
    Pps1Input = 29,
    WheelSpeedSensor = 30,
    WheelEncoderPhaseA = 31,
    WheelEncoderPhaseB = 32,
    Event1Input = 33,
    Event2Input = 34,
    GnssReceiverPassthrough = 38,
    Tss1Output = 39,
    Simrad1000Output = 40,
    Simrad3000Output = 41,
    SerialPortPassthrough = 42,
    GimbalEncoderPhaseA = 43,
    GimbalEncoderPhaseB = 44,
    OdometerDirectionForwardLow = 45,
    OdometerDirectionForwardHigh = 46,
    NortekDvlInput = 51,
    ReverseAlignmentForwardLow = 53,
    ReverseAlignmentForwardHigh = 54,
    ZeroAngularVelocityInput = 55,
    WaterLinkedDvlInput = 59,
    NortekNucleusDvlInput = 63,
    NortekNucleusDvlOutput = 64,
    ValeportSvsInput = 65,
    SeptentrioGnssInput = 66,
}

impl GpioFunction {
    pub fn function_type(self) -> GpioFunctionType {
        match self {
            Self::NmeaOutput
            | Self::AnppOutput
            | Self::Tss1Output
            | Self::Simrad1000Output
            | Self::Simrad3000Output
            | Self::NortekNucleusDvlOutput => GpioFunctionType::Gpio1,
            Self::NmeaInput
            | Self::NovatelGnssInput
            | Self::TopconGnssInput
            | Self::AnppInput
            | Self::RtcmCorrectionsInput
            | Self::TrimbleGnssInput
            | Self::UBloxGnssInput
            | Self::HemisphereGnssInput
            | Self::TeledyneDvlInput
            | Self::TritechUsblInput
            | Self::LinkquestDvlInput
            | Self::NortekDvlInput
            | Self::WaterLinkedDvlInput
            | Self::NortekNucleusDvlInput
            | Self::ValeportSvsInput
            | Self::SeptentrioGnssInput => GpioFunctionType::Gpio2,
            _ => GpioFunctionType::Both,
        }
    }
}

/// Auxiliary RS232 port function selection
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum AuxiliaryFunction {
    #[default]
    Inactive = 0,
    Pps1Output = 1,
    GnssFixOutput = 2,
    OdometerInput = 3,
    ZeroVelocityInput = 4,
    PitotTubeInput = 5,
    NmeaInput = 6,
    NmeaOutput = 7,
    NovatelGnssInput = 8,
    TopconGnssInput = 9,
    AnppInput = 11,
    AnppOutput = 12,
    DisableMagnetometers = 13,
    DisableGnss = 14,
    DisablePressure = 15,
    SetZeroOrientationAlignment = 16,
    SystemStatePacketTrigger = 17,
    RawSensorsPacketTrigger = 18,
    RtcmCorrectionsInput = 19,
    TrimbleGnssInput = 20,
    UBloxGnssInput = 21,
    HemisphereGnssInput = 22,
    TeledyneDvlInput = 23,
    TritechUsblInput = 24,
    LinkquestDvlInput = 25,
    PressureDepthTransducer = 26,
    LeftWheelSpeedSensor = 27,
    RightWheelSpeedSensor = 28,
    Pps1Input = 29,
    WheelSpeedSensor = 30,
    Event1Input = 33,
    Event2Input = 34,
    LinkquestUsblInput = 35,
    GnssReceiverPassthrough = 38,
    Tss1Output = 39,
    Simrad1000Output = 40,
    Simrad3000Output = 41,
    SerialPortPassthrough = 42,
    OdometerDirectionForwardLow = 45,
    OdometerDirectionForwardHigh = 46,
    NortekDvlInput = 51,
    ReverseAlignmentForwardLow = 53,
    ReverseAlignmentForwardHigh = 54,
    ZeroAngularVelocityInput = 55,
    WaterLinkedDvlInput = 59,
    NortekNucleusDvlInput = 63,
    NortekNucleusDvlOutput = 64,
    ValeportSvsInput = 65,
    SeptentrioGnssInput = 66,
}

impl AuxiliaryFunction {
    pub fn function_type(self) -> GpioAuxFunctionType {
        match self {
            Self::Pps1Output
            | Self::GnssFixOutput
            | Self::NmeaOutput
            | Self::AnppOutput
            | Self::Tss1Output
            | Self::Simrad1000Output
            | Self::Simrad3000Output
            | Self::NortekNucleusDvlOutput => GpioAuxFunctionType::Transmit,
            Self::OdometerInput
            | Self::ZeroVelocityInput
            | Self::PitotTubeInput
            | Self::NmeaInput
            | Self::NovatelGnssInput
            | Self::TopconGnssInput
            | Self::AnppInput
            | Self::DisableGnss
            | Self::DisablePressure
            | Self::SetZeroOrientationAlignment
            | Self::SystemStatePacketTrigger
            | Self::RawSensorsPacketTrigger
            | Self::RtcmCorrectionsInput
            | Self::TrimbleGnssInput
            | Self::UBloxGnssInput
            | Self::HemisphereGnssInput
            | Self::TeledyneDvlInput
            | Self::TritechUsblInput
            | Self::LinkquestDvlInput
            | Self::PressureDepthTransducer
            | Self::LeftWheelSpeedSensor
            | Self::RightWheelSpeedSensor
            | Self::Pps1Input
            | Self::WheelSpeedSensor
            | Self::Event1Input
            | Self::Event2Input
            | Self::LinkquestUsblInput
            | Self::OdometerDirectionForwardLow
            | Self::OdometerDirectionForwardHigh
            | Self::NortekDvlInput
            | Self::ReverseAlignmentForwardLow
            | Self::ReverseAlignmentForwardHigh
            | Self::ZeroAngularVelocityInput
            | Self::WaterLinkedDvlInput
            | Self::NortekNucleusDvlInput
            | Self::ValeportSvsInput
            | Self::SeptentrioGnssInput => GpioAuxFunctionType::Receive,
            _ => GpioAuxFunctionType::Both,
        }
    }
}

/// GPIO logic voltage level for GpioConfiguration
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(repr = u8)]
pub enum GpioVoltage {
    Volts5 = 0,
    Volts3_3 = 1,
    #[default]
    PowerDisabled = 2,
}
