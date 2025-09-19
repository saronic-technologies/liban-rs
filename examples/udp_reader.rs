use liban::parser::AnppParser;
use std::env;
use std::net::UdpSocket;
use std::io::Result;

/// Example that binds to a UDP port and reads ANPP packets.
///
/// Usage:
///     cargo run --example udp_reader <bind_port>
///     cargo run --example udp_reader 55555
///     cargo run --example udp_reader 0.0.0.0:55555
fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bind_address>", args[0]);
        eprintln!("Example: {} 55555", args[0]);
        eprintln!("Example: {} 0.0.0.0:55555", args[0]);
        std::process::exit(1);
    }

    let bind_address = &args[1];

    // Handle both "port" and "host:port" formats
    let full_address = if bind_address.contains(':') {
        bind_address.to_string()
    } else {
        format!("0.0.0.0:{}", bind_address)
    };

    println!("Binding UDP socket to {}...", full_address);

    // Bind to the UDP socket
    let socket = match UdpSocket::bind(&full_address) {
        Ok(socket) => {
            println!("Successfully bound to {}", full_address);
            socket
        }
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", full_address, e);
            std::process::exit(1);
        }
    };

    // Create the ANPP parser (UDP doesn't provide a continuous stream like TCP)
    let mut parser = AnppParser::new();
    let mut buffer = [0u8; 4096]; // Buffer for UDP packets

    println!("Listening for ANPP packets on UDP... (Press Ctrl+C to stop)");
    println!("---");

    let mut packet_count = 0;
    let mut datagram_count = 0;
    let mut error_count = 0;

    loop {
        // Receive UDP datagram
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, source_addr)) => {
                datagram_count += 1;
                println!("Datagram #{} from {}: {} bytes",
                        datagram_count, source_addr, bytes_received);

                // Process the received data through the parser
                if let Some(packet) = parser.consume(&buffer[..bytes_received]) {
                    packet_count += 1;
                    println!("  Packet #{}: {:?}", packet_count, packet);

                    // Print packet-specific information
                    match packet {
                        liban::AnppPacket::SystemState(state) => {
                            println!("    → System State: Lat={:.6}°, Lon={:.6}°, Height={:.2}m",
                                    state.latitude.to_degrees(),
                                    state.longitude.to_degrees(),
                                    state.height);
                        }
                        liban::AnppPacket::DeviceInformation(info) => {
                            println!("    → Device Info: SW v{}, Device ID: {}, HW Rev: {}",
                                    info.software_version,
                                    info.device_id,
                                    info.hardware_revision);
                        }
                        liban::AnppPacket::Status(status) => {
                            println!("    → Status: System={:?}, Filter={:?}",
                                    status.system_status,
                                    status.filter_status);
                        }
                        liban::AnppPacket::UnixTime(time) => {
                            println!("    → Time: {} seconds, {} microseconds",
                                    time.unix_time_seconds,
                                    time.microseconds);
                        }
                        liban::AnppPacket::Acknowledge(ack) => {
                            println!("    → Acknowledge: Packet ID {}, Result: {}",
                                    ack.packet_id,
                                    ack.result);
                        }
                        liban::AnppPacket::Unsupported(data) => {
                            println!("    → Unsupported packet: {} bytes", data.len());
                        }
                        _ => {
                            println!("    → Other packet type");
                        }
                    }
                } else {
                    println!("  No valid ANPP packet found in this datagram");
                }
                println!();
            }
            Err(e) => {
                error_count += 1;
                eprintln!("UDP Error #{}: {}", error_count, e);

                // For UDP, most errors are recoverable, so continue listening
                if error_count > 10 {
                    eprintln!("Too many consecutive errors, exiting...");
                    break;
                }
            }
        }
    }

    println!("---");
    println!("Session summary:");
    println!("  Total UDP datagrams received: {}", datagram_count);
    println!("  Total ANPP packets parsed: {}", packet_count);
    println!("  Total UDP errors: {}", error_count);

    Ok(())
}