use liban::{BoreasInterface, PacketId, SystemStatus};
use std::io::{self, Write};
use std::time::Duration;
use tokio;

/// Interactive packet viewer for Advanced Navigation Boreas devices
/// Displays available packet types
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced Navigation Packet Viewer");
    println!("=====================================");

    // Get connection details
    print!("Enter Boreas device IP address (default: 192.168.0.42): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let host = input.trim();
    let host = if host.is_empty() {
        "192.168.0.42"
    } else {
        host
    }
    .to_string();

    print!("Enter TCP port (default: 16720): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let port: u16 = input.trim().parse().unwrap_or(16720);

    // Connect to device
    println!("\n🔗 Connecting to {}:{}...", host, port);
    let mut interface = match BoreasInterface::new(&host, port, Duration::from_secs(10)).await {
        Ok(interface) => {
            println!("✅ Connected successfully!");
            interface
        }
        Err(e) => {
            println!("❌ Connection failed: {}", e);
            return Ok(());
        }
    };

    // Show available packets
    show_available_packets();

    loop {
        println!("\n{}", "=".repeat(50));
        print!("Enter packet ID (or 'quit' to exit): ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
            break;
        }

        if input.eq_ignore_ascii_case("help") || input.eq_ignore_ascii_case("h") {
            show_available_packets();
            continue;
        }

        let packet_id: u8 = match input.parse() {
            Ok(id) => id,
            Err(_) => {
                println!("❌ Invalid packet ID. Enter a number between 0-255.");
                continue;
            }
        };

        // Check if packet is available
        if !is_readable_packet(packet_id) {
            println!(
                "❌ Packet ID {} is not available or not implemented.",
                packet_id
            );
            println!("   Only implemented packets can be requested.");
            println!("   Type 'help' to see available packets.");
            continue;
        }

        // Request and display packet
        match request_and_display_packet(&mut interface, packet_id).await {
            Ok(_) => {}
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    println!("\n👋 Disconnecting...");
    interface.disconnect().await?;
    println!("✅ Disconnected successfully!");

    Ok(())
}

fn show_available_packets() {
    println!("\n📦 Available Packet Types:");
    println!("System Packets:");
    println!("  0  - Acknowledge ✅");
    println!("  2  - Boot Mode ✅");
    println!("  3  - Device Information ⭐✅");
    println!("  10 - Serial Port Passthrough ✅");
    println!("  11 - IP Configuration ✅");
    println!("  14 - Subcomponent Information ✅");

    println!("\nState Packets (Read-only):");
    println!("  20 - System State ⭐✅");
    println!("  21 - Unix Time ⭐✅");
    println!("  22 - Formatted Time ⭐✅");
    println!("  23 - Status ⭐✅");

    println!("\nConfiguration Packets:");
    println!("  180 - Packet Timer Period ⭐✅");
    println!("  181 - Packets Period ✅");
    println!("  185 - Installation Alignment ⭐✅");
    println!("  186 - Filter Options ⭐✅");
    println!("  192 - Odometer Configuration ⭐✅");
    println!("  193 - Set Zero Orientation Alignment ✅");
    println!("  194 - Reference Point Offsets ⭐✅");
    println!("  202 - IP Dataports Configuration ⭐✅");

    println!("\n⭐ = Detailed parsing available");
    println!("✅ = Available for request");
    println!("Type 'help' to see this list again");
}

/// Check if a packet ID is available for request
fn is_readable_packet(packet_id: u8) -> bool {
    matches!(
        packet_id,
        0   |  // Acknowledge
        2   |  // Boot Mode  
        3   |  // Device Information
        10  |  // Serial Port Passthrough
        11  |  // IP Configuration
        14  |  // Subcomponent Information
        20  |  // System State
        21  |  // Unix Time
        22  |  // Formatted Time
        23  |  // Status
        180 |  // Packet Timer Period
        181 |  // Packets Period
        185 |  // Installation Alignment
        186 |  // Filter Options
        192 |  // Odometer Configuration
        193 |  // Set Zero Orientation Alignment
        194 |  // Reference Point Offsets
        202 // IP Dataports Configuration
    )
}

async fn request_and_display_packet(
    interface: &mut BoreasInterface,
    packet_id: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Requesting packet ID {}...", packet_id);

    // Convert to PacketId enum if possible
    let packet_enum = PacketId::from_u8(packet_id);

    match packet_enum {
        Some(pid) => {
            println!("📋 Packet Type: {:?}", pid);

            // Use specialized methods for packets with detailed parsing
            match pid {
                PacketId::DeviceInformation => {
                    let info = interface.get_device_information().await?;
                    println!("✨ Device Information:");
                    println!("  Software Version: {}", info.software_version);
                    println!("  Device ID: {}", info.device_id);
                    println!("  Hardware Revision: {}", info.hardware_revision);
                    println!("  Serial Number: {}", info.get_serial_number());
                    println!(
                        "  Serial Number Parts: {} / {} / {}",
                        info.serial_number_1, info.serial_number_2, info.serial_number_3
                    );
                }

                PacketId::SystemState => {
                    let state = interface.get_system_state().await?;
                    let system_status = state.get_system_status();

                    println!("✨ System State:");
                    println!("  🚨 System Status: {}", system_status);
                    println!("  🔧 Filter Status: 0x{:04X}", state.filter_status);
                    println!(
                        "  ⏰ Unix Time: {} seconds, {} microseconds",
                        state.unix_time_seconds, state.microseconds
                    );
                    println!("  🌍 Position:");
                    println!(
                        "    Latitude: {:.8} rad ({:.6}°)",
                        state.latitude,
                        state.latitude.to_degrees()
                    );
                    println!(
                        "    Longitude: {:.8} rad ({:.6}°)",
                        state.longitude,
                        state.longitude.to_degrees()
                    );
                    println!("    Height: {:.3} m", state.height);
                    println!("  🏃 Velocity (NED):");
                    println!("    North: {:.3} m/s", state.velocity_north);
                    println!("    East: {:.3} m/s", state.velocity_east);
                    println!("    Down: {:.3} m/s", state.velocity_down);
                    println!("  📐 Orientation:");
                    println!(
                        "    Roll: {:.3} rad ({:.2}°)",
                        state.roll,
                        state.roll.to_degrees()
                    );
                    println!(
                        "    Pitch: {:.3} rad ({:.2}°)",
                        state.pitch,
                        state.pitch.to_degrees()
                    );
                    println!(
                        "    Heading: {:.3} rad ({:.2}°)",
                        state.heading,
                        state.heading.to_degrees()
                    );
                    println!("  🎯 Accelerations:");
                    println!("    Body X: {:.3} m/s²", state.body_acceleration_x);
                    println!("    Body Y: {:.3} m/s²", state.body_acceleration_y);
                    println!("    Body Z: {:.3} m/s²", state.body_acceleration_z);
                    println!("    G-Force: {:.3} g", state.g_force);
                    println!("  🌀 Angular Velocities:");
                    println!("    X: {:.3} rad/s", state.angular_velocity_x);
                    println!("    Y: {:.3} rad/s", state.angular_velocity_y);
                    println!("    Z: {:.3} rad/s", state.angular_velocity_z);
                    println!("  📊 Standard Deviations:");
                    println!(
                        "    Latitude: {:.3} m, Longitude: {:.3} m, Height: {:.3} m",
                        state.latitude_standard_deviation,
                        state.longitude_standard_deviation,
                        state.height_standard_deviation
                    );
                }

                PacketId::Status => {
                    let status = interface.get_status().await?;
                    let system_status = SystemStatus::new(status.system_status);
                    println!("✨ Status:");
                    println!("  🚨 System Status: {}", system_status);
                    println!("  🔧 Filter Status: 0x{:04X}", status.filter_status);
                }

                PacketId::UnixTime => {
                    let time = interface.get_unix_time().await?;
                    println!("✨ Unix Time:");
                    println!("  Seconds: {}", time.unix_time_seconds);
                    println!("  Microseconds: {}", time.microseconds);

                    // Convert to human readable
                    use std::time::UNIX_EPOCH;
                    let system_time =
                        UNIX_EPOCH + Duration::from_secs(time.unix_time_seconds as u64);
                    println!("  Human readable: {:?}", system_time);
                }

                PacketId::FormattedTime => {
                    let time = interface.get_formatted_time().await?;
                    println!("✨ Formatted Time:");
                    println!(
                        "  📅 Date: {}-{:02}-{:02} (Year-Month-Day)",
                        time.year, time.month, time.month_day
                    );
                    println!(
                        "  🕐 Time: {:02}:{:02}:{:02} (Hour:Minute:Second)",
                        time.hour, time.minute, time.second
                    );
                    println!("  📊 Details:");
                    println!("    Year Day: {}", time.year_day);
                    println!("    Week Day: {}", time.week_day);
                    println!("    Microseconds: {}", time.microseconds);
                }

                PacketId::PacketTimerPeriod => {
                    let config = interface.get_packet_timer_period().await?;
                    println!("✨ Packet Timer Period:");
                    println!("  Permanent: {}", config.permanent);
                    println!("  UTC Synchronisation: {}", config.utc_synchronisation);
                    println!(
                        "  Period: {} microseconds ({:.3} ms)",
                        config.packet_timer_period,
                        config.packet_timer_period as f64 / 1000.0
                    );
                    println!("  Rate: {:.2} Hz", config.get_rate_hz());
                }

                PacketId::FilterOptions => {
                    let config = interface.get_filter_options().await?;
                    println!("✨ Filter Options:");
                    println!("  Permanent: {}", config.permanent);
                    if let Some(vehicle_type) = config.get_vehicle_type() {
                        println!("  Vehicle Type: {}", vehicle_type);
                    }
                    println!("  Features: {}", config.get_enabled_features().join(", "));
                }

                PacketId::OdometerConfiguration => {
                    let config = interface.get_odometer_configuration().await?;
                    println!("✨ Odometer Configuration:");
                    println!("  {}", config.get_summary());
                }

                PacketId::ReferencePointOffsets => {
                    let config = interface.get_reference_point_offsets().await?;
                    println!("✨ Reference Point Offsets:");
                    println!("  {}", config.get_summary());
                }

                PacketId::IpDataportsConfiguration => {
                    let config = interface.get_ip_dataports_configuration().await?;
                    println!("✨ IP Dataports Configuration:");
                    println!("  {}", config.get_summary());
                    for (index, entry) in config.get_active_dataports() {
                        println!("  Dataport {}: {}", index + 1, entry);
                    }
                }

                _ => {
                    // For other available packets, use generic packet request
                    let (response_id, raw_data) = interface.request_packet(pid).await?;
                    display_raw_packet_data(response_id, &raw_data);
                }
            }
        }
        None => {
            // This should not happen since we validate packet ID earlier
            println!(
                "⚠️  Unknown packet ID {} - this should not happen!",
                packet_id
            );
            let (response_id, raw_data) = interface.send_packet(1, &[packet_id]).await?;
            display_raw_packet_data(response_id, &raw_data);
        }
    }

    Ok(())
}

fn display_raw_packet_data(packet_id: u8, data: &[u8]) {
    println!("✨ Raw Packet Data:");
    println!("  📋 Response Packet ID: {}", packet_id);
    println!("  📏 Data Length: {} bytes", data.len());

    if !data.is_empty() {
        println!("  📊 Raw Data (hex):");
        for (i, chunk) in data.chunks(16).enumerate() {
            print!("    {:04x}: ", i * 16);
            for byte in chunk {
                print!("{:02x} ", byte);
            }

            // Add ASCII representation
            print!(" |");
            for byte in chunk {
                let c = if byte.is_ascii_graphic() {
                    *byte as char
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("|");
        }

        // Show as little-endian interpretations for common sizes
        if data.len() >= 2 {
            let u16_val = u16::from_le_bytes([data[0], data[1]]);
            println!("  🔢 First 2 bytes as u16 (LE): {}", u16_val);
        }
        if data.len() >= 4 {
            let u32_val = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            let f32_val = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            println!("  🔢 First 4 bytes as u32 (LE): {}", u32_val);
            println!("  🔢 First 4 bytes as f32 (LE): {:.6}", f32_val);
        }
        if data.len() >= 8 {
            let u64_val = u64::from_le_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]);
            let f64_val = f64::from_le_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]);
            println!("  🔢 First 8 bytes as u64 (LE): {}", u64_val);
            println!("  🔢 First 8 bytes as f64 (LE): {:.8}", f64_val);
        }
    }
}
