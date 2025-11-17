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
                liban::Packet::SystemState(clean) => {
                    println!("SystemState");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("POSITION:");
                    println!("   Latitude:  {:.6}°", clean.latitude.to_degrees());
                    println!("   Longitude: {:.6}°", clean.longitude.to_degrees());
                    println!("   Height:    {:.2} m", clean.height);

                    println!("\nVELOCITY:");
                    println!("   North: {:.2} m/s", clean.velocity_north);
                    println!("   East:  {:.2} m/s", clean.velocity_east);
                    println!("   Down:  {:.2} m/s", clean.velocity_down);

                    println!("\nORIENTATION:");
                    println!("   Roll:    {:.2}°", clean.roll.to_degrees());
                    println!("   Pitch:   {:.2}°", clean.pitch.to_degrees());
                    println!("   Heading: {:.2}°", clean.heading.to_degrees());

                    println!("\nGNSS STATUS:");
                    println!("   Fix Type: {:?}", clean.filter_status.gnss_fix_type);
                    println!("   Navigation Initialized: {}", clean.filter_status.navigation_filter_initialised);
                    println!("   Heading Initialized: {}", clean.filter_status.heading_initialised);

                    // Clean boolean fields instead of bitflags
                    let mut warnings = Vec::new();
                    if clean.system_status.system_failure { warnings.push("System Failure"); }
                    if clean.system_status.gnss_failure { warnings.push("GNSS Failure"); }
                    if clean.system_status.low_voltage_alarm { warnings.push("Low Voltage"); }
                    if clean.system_status.high_voltage_alarm { warnings.push("High Voltage"); }
                    if clean.system_status.gnss_antenna_disconnected { warnings.push("GNSS Antenna Disconnected"); }

                    if !warnings.is_empty() {
                        println!("\nWARNINGS:");
                        for warning in warnings {
                            println!("   {}", warning);
                        }
                    }
                }

                liban::Packet::UnixTime(clean) => {
                    println!("UnixTime");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("TIME: {}s + {}μs", clean.unix_time_seconds, clean.microseconds);
                }

                liban::Packet::Status(clean) => {
                    println!("Status");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STATUS:");
                    println!("   GNSS Fix: {:?}", clean.filter_status.gnss_fix_type);

                    // Individual boolean fields
                    let mut warnings = Vec::new();
                    if clean.system_status.system_failure { warnings.push("System Failure"); }
                    if clean.system_status.gnss_failure { warnings.push("GNSS Failure"); }
                    if clean.system_status.low_voltage_alarm { warnings.push("Low Voltage"); }
                    if clean.system_status.high_voltage_alarm { warnings.push("High Voltage"); }

                    if warnings.is_empty() {
                        println!("   All systems nominal");
                    } else {
                        println!("   Warnings: {}", warnings.join(", "));
                    }
                }

                liban::Packet::Satellites(clean) => {
                    println!("Satellites");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("SATELLITES:");
                    println!("   GPS:     {}", clean.gps_satellites);
                    println!("   GLONASS: {}", clean.glonass_satellites);
                    println!("   BeiDou:  {}", clean.beidou_satellites);
                    println!("   Galileo: {}", clean.galileo_satellites);
                    println!("   SBAS:    {}", clean.sbas_satellites);
                    println!("   Total:   {}",
                             clean.gps_satellites + clean.glonass_satellites +
                             clean.beidou_satellites + clean.galileo_satellites + clean.sbas_satellites);
                    println!("\n   HDOP: {:.2}", clean.hdop);
                    println!("   VDOP: {:.2}", clean.vdop);
                }

                liban::Packet::RawSensors(clean) => {
                    println!("RawSensors");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("RAW SENSORS:");
                    println!("   Accelerometer: [{:.3}, {:.3}, {:.3}] m/s²",
                             clean.accelerometer_x, clean.accelerometer_y, clean.accelerometer_z);
                    println!("   Gyroscope:     [{:.3}, {:.3}, {:.3}] rad/s",
                             clean.gyroscope_x, clean.gyroscope_y, clean.gyroscope_z);
                    println!("   IMU Temp:      {:.1}°C", clean.imu_temperature);
                    println!("   Pressure:      {:.0} Pa ({:.2} hPa)", clean.pressure, clean.pressure / 100.0);
                    println!("   Pressure Temp: {:.1}°C", clean.pressure_temperature);
                    // Note: No reserved fields exposed in clean API!
                }

                liban::Packet::SensorTemperature(clean) => {
                    println!("SensorTemperature");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("SENSOR TEMPERATURES:");
                    println!("   Accelerometer: [{:.1}, {:.1}, {:.1}]°C",
                             clean.accelerometer_temp_0, clean.accelerometer_temp_1, clean.accelerometer_temp_2);
                    println!("   Gyroscope:     [{:.1}, {:.1}, {:.1}]°C",
                             clean.gyroscope_temp_0, clean.gyroscope_temp_1, clean.gyroscope_temp_2);
                    println!("   Pressure:      {:.1}°C", clean.pressure_sensor_temp);
                    // Note: No reserved field in clean API!
                }

                liban::Packet::DeviceInformation(clean) => {
                    println!("DeviceInformation");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("DEVICE INFORMATION:");
                    println!("   Software Version: 0x{:08X}", clean.software_version);
                    println!("   Device ID:        {}", clean.device_id);
                    println!("   Hardware Rev:     {}", clean.hardware_revision);
                    println!("   Serial Number:    {:08X}-{:08X}-{:08X}",
                             clean.serial_number_1, clean.serial_number_2, clean.serial_number_3);
                }

                liban::Packet::Acknowledge(clean) => {
                    println!("Acknowledge");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("ACKNOWLEDGE:");
                    println!("   Packet: {:?}", clean.acknowledged_packet);
                    println!("   Result: {:?}", clean.result);
                }

                liban::Packet::EulerOrientationStdDev(clean) => {
                    println!("EulerOrientationStdDev");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("ORIENTATION UNCERTAINTY:");
                    println!("   Roll:    {:.4}°", clean.roll_std_dev.to_degrees());
                    println!("   Pitch:   {:.4}°", clean.pitch_std_dev.to_degrees());
                    println!("   Heading: {:.4}°", clean.heading_std_dev.to_degrees());
                }

                liban::Packet::Heave(clean) => {
                    println!("Heave");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("HEAVE:");
                    println!("   Points: [{:.2}, {:.2}, {:.2}, {:.2}] m",
                             clean.heave_point_1, clean.heave_point_2,
                             clean.heave_point_3, clean.heave_point_4);
                }

                liban::Packet::ExternalTime(clean) => {
                    println!("ExternalTime");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("EXTERNAL TIME: {}s + {}μs", clean.unix_time_seconds, clean.microseconds);
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
