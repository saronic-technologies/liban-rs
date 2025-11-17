use liban::{Packet, PacketsPeriod, PacketPeriod, FilterOptions, VehicleType, Request, PacketKind};
use clap::Parser as ClapParser;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

#[derive(ClapParser)]
#[command(name = "send_config")]
#[command(about = "Send configuration packets to an ANPP device", long_about = None)]
struct Args {
    /// IP address of the device
    #[arg(short, long, default_value = "192.168.42.42")]
    ip: String,

    /// Port number
    #[arg(short, long, default_value = "16718")]
    port: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("=== ANPP Config Sender ===");
    println!("Connecting to {}:{}...\n", args.ip, args.port);

    let mut stream = TcpStream::connect(format!("{}:{}", args.ip, args.port))?;
    println!("Connected!\n");

    // Example 1: Configure packet periods using Duration
    println!("1. Configuring packet periods...");
    let packets_period = PacketsPeriod {
        permanent: false,
        clear_existing: true,
        packet_periods: vec![
            PacketPeriod {
                packet_type: liban::PacketKind::SystemState,
                period: Duration::from_millis(100),  // 100ms = 10Hz
            },
            PacketPeriod {
                packet_type: liban::PacketKind::Status,
                period: Duration::from_millis(1000),  // 1s = 1Hz
            },
            PacketPeriod {
                packet_type: liban::PacketKind::Satellites,
                period: Duration::from_millis(5000),  // 5s
            },
        ],
    };

    // Convert to bytes and send
    let packet = Packet::PacketsPeriod(packets_period);
    let bytes = packet.to_bytes()?;
    stream.write_all(&bytes)?;
    println!("   Sent PacketsPeriod configuration");
    println!("   - SystemState: 100ms");
    println!("   - Status: 1000ms");
    println!("   - Satellites: 5000ms\n");

    // Example 2: Configure filter options
    println!("2. Configuring filter options...");
    let filter_opts = FilterOptions {
        permanent: false,
        vehicle_type: VehicleType::Boat,
        internal_gnss_enabled: true,
        atmospheric_altitude_enabled: true,
        velocity_heading_enabled: true,
        reversing_detection_enabled: false,
        motion_analysis_enabled: true,
    };

    let packet = Packet::FilterOptions(filter_opts);
    let bytes = packet.to_bytes()?;
    stream.write_all(&bytes)?;
    println!("   Sent FilterOptions configuration");
    println!("   - Vehicle Type: Boat");
    println!("   - Internal GNSS: enabled");
    println!("   - Atmospheric Altitude: enabled");
    println!("   - Velocity Heading: enabled");
    println!("   - Motion Analysis: enabled\n");

    // Example 3: Request device information
    println!("3. Requesting device information...");
    let request = Request {
        requested_packet: PacketKind::DeviceInformation,
    };

    let packet = Packet::Request(request);
    let bytes = packet.to_bytes()?;
    stream.write_all(&bytes)?;
    println!("   Sent request for DeviceInformation\n");

    println!("Configuration sent successfully!");
    println!("Note: All reserved fields were automatically filled with 0");

    Ok(())
}
