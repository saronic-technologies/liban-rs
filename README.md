# liban - Advanced Navigation Library

A comprehensive Rust library for communicating with Advanced Navigation devices using the Advanced Navigation Packet Protocol (ANPP)

## Supported Packet Types

The library implements the following ANPP (Advanced Navigation Packet Protocol) packets:

### System Packets (0-14)
- **AcknowledgePacket** (ID 0) - Device command acknowledgments
- **RequestPacket** (ID 1) - Request specific packet types from device  
- **BootModePacket** (ID 2) - Device boot mode control
- **DeviceInformation** (ID 3) - Hardware/software version info and 3-part serial number
- **RestoreFactorySettingsPacket** (ID 4) - Factory reset command with verification 0x85429E1C (re-enables DHCP)
- **ResetPacket** (ID 5) - Device reset command with verification 0x21057A7E
- **SerialPortPassthroughPacket** (ID 10) - Serial port data passthrough
- **IpConfigurationPacket** (ID 11) - Network configuration settings with IP address conversion
- **SubcomponentInformationPacket** (ID 14) - Subcomponent details

### State Packets (20-23)
- **SystemState** (ID 20) - Complete navigation state (position, velocity, attitude, accelerations) with status interpretation
- **UnixTimePacket** (ID 21) - Unix timestamp with microsecond precision
- **FormattedTimePacket** (ID 22) - Human-readable date/time breakdown
- **StatusPacket** (ID 23) - System and filter status flags with comprehensive bit interpretation

### Configuration Packets (180-202)
- **PacketTimerPeriodPacket** (ID 180) - Packet transmission timer period with UTC synchronization support
- **PacketsPeriodPacket** (ID 181) - Individual packet transmission rates with variable length
- **InstallationAlignmentPacket** (ID 185) - Device mounting alignment parameters
- **FilterOptionsPacket** (ID 186) - Navigation filter configuration with 15 vehicle types (0-14)
- **OdometerConfigurationPacket** (ID 192) - Odometer sensor parameters with automatic pulse measurement
- **SetZeroOrientationAlignmentPacket** (ID 193) - Zero orientation reference with verification 0x9A4E8055
- **ReferencePointOffsetsPacket** (ID 194) - Reference point offsets for 4 heave points with COG lever arm support
- **IpDataportsConfigurationPacket** (ID 202) - 4 IP dataport configurations (TCP Server/Client, UDP, MODE_NONE)

## Protocol Details

### ANPP Packet Structure
```
[Header LRC][Packet ID][Length][CRC16][Data Payload]
```

- **Header LRC**: `(PacketID + Length + CRC0 + CRC1) XOR 0xFF + 1`
- **CRC16-CCITT**: Polynomial 0x1021 over data payload
- **Little-endian**: All multi-byte values
- **Maximum payload**: 255 bytes

## Testing

Run tests with:

```bash
cargo test
cargo test --features integration-tests  # Requires hardware
```