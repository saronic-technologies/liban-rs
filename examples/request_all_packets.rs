//! Example: Request all packet types and print using Debug trait
//!
//! This example connects to an Advanced Navigation device and requests
//! every available packet type, printing the response using the Debug trait.
//! This is useful for exploring what data is available from a device.

use binrw::BinRead;
use std::time::Duration;
use liban::Result;
use liban::packet::PacketId;
use liban::packet::system::{DeviceInformationPacket, BootModePacket};
use liban::packet::state::{SystemStatePacket, UnixTimePacket, StatusPacket};
use liban::packet::config::{
    PacketTimerPeriodPacket, PacketsPeriodPacket, InstallationAlignmentPacket,
    FilterOptionsPacket, OdometerConfigurationPacket, ReferencePointOffsetsPacket,
    IpDataportsConfigurationPacket,
};
use liban::protocol::AnppProtocol;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Interface for requesting packets from an Advanced Navigation device
struct PacketRequester {
    stream: TcpStream,
    timeout_duration: Duration,
}

impl PacketRequester {
    /// Create a new packet requester and connect to the device
    async fn new(host: &str, port: u16, timeout_duration: Duration) -> Result<Self> {
        println!("Connecting to {}:{}...", host, port);

        let stream = timeout(
            timeout_duration,
            TcpStream::connect((host, port))
        )
        .await
        .map_err(|_| liban::AnError::Timeout)?
        .map_err(liban::AnError::Network)?;

        println!("Connected!\n");

        Ok(Self {
            stream,
            timeout_duration,
        })
    }

    /// Send a raw packet and receive response
    async fn send_packet(&mut self, packet_id: PacketId, data: &[u8]) -> Result<(u8, Vec<u8>)> {
        // Create and send packet
        let packet = AnppProtocol::create_packet(packet_id, data)?;

        timeout(
            self.timeout_duration,
            self.stream.write_all(&packet)
        )
        .await
        .map_err(|_| liban::AnError::Timeout)?
        .map_err(liban::AnError::Network)?;

        // Read response
        let mut response_buffer = vec![0u8; 1024];
        let bytes_read = timeout(
            self.timeout_duration,
            self.stream.read(&mut response_buffer),
        )
        .await
        .map_err(|_| liban::AnError::Timeout)?
        .map_err(liban::AnError::Network)?;

        if bytes_read == 0 {
            return Err(liban::AnError::Device("No response from device".to_string()));
        }

        response_buffer.truncate(bytes_read);
        let (header, data) = AnppProtocol::parse_packet(&response_buffer)?;
        Ok((header.packet_id.as_u8(), data))
    }

    /// Request a specific packet from the device
    async fn request_packet(&mut self, packet_id: PacketId) -> Result<(u8, Vec<u8>)> {
        let request_data = vec![packet_id.as_u8()];
        self.send_packet(PacketId::new(1), &request_data).await
    }

    /// Generic helper to request a packet and parse the response
    async fn request_and_parse<T>(&mut self, packet_id: PacketId, packet_name: &str) -> Result<T>
    where
        T: for<'a> BinRead<Args<'a> = ()> + std::fmt::Debug,
    {
        println!("Requesting {} (ID {})...", packet_name, packet_id.as_u8());

        match self.request_packet(packet_id).await {
            Ok((response_id, data)) => {
                if response_id != packet_id.as_u8() {
                    return Err(liban::AnError::Device(format!(
                        "Unexpected response packet ID: expected {}, got {}",
                        packet_id.as_u8(),
                        response_id
                    )));
                }

                // Debug: Show raw packet data
                println!("Raw packet data ({} bytes): {:02X?}", data.len(), data);

                let mut cursor = std::io::Cursor::new(&data);
                match T::read_le(&mut cursor) {
                    Ok(parsed_packet) => {
                        println!("✓ {} received:", packet_name);
                        println!("{:#?}\n", parsed_packet);
                        Ok(parsed_packet)
                    }
                    Err(e) => {
                        println!("✗ Failed to parse {}: {}\n", packet_name, e);
                        Err(liban::AnError::InvalidPacket(format!("Failed to parse {}: {}", packet_name, e)))
                    }
                }
            }
            Err(e) => {
                println!("✗ Failed to request {}: {}\n", packet_name, e);
                Err(e)
            }
        }
    }

    /// Request all available packet types
    async fn request_all_packets(&mut self) -> Result<()> {
        println!("=== Requesting All Available Packet Types ===\n");

        // System Packets
        println!("--- System Packets ---");

        let _ = self.request_and_parse::<DeviceInformationPacket>(
            PacketId::new(3),
            "Device Information"
        ).await;

        let _ = self.request_and_parse::<BootModePacket>(
            PacketId::new(2),
            "Boot Mode"
        ).await;

        // State Packets
        println!("--- State Packets ---");

        let _ = self.request_and_parse::<SystemStatePacket>(
            PacketId::new(20),
            "System State"
        ).await;

        let _ = self.request_and_parse::<UnixTimePacket>(
            PacketId::new(21),
            "Unix Time"
        ).await;

        let _ = self.request_and_parse::<StatusPacket>(
            PacketId::new(23),
            "Status"
        ).await;

        // Configuration Packets
        println!("--- Configuration Packets ---");

        let _ = self.request_and_parse::<PacketTimerPeriodPacket>(
            PacketId::new(180),
            "Packet Timer Period"
        ).await;

        let _ = self.request_and_parse::<PacketsPeriodPacket>(
            PacketId::new(181),
            "Packets Period"
        ).await;

        let _ = self.request_and_parse::<InstallationAlignmentPacket>(
            PacketId::new(185),
            "Installation Alignment"
        ).await;

        let _ = self.request_and_parse::<FilterOptionsPacket>(
            PacketId::new(186),
            "Filter Options"
        ).await;

        let _ = self.request_and_parse::<OdometerConfigurationPacket>(
            PacketId::new(192),
            "Odometer Configuration"
        ).await;

        let _ = self.request_and_parse::<ReferencePointOffsetsPacket>(
            PacketId::new(194),
            "Reference Point Offsets"
        ).await;

        let _ = self.request_and_parse::<IpDataportsConfigurationPacket>(
            PacketId::new(202),
            "IP Dataports Configuration"
        ).await;

        println!("=== Packet Request Summary Complete ===");
        Ok(())
    }
}

/// Demonstrate what the packet structures look like using sample data
async fn show_demo_packets() -> Result<()> {
    println!("=== Demo: Sample Packet Structures ===\n");
    println!("This shows what the Debug output looks like for each packet type\n");

    // System Packets
    println!("--- System Packets ---");

    println!("✓ Device Information received:");
    let device_info = DeviceInformationPacket {
        software_version: 0x01020304,
        device_id: 0x89ABCDEF,
        hardware_revision: 0x00010002,
        serial_number_1: 0x12345678,
        serial_number_2: 0x9ABCDEF0,
        serial_number_3: 0x11111111,
    };
    println!("{:#?}\n", device_info);

    println!("✓ Boot Mode received:");
    let boot_mode = BootModePacket { boot_mode: 0 };
    println!("{:#?}\n", boot_mode);

    // State Packets
    println!("--- State Packets ---");

    println!("✓ System State received:");
    let system_state = SystemStatePacket {
        system_status: liban::packet::flags::SystemStatusFlags::empty(),
        filter_status: liban::packet::flags::FilterStatusFlags::ORIENTATION_FILTER_INITIALISED,
        unix_time_seconds: 1640995200,
        microseconds: 123456,
        latitude: 0.785398163, // ~45 degrees
        longitude: 1.047197551, // ~60 degrees
        height: 100.5,
        velocity_north: 1.5,
        velocity_east: 2.5,
        velocity_down: -0.1,
        body_acceleration_x: 0.02,
        body_acceleration_y: -0.01,
        body_acceleration_z: 9.81,
        g_force: 1.0,
        roll: 0.1,
        pitch: -0.05,
        heading: 1.57, // ~90 degrees
        angular_velocity_x: 0.001,
        angular_velocity_y: 0.002,
        angular_velocity_z: 0.003,
        latitude_std_dev: 0.5,
        longitude_std_dev: 0.6,
        height_std_dev: 1.0,
    };
    println!("{:#?}\n", system_state);

    println!("✓ Unix Time received:");
    let unix_time = UnixTimePacket {
        unix_time_seconds: 1640995200,
        microseconds: 123456,
    };
    println!("{:#?}\n", unix_time);

    println!("✓ Status received:");
    let status = StatusPacket {
        system_status: liban::packet::flags::SystemStatusFlags::empty(),
        filter_status: liban::packet::flags::FilterStatusFlags::NAVIGATION_FILTER_INITIALISED,
    };
    println!("{:#?}\n", status);

    // Configuration Packets
    println!("--- Configuration Packets ---");

    println!("✓ Packet Timer Period received:");
    let packet_timer = PacketTimerPeriodPacket {
        permanent: 1,
        utc_synchronisation: 0,
        packet_timer_period: 1000, // 1000ms
    };
    println!("{:#?}\n", packet_timer);

    println!("✓ Packets Period received:");
    let packets_period = PacketsPeriodPacket {
        permanent: 1,
        clear_existing: 0,
        packet_periods: vec![
            liban::packet::config::PacketPeriodEntry {
                packet_id: 20, // SystemState
                period: 100,   // 100ms
            },
            liban::packet::config::PacketPeriodEntry {
                packet_id: 21, // UnixTime
                period: 1000,  // 1000ms
            },
        ],
    };
    println!("{:#?}\n", packets_period);

    println!("✓ Installation Alignment received:");
    let installation = InstallationAlignmentPacket {
        permanent: 1,
        alignment_dcm: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ],
        gnss_antenna_offset: liban::packet::config::OffsetVector { x: 0.0, y: 0.1, z: 0.5 },
        odometer_offset: liban::packet::config::OffsetVector { x: 0.0, y: 0.0, z: -0.2 },
        external_data_offset: liban::packet::config::OffsetVector { x: 0.0, y: 0.0, z: 0.0 },
    };
    println!("{:#?}\n", installation);

    println!("✓ Filter Options received:");
    let filter_options = FilterOptionsPacket {
        permanent: 1,
        vehicle_type: liban::packet::config::VehicleType::Car,
        internal_gnss_enabled: 1,
        reserved1: 0,
        atmospheric_altitude_enabled: 0,
        velocity_heading_enabled: 1,
        reversing_detection_enabled: 1,
        motion_analysis_enabled: 1,
        reserved2: 0,
        reserved3: [0; 8],
    };
    println!("{:#?}\n", filter_options);

    println!("=== Demo Complete ===");
    println!("To connect to a real device, use: <program> <host> <port>");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Handle demo mode
    if args.len() >= 2 && args[1] == "--demo" {
        show_demo_packets().await?;
        return Ok(());
    }

    if args.len() < 3 {
        println!("Usage: {} <host> <port> [timeout_seconds]", args[0]);
        println!("       {} --demo", args[0]);
        println!("Example: {} 192.168.1.100 16718", args[0]);
        println!("         {} 192.168.1.100 16718 10", args[0]);
        println!("         {} --demo   # Show sample packet structures", args[0]);
        std::process::exit(1);
    }

    let host = &args[1];
    let port: u16 = args[2].parse().map_err(|_| {
        liban::AnError::Network(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid port number"
        ))
    })?;

    let timeout_seconds = if args.len() >= 4 {
        args[3].parse().unwrap_or(5)
    } else {
        5
    };

    println!("=== Advanced Navigation Packet Explorer ===");
    println!("Connecting to {}:{} with {}s timeout", host, port, timeout_seconds);
    println!("This example will request every available packet type and print the response\n");

    let mut requester = PacketRequester::new(host, port, Duration::from_secs(timeout_seconds)).await?;

    if let Err(e) = requester.request_all_packets().await {
        eprintln!("Error during packet requests: {}", e);
        std::process::exit(1);
    }

    Ok(())
}