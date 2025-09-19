use liban::reader::AnppReader;
use std::env;
use std::net::TcpStream;
use std::io::Result;

/// Example that connects to a TCP stream and reads ANPP packets.
///
/// Usage:
///     cargo run --example stream_reader <host> <port>
///     cargo run --example stream_reader 192.168.1.100 55555
///     cargo run --example stream_reader localhost 8080
fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <host> <port>", args[0]);
        eprintln!("Example: {} 192.168.1.100 55555", args[0]);
        std::process::exit(1);
    }

    let host = &args[1];
    let port = &args[2];
    let address = format!("{}:{}", host, port);

    println!("Connecting to ANPP stream at {}...", address);


    // let stream: Box<dyn std::io::Read> = {
    //     let ip_port = format!("{}:{}", (*IP).as_str(), (*PORT).to_string().as_str());
    //     info!("Connecting to septentrio on: {ip_port}...");
    //     Box::new(TcpStream::connect(ip_port)?)
    // };
    // Connect to the TCP stream
    let stream = Box::new(TcpStream::connect(&address)?);

    // Create the ANPP reader
    let anpp_reader = AnppReader::new(stream);

    println!("Reading ANPP packets... (Press Ctrl+C to stop)");
    println!("---");

    let mut packet_count = 0;
    let mut error_count = 0;

    // Read and print packets
    for result in anpp_reader {
        match result {
            Ok(packet) => {
                packet_count += 1;
                println!("Packet #{}: {:?}", packet_count, packet);

                // Print packet-specific information
                match packet {
                    liban::AnppPacket::SystemState(state) => {
                        println!("  → System State: Lat={:.6}°, Lon={:.6}°, Height={:.2}m",
                                state.latitude.to_degrees(),
                                state.longitude.to_degrees(),
                                state.height);
                    }
                    liban::AnppPacket::DeviceInformation(info) => {
                        println!("  → Device Info: SW v{}, Device ID: {}, HW Rev: {}",
                                info.software_version,
                                info.device_id,
                                info.hardware_revision);
                    }
                    liban::AnppPacket::Status(status) => {
                        println!("  → Status: System={:?}, Filter={:?}",
                                status.system_status,
                                status.filter_status);
                    }
                    liban::AnppPacket::UnixTime(time) => {
                        println!("  → Time: {} seconds, {} microseconds",
                                time.unix_time_seconds,
                                time.microseconds);
                    }
                    liban::AnppPacket::Acknowledge(ack) => {
                        println!("  → Acknowledge: Packet ID {}, Result: {}",
                                ack.packet_id,
                                ack.result);
                    }
                    liban::AnppPacket::Unsupported(data) => {
                        println!("  → Unsupported packet: {} bytes", data.len());
                    }
                    _ => {
                        println!("  → Other packet type");
                    }
                }
                println!();
            }
            Err(e) => {
                error_count += 1;
                eprintln!("I/O Error #{}: {}", error_count, e);

                // Break on certain types of errors
                if e.kind() == std::io::ErrorKind::UnexpectedEof
                    || e.kind() == std::io::ErrorKind::ConnectionAborted
                    || e.kind() == std::io::ErrorKind::ConnectionReset {
                    eprintln!("Connection lost, exiting...");
                    break;
                }
            }
        }
    }

    println!("---");
    println!("Session summary:");
    println!("  Total packets received: {}", packet_count);
    println!("  Total I/O errors: {}", error_count);

    Ok(())
}