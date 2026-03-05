use liban::Packet;
use clap::Parser as ClapParser;
use std::net::UdpSocket;

#[derive(ClapParser)]
#[command(name = "udp_reader")]
#[command(about = "Read ANPP packets from a UDP socket", long_about = None)]
struct Args {
    /// Bind address
    #[arg(short, long, default_value = "0.0.0.0")]
    bind: String,

    /// Port number
    #[arg(short, long, default_value = "16718")]
    port: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.bind, args.port);
    let socket = UdpSocket::bind(&addr)?;
    println!("Listening on {} ...", addr);

    let mut buffer = [0u8; 4096];
    let mut packet_count: usize = 0;

    loop {
        let (n, src) = socket.recv_from(&mut buffer)?;

        match liban::parse_datagram(&buffer[..n]) {
            Ok(packet) => {
                packet_count += 1;
                match packet {
                    Packet::SystemState(s) => {
                        println!("#{packet_count} [{src}] SystemState: Lat={:.6} Lon={:.6} H={:.2}m Hdg={:.1}",
                                s.latitude.to_degrees(),
                                s.longitude.to_degrees(),
                                s.height,
                                s.heading.to_degrees());
                    }
                    Packet::UnixTime(t) => {
                        println!("#{packet_count} [{src}] UnixTime: {}s + {}us",
                                t.unix_time_seconds, t.microseconds);
                    }
                    Packet::Status(s) => {
                        println!("#{packet_count} [{src}] Status: GNSS={:?}",
                                s.filter_status.gnss_fix_type);
                    }
                    Packet::DeviceInformation(info) => {
                        println!("#{packet_count} [{src}] DeviceInfo: SW=0x{:08X} ID={} HW={}",
                                info.software_version, info.device_id, info.hardware_revision);
                    }
                    Packet::Acknowledge(ack) => {
                        println!("#{packet_count} [{src}] Ack: {:?} -> {:?}",
                                ack.acknowledged_packet, ack.result);
                    }
                    Packet::Unsupported => {
                        println!("#{packet_count} [{src}] Unsupported");
                    }
                    other => {
                        println!("#{packet_count} [{src}] {:?}", other);
                    }
                }
            }
            Err(e) => {
                eprintln!("[{src}] Parse error: {:?}", e);
            }
        }
    }
}
