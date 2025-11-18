use liban::{AnppParser, Packet, Request, PacketKind};
use clap::Parser as ClapParser;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

#[derive(ClapParser)]
#[command(name = "read_config")]
#[command(about = "Read configuration from an ANPP device", long_about = None)]
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

    println!("=== ANPP Config Reader ===");
    println!("Connecting to {}:{}...\n", args.ip, args.port);

    let mut stream = TcpStream::connect(format!("{}:{}", args.ip, args.port))?;
    println!("Connected!\n");

    let mut parser = AnppParser::new();
    let mut buffer = [0u8; 4096];

    // Request various configuration packets
    let configs_to_request = vec![
        ("Device Information", PacketKind::DeviceInformation),
        ("Filter Options", PacketKind::FilterOptions),
        ("Installation Alignment", PacketKind::InstallationAlignment),
        ("IP Configuration", PacketKind::IpConfiguration),
        ("Packet Timer Period", PacketKind::PacketTimerPeriod),
        ("Packets Period", PacketKind::PacketsPeriod),
        ("Odometer Configuration", PacketKind::OdometerConfiguration),
        ("Reference Point Offsets", PacketKind::ReferencePointOffsets),
        ("IP Dataports Configuration", PacketKind::IpDataportsConfiguration),
    ];

    println!("Requesting configurations...");
    for (name, packet_kind) in &configs_to_request {
        let request = Request {
            requested_packet: *packet_kind,
        };
        let packet = Packet::Request(request);
        let bytes = packet.to_bytes()?;
        stream.write_all(&bytes)?;
        println!("  Requested: {}", name);
        thread::sleep(Duration::from_millis(5));
    }
    println!();

    // Parse responses with short timeout per read
    let timeout = Duration::from_millis(200);
    stream.set_read_timeout(Some(timeout))?;

    println!("Reading responses...\n");

    let mut received_configs = std::collections::HashSet::new();
    let total_requested = configs_to_request.len();
    let mut consecutive_timeouts = 0;

    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                consecutive_timeouts = 0; // Reset on successful read
                n
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                consecutive_timeouts += 1;
                if consecutive_timeouts >= 3 {
                    println!("No more data available");
                    println!("Received {}/{} configurations", received_configs.len(), total_requested);
                    println!("\nNote: Some configs only respond if non-default values are set");
                    break;
                }
                continue;
            }
            Err(e) => {
                eprintln!("Error reading: {}", e);
                break;
            }
        };

        while let Some(packet) = parser.consume(&buffer[..n]) {
            match packet {
                Packet::DeviceInformation(info) => {
                    if received_configs.insert("DeviceInformation") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("DEVICE INFORMATION");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Software Version: 0x{:08X}", info.software_version);
                        println!("  Device ID:        {}", info.device_id);
                        println!("  Hardware Rev:     {}", info.hardware_revision);
                        println!("  Serial Number:    {:08X}-{:08X}-{:08X}",
                                 info.serial_number_1, info.serial_number_2, info.serial_number_3);
                        println!();
                    }
                }

                Packet::FilterOptions(opts) => {
                    if received_configs.insert("FilterOptions") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("FILTER OPTIONS");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent:                    {}", opts.permanent);
                        println!("  Vehicle Type:                 {:?}", opts.vehicle_type);
                        println!("  Internal GNSS Enabled:        {}", opts.internal_gnss_enabled);
                        println!("  Atmospheric Altitude Enabled: {}", opts.atmospheric_altitude_enabled);
                        println!("  Velocity Heading Enabled:     {}", opts.velocity_heading_enabled);
                        println!("  Reversing Detection Enabled:  {}", opts.reversing_detection_enabled);
                        println!("  Motion Analysis Enabled:      {}", opts.motion_analysis_enabled);
                        println!();
                    }
                }

                Packet::InstallationAlignment(align) => {
                    if received_configs.insert("InstallationAlignment") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("INSTALLATION ALIGNMENT");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent: {}", align.permanent);
                        println!("  Alignment DCM:");
                        for row in &align.alignment_dcm {
                            println!("    [{:.6}, {:.6}, {:.6}]", row[0], row[1], row[2]);
                        }
                        println!("  GNSS Antenna Offset: ({:.3}, {:.3}, {:.3}) m",
                                 align.gnss_antenna_offset.x,
                                 align.gnss_antenna_offset.y,
                                 align.gnss_antenna_offset.z);
                        println!("  Odometer Offset:     ({:.3}, {:.3}, {:.3}) m",
                                 align.odometer_offset.x,
                                 align.odometer_offset.y,
                                 align.odometer_offset.z);
                        println!("  External Data Offset: ({:.3}, {:.3}, {:.3}) m",
                                 align.external_data_offset.x,
                                 align.external_data_offset.y,
                                 align.external_data_offset.z);
                        println!();
                    }
                }

                Packet::IpConfiguration(ip_config) => {
                    if received_configs.insert("IpConfiguration") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("IP CONFIGURATION");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent:   {}", ip_config.permanent);
                        println!("  DHCP Mode:   {}", ip_config.dhcp_mode);
                        println!("  IP Address:  {}.{}.{}.{}",
                                 (ip_config.ip_address >> 24) & 0xFF,
                                 (ip_config.ip_address >> 16) & 0xFF,
                                 (ip_config.ip_address >> 8) & 0xFF,
                                 ip_config.ip_address & 0xFF);
                        println!("  Netmask:     {}.{}.{}.{}",
                                 (ip_config.ip_netmask >> 24) & 0xFF,
                                 (ip_config.ip_netmask >> 16) & 0xFF,
                                 (ip_config.ip_netmask >> 8) & 0xFF,
                                 ip_config.ip_netmask & 0xFF);
                        println!("  Gateway:     {}.{}.{}.{}",
                                 (ip_config.ip_gateway >> 24) & 0xFF,
                                 (ip_config.ip_gateway >> 16) & 0xFF,
                                 (ip_config.ip_gateway >> 8) & 0xFF,
                                 ip_config.ip_gateway & 0xFF);
                        println!("  DNS Server:  {}.{}.{}.{}",
                                 (ip_config.dns_server >> 24) & 0xFF,
                                 (ip_config.dns_server >> 16) & 0xFF,
                                 (ip_config.dns_server >> 8) & 0xFF,
                                 ip_config.dns_server & 0xFF);
                        println!();
                    }
                }

                Packet::PacketTimerPeriod(timer) => {
                    if received_configs.insert("PacketTimerPeriod") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("PACKET TIMER PERIOD");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent:            {}", timer.permanent);
                        println!("  UTC Synchronisation:  {}", timer.utc_synchronisation);
                        println!("  Packet Timer Period:  {:?}", timer.packet_timer_period);
                        println!();
                    }
                }

                Packet::PacketsPeriod(periods) => {
                    if received_configs.insert("PacketsPeriod") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("PACKETS PERIOD");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent:      {}", periods.permanent);
                        println!("  Clear Existing: {}", periods.clear_existing);
                        println!("  Configured Packets:");
                        for period in &periods.packet_periods {
                            println!("    {:?}: {:?}", period.packet_type, period.period);
                        }
                        println!();
                    }
                }

                Packet::OdometerConfiguration(odom) => {
                    if received_configs.insert("OdometerConfiguration") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("ODOMETER CONFIGURATION");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent:                   {}", odom.permanent);
                        println!("  Automatic Pulse Measurement: {}", odom.automatic_pulse_measurement);
                        println!("  Pulse Length:                {:.3} m", odom.pulse_length);
                        println!();
                    }
                }

                Packet::ReferencePointOffsets(offsets) => {
                    if received_configs.insert("ReferencePointOffsets") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("REFERENCE POINT OFFSETS");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("  Permanent: {}", offsets.permanent);
                        println!("  Heave Point 1 (Primary): ({:.3}, {:.3}, {:.3}) m",
                                 offsets.heave_point_1.x, offsets.heave_point_1.y, offsets.heave_point_1.z);
                        println!("  Heave Point 2 (COG):     ({:.3}, {:.3}, {:.3}) m",
                                 offsets.heave_point_2.x, offsets.heave_point_2.y, offsets.heave_point_2.z);
                        println!("  Heave Point 3:           ({:.3}, {:.3}, {:.3}) m",
                                 offsets.heave_point_3.x, offsets.heave_point_3.y, offsets.heave_point_3.z);
                        println!("  Heave Point 4:           ({:.3}, {:.3}, {:.3}) m",
                                 offsets.heave_point_4.x, offsets.heave_point_4.y, offsets.heave_point_4.z);
                        println!();
                    }
                }

                Packet::IpDataportsConfiguration(dataports) => {
                    if received_configs.insert("IpDataportsConfiguration") {
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("IP DATAPORTS CONFIGURATION");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        for (i, dataport) in dataports.dataports.iter().enumerate() {
                            println!("  Dataport {}:", i);
                            println!("    IP:   {}.{}.{}.{}",
                                     (dataport.ip_address >> 24) & 0xFF,
                                     (dataport.ip_address >> 16) & 0xFF,
                                     (dataport.ip_address >> 8) & 0xFF,
                                     dataport.ip_address & 0xFF);
                            println!("    Port: {}", dataport.port);
                            println!("    Mode: {:?}", dataport.mode);
                        }
                        println!();
                    }
                }

                Packet::Acknowledge(ack) => {
                    println!("  Acknowledge: {:?} -> {:?}", ack.acknowledged_packet, ack.result);
                }

                _ => {
                    // Ignore streaming telemetry packets like SystemState, UnixTime, etc.
                }
            }

            // Exit once we've received all configs
            if received_configs.len() >= total_requested {
                println!("\nReceived all {} requested configurations. Exiting.", total_requested);
                return Ok(());
            }
        }
    }

    Ok(())
}
