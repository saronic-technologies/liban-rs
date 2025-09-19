use liban::reader::AnppReader;
use std::env;
use std::fs::File;
use std::io::Result;

/// Example that reads ANPP packets from a binary file.
///
/// Usage:
///     cargo run --example file_reader <filename>
///     cargo run --example file_reader data/anpp_capture.bin
fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        eprintln!("Example: {} data/anpp_capture.bin", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    println!("Opening ANPP file: {}...", filename);

    // Open the file
    let file = match File::open(filename) {
        Ok(file) => {
            println!("Successfully opened {}", filename);
            file
        }
        Err(e) => {
            eprintln!("Failed to open {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    // Create the ANPP reader
    let anpp_reader = AnppReader::new(file);

    println!("Reading ANPP packets from file...");
    println!("---");

    let mut packet_count = 0;
    let mut error_count = 0;
    let mut packet_types = std::collections::HashMap::new();

    // Read and print packets
    for result in anpp_reader {
        match result {
            Ok(packet) => {
                packet_count += 1;

                // Count packet types
                let packet_type = match &packet {
                    liban::AnppPacket::SystemState(_) => "SystemState",
                    liban::AnppPacket::DeviceInformation(_) => "DeviceInformation",
                    liban::AnppPacket::Status(_) => "Status",
                    liban::AnppPacket::UnixTime(_) => "UnixTime",
                    liban::AnppPacket::Acknowledge(_) => "Acknowledge",
                    liban::AnppPacket::Request(_) => "Request",
                    liban::AnppPacket::BootMode(_) => "BootMode",
                    liban::AnppPacket::Reset(_) => "Reset",
                    liban::AnppPacket::RestoreFactorySettings(_) => "RestoreFactorySettings",
                    liban::AnppPacket::IpConfiguration(_) => "IpConfiguration",
                    liban::AnppPacket::PacketTimerPeriod(_) => "PacketTimerPeriod",
                    liban::AnppPacket::PacketsPeriod(_) => "PacketsPeriod",
                    liban::AnppPacket::InstallationAlignment(_) => "InstallationAlignment",
                    liban::AnppPacket::FilterOptions(_) => "FilterOptions",
                    liban::AnppPacket::OdometerConfiguration(_) => "OdometerConfiguration",
                    liban::AnppPacket::SetZeroOrientationAlignment(_) => "SetZeroOrientationAlignment",
                    liban::AnppPacket::ReferencePointOffsets(_) => "ReferencePointOffsets",
                    liban::AnppPacket::IpDataportsConfiguration(_) => "IpDataportsConfiguration",
                    liban::AnppPacket::Unsupported(_) => "Unsupported",
                };
                *packet_types.entry(packet_type).or_insert(0) += 1;

                println!("Packet #{}: {:?}", packet_count, packet);

                // Print packet-specific information for key packet types
                match packet {
                    liban::AnppPacket::SystemState(state) => {
                        println!("  → Position: Lat={:.6}°, Lon={:.6}°, Height={:.2}m",
                                state.latitude.to_degrees(),
                                state.longitude.to_degrees(),
                                state.height);
                        println!("  → Velocity: N={:.2} E={:.2} D={:.2} m/s",
                                state.velocity_north,
                                state.velocity_east,
                                state.velocity_down);
                        println!("  → Attitude: Roll={:.1}°, Pitch={:.1}°, Heading={:.1}°",
                                state.roll.to_degrees(),
                                state.pitch.to_degrees(),
                                state.heading.to_degrees());
                    }
                    liban::AnppPacket::DeviceInformation(info) => {
                        println!("  → Software Version: {}", info.software_version);
                        println!("  → Device ID: {}", info.device_id);
                        println!("  → Hardware Revision: {}", info.hardware_revision);
                        println!("  → Serial Numbers: {}, {}, {}",
                                info.serial_number_1,
                                info.serial_number_2,
                                info.serial_number_3);
                    }
                    liban::AnppPacket::Status(status) => {
                        println!("  → System Status: {:?}", status.system_status);
                        println!("  → Filter Status: {:?}", status.filter_status);
                    }
                    liban::AnppPacket::UnixTime(time) => {
                        println!("  → Unix Time: {}.{:06} seconds",
                                time.unix_time_seconds,
                                time.microseconds);
                    }
                    _ => {
                        // For other packet types, just show the debug output
                    }
                }
                println!();
            }
            Err(e) => {
                error_count += 1;
                eprintln!("I/O Error #{}: {}", error_count, e);

                // For file reading, EOF is expected
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("Reached end of file.");
                    break;
                }
            }
        }
    }

    println!("---");
    println!("File processing summary:");
    println!("  Total packets read: {}", packet_count);
    println!("  Total I/O errors: {}", error_count);
    println!();
    println!("Packet type breakdown:");
    for (packet_type, count) in packet_types.iter() {
        println!("  {}: {} packets", packet_type, count);
    }

    Ok(())
}