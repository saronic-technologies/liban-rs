use liban::AnppParser;
use clap::Parser as ClapParser;
use std::io::Read;
use std::net::TcpStream;

#[derive(ClapParser)]
#[command(name = "parse_packets")]
#[command(about = "Parse ANPP packets from a device", long_about = None)]
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

    println!("=== ANPP Packet Parser ===");
    println!("Connecting to {}:{}...\n", args.ip, args.port);

    let mut stream = TcpStream::connect(format!("{}:{}", args.ip, args.port))?;
    println!("Connected!\n");

    let mut parser = AnppParser::new();
    let mut buffer = [0u8; 4096];

    loop {
        // Read data from TCP stream
        let n = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection closed by remote");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from TCP: {}", e);
                break;
            }
        };

        // Feed data to parser and get packets
        while let Some(packet) = parser.consume(&buffer[..n]) {
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Parser returns clean types directly - no conversion needed!
            match packet {
                liban::Packet::SystemState(p) => {
                    println!("SystemState");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("POSITION:");
                    println!("   Latitude:  {:.6}°", p.latitude.to_degrees());
                    println!("   Longitude: {:.6}°", p.longitude.to_degrees());
                    println!("   Height:    {:.2} m", p.height);

                    println!("\nVELOCITY:");
                    println!("   North: {:.2} m/s", p.velocity_north);
                    println!("   East:  {:.2} m/s", p.velocity_east);
                    println!("   Down:  {:.2} m/s", p.velocity_down);

                    println!("\nORIENTATION:");
                    println!("   Roll:    {:.2}°", p.roll.to_degrees());
                    println!("   Pitch:   {:.2}°", p.pitch.to_degrees());
                    println!("   Heading: {:.2}°", p.heading.to_degrees());

                    println!("\nGNSS STATUS:");
                    println!("   Fix Type: {:?}", p.filter_status.gnss_fix_type);
                    println!("   Navigation Initialized: {}", p.filter_status.navigation_filter_initialised);
                    println!("   Heading Initialized: {}", p.filter_status.heading_initialised);

                    // Clean boolean fields instead of bitflags
                    let mut warnings = Vec::new();
                    if p.system_status.system_failure { warnings.push("System Failure"); }
                    if p.system_status.gnss_failure { warnings.push("GNSS Failure"); }
                    if p.system_status.low_voltage_alarm { warnings.push("Low Voltage"); }
                    if p.system_status.high_voltage_alarm { warnings.push("High Voltage"); }
                    if p.system_status.gnss_antenna_disconnected { warnings.push("GNSS Antenna Disconnected"); }

                    if !warnings.is_empty() {
                        println!("\nWARNINGS:");
                        for warning in warnings {
                            println!("   {}", warning);
                        }
                    }
                }

                liban::Packet::UnixTime(p) => {
                    println!("UnixTime");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("TIME: {}s + {}μs", p.unix_time_seconds, p.microseconds);
                }

                liban::Packet::Status(p) => {
                    println!("Status");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STATUS:");
                    println!("   GNSS Fix: {:?}", p.filter_status.gnss_fix_type);

                    // Individual boolean fields
                    let mut warnings = Vec::new();
                    if p.system_status.system_failure { warnings.push("System Failure"); }
                    if p.system_status.gnss_failure { warnings.push("GNSS Failure"); }
                    if p.system_status.low_voltage_alarm { warnings.push("Low Voltage"); }
                    if p.system_status.high_voltage_alarm { warnings.push("High Voltage"); }

                    if warnings.is_empty() {
                        println!("   All systems nominal");
                    } else {
                        println!("   Warnings: {}", warnings.join(", "));
                    }
                }

                liban::Packet::Satellites(p) => {
                    println!("Satellites");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("SATELLITES:");
                    println!("   GPS:     {}", p.gps_satellites);
                    println!("   GLONASS: {}", p.glonass_satellites);
                    println!("   BeiDou:  {}", p.beidou_satellites);
                    println!("   Galileo: {}", p.galileo_satellites);
                    println!("   SBAS:    {}", p.sbas_satellites);
                    println!("   Total:   {}",
                             p.gps_satellites + p.glonass_satellites +
                             p.beidou_satellites + p.galileo_satellites + p.sbas_satellites);
                    println!("\n   HDOP: {:.2}", p.hdop);
                    println!("   VDOP: {:.2}", p.vdop);
                }

                liban::Packet::RawSensors(p) => {
                    println!("RawSensors");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("RAW SENSORS:");
                    println!("   Accelerometer: [{:.3}, {:.3}, {:.3}] m/s²",
                             p.accelerometer_x, p.accelerometer_y, p.accelerometer_z);
                    println!("   Gyroscope:     [{:.3}, {:.3}, {:.3}] rad/s",
                             p.gyroscope_x, p.gyroscope_y, p.gyroscope_z);
                    println!("   IMU Temp:      {:.1}°C", p.imu_temperature);
                    println!("   Pressure:      {:.0} Pa ({:.2} hPa)", p.pressure, p.pressure / 100.0);
                    println!("   Pressure Temp: {:.1}°C", p.pressure_temperature);
                }

                liban::Packet::SensorTemperature(p) => {
                    println!("SensorTemperature");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("SENSOR TEMPERATURES:");
                    println!("   Accelerometer: [{:.1}, {:.1}, {:.1}]°C",
                             p.accelerometer_temp_0, p.accelerometer_temp_1, p.accelerometer_temp_2);
                    println!("   Gyroscope:     [{:.1}, {:.1}, {:.1}]°C",
                             p.gyroscope_temp_0, p.gyroscope_temp_1, p.gyroscope_temp_2);
                    println!("   Pressure:      {:.1}°C", p.pressure_sensor_temp);
                }

                liban::Packet::DeviceInformation(p) => {
                    println!("DeviceInformation");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("DEVICE INFORMATION:");
                    println!("   Software Version: 0x{:08X}", p.software_version);
                    println!("   Device ID:        {}", p.device_id);
                    println!("   Hardware Rev:     {}", p.hardware_revision);
                    println!("   Serial Number:    {:08X}-{:08X}-{:08X}",
                             p.serial_number_1, p.serial_number_2, p.serial_number_3);
                }

                liban::Packet::Acknowledge(p) => {
                    println!("Acknowledge");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("ACKNOWLEDGE:");
                    println!("   Packet: {:?}", p.acknowledged_packet);
                    println!("   Result: {:?}", p.result);
                }

                liban::Packet::EulerOrientationStdDev(p) => {
                    println!("EulerOrientationStdDev");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("ORIENTATION UNCERTAINTY:");
                    println!("   Roll:    {:.4}°", p.roll_std_dev.to_degrees());
                    println!("   Pitch:   {:.4}°", p.pitch_std_dev.to_degrees());
                    println!("   Heading: {:.4}°", p.heading_std_dev.to_degrees());
                }

                liban::Packet::Heave(p) => {
                    println!("Heave");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("HEAVE:");
                    println!("   Points: [{:.2}, {:.2}, {:.2}, {:.2}] m",
                             p.heave_point_1, p.heave_point_2,
                             p.heave_point_3, p.heave_point_4);
                }

                liban::Packet::ExternalTime(p) => {
                    println!("ExternalTime");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("EXTERNAL TIME: {}s + {}μs", p.unix_time_seconds, p.microseconds);
                }

                liban::Packet::Unsupported => {
                    println!("Unsupported packet type");
                }

                _ => {
                    println!("(Config packet - not displayed in this example)");
                }
            }
            println!();
        }
    }

    Ok(())
}
