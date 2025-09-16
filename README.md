# liban - Advanced Navigation Library

A comprehensive Rust library for communicating with Advanced Navigation devices using the Advanced Navigation Packet Protocol (ANPP), with specific support for the Boreas D90 GPS/INS system.

## Features

- **Complete ANPP Implementation**: Full Advanced Navigation Packet Protocol support with CRC16-CCITT validation
- **Async TCP Communication**: Built on Tokio for efficient network operations
- **Type-Safe Packet Handling**: Strongly typed packet structures with serde serialization
- **Comprehensive Error Management**: Detailed error types with proper error propagation
- **Boreas D90 Support**: Specialized functions for Boreas D90 GPS/INS devices
- **Connection Management**: Automatic reconnection, timeout handling, and graceful disconnection

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
liban = { path = "../liban" }
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use liban::{BoreasInterface, PacketId};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to Boreas D90 device
    let mut interface = BoreasInterface::new(
        "192.168.1.100",  // Device IP
        16718,            // Standard Boreas TCP port
        Duration::from_secs(5)  // Timeout
    ).await?;

    // Get device information
    let device_info = interface.get_device_information().await?;
    println!("Device ID: {}", device_info.device_id);
    println!("Serial Number: {}", device_info.get_serial_number());
    
    // Get current system state
    let system_state = interface.get_system_state().await?;
    println!("Position: {:.6}, {:.6}", 
             system_state.latitude, 
             system_state.longitude);
    println!("Heading: {:.2}°", system_state.heading);

    println!("Roll: {:.2}°, Pitch: {:.2}°, Yaw: {:.2}°", 
             system_state.roll.to_degrees(), 
             system_state.pitch.to_degrees(), 
             system_state.heading.to_degrees());
    
    // Gracefully disconnect
    interface.disconnect().await?;
    Ok(())
}
```

### Configuration Example

```rust,no_run
use liban::BoreasInterface;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut interface = BoreasInterface::new(
        "boreas.local", 16718, Duration::from_secs(10)
    ).await?;

    // Configure IP settings (example)
    let ip_config = vec![
        192, 168, 1, 101,  // New IP address
        255, 255, 255, 0,  // Subnet mask
        192, 168, 1, 1,    // Gateway
    ];
    
    let ack = interface.configure_ip(&ip_config).await?;
    if ack.result == 0 {
        println!("IP configuration successful");
    } else {
        println!("IP configuration failed: {}", ack.result);
    }

    // Reset device to apply changes
    interface.reset_device().await?;
    
    // Note: Factory reset can be performed with:
    // interface.restore_factory_settings().await?;
    // Warning: This will re-enable DHCP and lose static IP settings
    
    Ok(())
}
```

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

### Advanced Features
- **FilterStatus** - 16-bit filter status interpretation with initialization tracking
- **SystemStatus** - 16-bit system status with comprehensive alarm and failure detection
- **VehicleType** enum - 15 predefined vehicle types for optimal filter performance
- **IpDataportMode** enum - 5 connection modes (MODE_NONE, Reserved, MODE_TCP_SERVER, MODE_TCP_CLIENT, MODE_UDP)
- **OffsetVector** - 3D coordinate management for reference point calculations

All packet types support automatic serialization/deserialization using the `AnppPacket` trait with serde and bincode for ANPP protocol compliance.

## Protocol Details

### ANPP Packet Structure
```
[Header LRC][Packet ID][Length][CRC16][Data Payload]
```

- **Header LRC**: `(PacketID + Length + CRC0 + CRC1) XOR 0xFF + 1`
- **CRC16-CCITT**: Polynomial 0x1021 over data payload
- **Little-endian**: All multi-byte values
- **Maximum payload**: 255 bytes

### Connection Parameters
- **Default Port**: 16718 (Boreas D90)
- **Protocol**: TCP/IP
- **Timeout**: Configurable (recommended 5-10 seconds)
- **Byte Order**: Little-endian

## Error Handling

```rust,no_run
use liban::{BoreasInterface, AnError};

match interface.get_system_state().await {
    Ok(state) => println!("Success: {:?}", state),
    Err(AnError::Timeout) => println!("Request timed out"),
    Err(AnError::InvalidChecksum) => println!("Data corruption detected"),
    Err(AnError::Network(e)) => println!("Network error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Advanced Usage

### Custom Packet Handling

```rust,no_run
use liban::{AnppProtocol, AnppPacket, PacketId, SystemState, RestoreFactorySettingsPacket, ResetPacket};
use liban::{FilterOptionsPacket, VehicleType, IpDataportsConfigurationPacket, IpDataportMode};
use std::net::Ipv4Addr;

// Create and serialize a packet
let system_state = SystemState { /* ... */ };
let packet_bytes = system_state.to_bytes()?;

// Send raw packet data
let (response_id, response_data) = interface.send_packet(
    PacketId::SystemState.as_u8(), 
    &packet_bytes
).await?;

// Deserialize received packet
let parsed_state = SystemState::from_bytes(&response_data)?;

// Create command packets with correct verification codes
let factory_reset = RestoreFactorySettingsPacket::new(); // Uses 0x85429E1C
let reset = ResetPacket::new(); // Uses 0x21057A7E

// Configure filter options with vehicle type
let mut filter_config = FilterOptionsPacket::new(true, VehicleType::Boat);
filter_config.set_internal_gnss_enabled(true);
filter_config.set_velocity_heading_enabled(true);

// Configure IP dataports
let mut ip_config = IpDataportsConfigurationPacket::new();
ip_config.set_tcp_server(0, 16718)?;  // Dataport 1 as TCP server on port 16718
ip_config.set_udp(1, Ipv4Addr::new(192, 168, 1, 100), 8080)?;  // Dataport 2 as UDP client

// Status interpretation
let system_state = interface.get_system_state().await?;
let system_status = system_state.get_system_status();
let filter_status = system_state.get_filter_status();

if system_status.is_healthy() && filter_status.is_fully_initialised() {
    println!("System ready for navigation!");
} else {
    println!("System issues: {:?}", system_status.get_active_flags());
}
```

### Connection Management

```rust,no_run
// Check connection status
if !interface.is_connected() {
    interface.connect().await?;
}

// Manual reconnection after network issues
interface.disconnect().await?;
tokio::time::sleep(Duration::from_secs(1)).await;
interface.connect().await?;
```

## Integration with Existing Systems

This library is designed to integrate with Saronic's autonomous vehicle systems:

```rust,no_run
use liban::{BoreasInterface, VehicleType, OffsetVector};

// Convert to AHRS format (example)
let system_state = interface.get_system_state().await?;
let system_status = system_state.get_system_status();
let filter_status = system_state.get_filter_status();

let ahrs = Ahrs {
    gps_lock: filter_status.gnss_fix_status(),
    location: Lla {
        lat: system_state.latitude,
        lon: system_state.longitude, 
        alt: system_state.height as f64,
    },
    attitude: Att {
        roll: system_state.roll as f64,
        pitch: system_state.pitch as f64,
        yaw: system_state.heading as f64,
    },
    system_healthy: system_status.is_healthy(),
    filter_initialized: filter_status.is_fully_initialised(),
    // ... other fields
};

// Configure for maritime vehicle
let mut filter_config = FilterOptionsPacket::new(true, VehicleType::Boat);
filter_config.set_internal_gnss_enabled(true);
filter_config.set_velocity_heading_enabled(true);
filter_config.set_motion_analysis_enabled(true);

// Set reference point offsets for sensor mounting
let mut offsets = ReferencePointOffsetsPacket::new(true);
offsets.set_primary_reference_offset(OffsetVector::new(1.5, 0.0, -0.5));  // 1.5m forward, 0.5m down
offsets.set_cog_lever_arm_offset(OffsetVector::new(0.0, 0.0, 0.0));  // At center

// Configure multiple data streams
let mut ip_config = IpDataportsConfigurationPacket::new();
ip_config.set_tcp_server(0, 16718)?;  // Main data stream
ip_config.set_udp(1, Ipv4Addr::new(192, 168, 1, 200), 8080)?;  // Secondary stream
```

## Testing

Run tests with:

```bash
cargo test
cargo test --features integration-tests  # Requires hardware
```

## Hardware Requirements

- Advanced Navigation Boreas D90 GPS/INS device
- Network connectivity (Ethernet recommended)
- TCP port 16718 accessible

## License

This library is part of the Saronic prototype software suite.