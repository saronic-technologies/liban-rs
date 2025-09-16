use liban::BoreasInterface;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging

    println!("🚀 Testing Boreas Interface...\n");

    // Configuration - adjust these for your device
    let host = "192.168.0.42"; // Replace with your device IP
    let port = 16720; // Standard Boreas D90 port
    let timeout = Duration::from_secs(5);

    // Create interface
    println!("📡 Connecting to Boreas device at {}:{}...", host, port);
    let mut interface = match BoreasInterface::new(host, port, timeout).await {
        Ok(interface) => {
            println!("✅ Successfully connected to device\n");
            interface
        }
        Err(e) => {
            println!("❌ Failed to connect to device: {}", e);
            println!("💡 Make sure the device is reachable at {}:{}", host, port);
            return Err(e.into());
        }
    };

    // Test basic device information
    test_device_info(&mut interface).await?;

    // Test system state
    test_system_state(&mut interface).await?;

    // Test status
    test_status(&mut interface).await?;

    // Test time functions
    test_time_functions(&mut interface).await?;

    // Test configuration functions
    test_configuration(&mut interface).await?;

    // Disconnect
    println!("🔌 Disconnecting from device...");
    interface.disconnect().await?;
    println!("✅ Disconnected successfully\n");

    println!("🎉 All tests completed successfully!");
    Ok(())
}

/// Test device information retrieval
async fn test_device_info(
    interface: &mut BoreasInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing device information...");

    match interface.get_device_information().await {
        Ok(device_info) => {
            println!("📋 Device Information:");
            println!("   Software Version: {}", device_info.software_version);
            println!("   Device ID: {}", device_info.device_id);
            println!("   Hardware Revision: {}", device_info.hardware_revision);
            println!("   Serial Number: {}", device_info.get_serial_number());
            println!(
                "   Serial Number Parts: {} / {} / {}",
                device_info.serial_number_1,
                device_info.serial_number_2,
                device_info.serial_number_3
            );
            println!("✅ Device information retrieved successfully\n");
        }
        Err(e) => {
            println!("❌ Failed to get device information: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

/// Test system state retrieval
async fn test_system_state(
    interface: &mut BoreasInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Testing system state...");

    match interface.get_system_state().await {
        Ok(system_state) => {
            println!("🌍 System State:");
            println!("   System Status: 0x{:04X}", system_state.system_status);
            println!("   Filter Status: 0x{:04X}", system_state.filter_status);
            println!("   Unix Time: {} seconds", system_state.unix_time_seconds);
            println!("   Microseconds: {}", system_state.microseconds);
            println!(
                "   Position: ({:.6}°, {:.6}°, {:.2}m)",
                system_state.latitude, system_state.longitude, system_state.height
            );
            println!(
                "   Velocity (NED): ({:.2}, {:.2}, {:.2}) m/s",
                system_state.velocity_north, system_state.velocity_east, system_state.velocity_down
            );
            println!(
                "   Orientation: Roll={:.2}°, Pitch={:.2}°, Heading={:.2}°",
                system_state.roll, system_state.pitch, system_state.heading
            );

            // Check system status flags
            let system_status = system_state.get_system_status();
            if system_status.is_healthy() {
                println!("✅ System is healthy");
            } else {
                println!(
                    "⚠️  System has active flags: {:?}",
                    system_status.get_active_flags()
                );
            }
            println!("✅ System state retrieved successfully\n");
        }
        Err(e) => {
            println!("❌ Failed to get system state: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

/// Test status packet
async fn test_status(interface: &mut BoreasInterface) -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 Testing status packet...");

    match interface.get_status().await {
        Ok(status) => {
            println!("📊 Status:");
            println!("   System Status: 0x{:04X}", status.system_status);
            println!("   Filter Status: 0x{:04X}", status.filter_status);
            println!("✅ Status retrieved successfully\n");
        }
        Err(e) => {
            println!("❌ Failed to get status: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

/// Test time functions
async fn test_time_functions(
    interface: &mut BoreasInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("⏰ Testing time functions...");

    // Test Unix time
    match interface.get_unix_time().await {
        Ok(unix_time) => {
            println!("🕐 Unix Time:");
            println!("   Seconds: {}", unix_time.unix_time_seconds);
            println!("   Microseconds: {}", unix_time.microseconds);
        }
        Err(e) => {
            println!("⚠️  Failed to get Unix time: {}", e);
        }
    }

    // Test formatted time
    match interface.get_formatted_time().await {
        Ok(formatted_time) => {
            println!("📅 Formatted Time:");
            println!(
                "   Date: {}-{:02}-{:02}",
                formatted_time.year, formatted_time.month, formatted_time.month_day
            );
            println!(
                "   Time: {:02}:{:02}:{:02}",
                formatted_time.hour, formatted_time.minute, formatted_time.second
            );
            println!("   Microseconds: {}", formatted_time.microseconds);
        }
        Err(e) => {
            println!("⚠️  Failed to get formatted time: {}", e);
        }
    }

    println!("✅ Time functions tested\n");
    Ok(())
}

/// Test configuration functions
async fn test_configuration(
    interface: &mut BoreasInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Testing configuration functions...");

    // Test getting packet timer period
    // match interface.get_packet_timer_period().await {
    //     Ok(timer_config) => {
    //         println!("⏲️  Current packet timer period: {} microseconds", timer_config.period_microseconds);

    //         // Test setting packet timer period (set it to the same value)
    //         println!("🔧 Testing packet timer period configuration...");
    //         match interface.set_packet_timer_period(&timer_config).await {
    //             Ok(ack) => {
    //                 println!("✅ Packet timer period set successfully. Ack result: {}", ack.result);
    //             }
    //             Err(e) => {
    //                 println!("⚠️  Failed to set packet timer period: {}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("⚠️  Failed to get packet timer period: {}", e);
    //     }
    // }

    // Test getting packet timer period
    match interface.get_packet_timer_period().await {
        Ok(timer_config) => {
            println!("⏲️  Packet Timer Configuration:");
            println!("   Permanent: {}", timer_config.permanent);
            println!("   UTC Sync: {}", timer_config.utc_synchronisation);
            println!(
                "   Period: {} microseconds",
                timer_config.packet_timer_period
            );
        }
        Err(e) => {
            println!("⚠️  Failed to get packet timer period: {}", e);
        }
    }

    // Test other configuration packets
    test_other_configs(interface).await?;

    println!("✅ Configuration functions tested\n");
    Ok(())
}

/// Test other configuration packets
async fn test_other_configs(
    interface: &mut BoreasInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    // Test odometer configuration
    match interface.get_odometer_configuration().await {
        Ok(odometer_config) => {
            println!("🏃 Odometer Configuration:");
            println!("   Permanent: {}", odometer_config.permanent);
            println!(
                "   Automatic: {}",
                odometer_config.automatic_pulse_measurement_active
            );
            println!(
                "   Pulse length: {:.4} meters",
                odometer_config.pulse_length
            );
        }
        Err(e) => {
            println!("⚠️  Failed to get odometer configuration: {}", e);
        }
    }

    // Test filter options
    match interface.get_filter_options().await {
        Ok(filter_config) => {
            println!("🔍 Filter Options:");
            println!("   Permanent: {}", filter_config.permanent);
            if let Some(vehicle_type) = filter_config.get_vehicle_type() {
                println!("   Vehicle type: {}", vehicle_type);
            }
            println!(
                "   Internal GNSS enabled: {}",
                filter_config.is_internal_gnss_enabled()
            );
            println!(
                "   Velocity heading enabled: {}",
                filter_config.is_velocity_heading_enabled()
            );
            println!(
                "   Motion analysis enabled: {}",
                filter_config.is_motion_analysis_enabled()
            );
        }
        Err(e) => {
            println!("⚠️  Failed to get filter options: {}", e);
        }
    }

    // Test reference point offsets
    match interface.get_reference_point_offsets().await {
        Ok(offsets_config) => {
            println!("📍 Reference Point Offsets:");
            println!("   Permanent: {}", offsets_config.permanent);
            println!(
                "   Primary reference (Heave 1): {}",
                offsets_config.get_primary_reference_offset()
            );
            println!(
                "   COG Lever Arm (Heave 2): {}",
                offsets_config.get_cog_lever_arm_offset()
            );
            println!(
                "   Heave Point 3: {}",
                offsets_config.get_heave_point_3_offset()
            );
            println!(
                "   Heave Point 4: {}",
                offsets_config.get_heave_point_4_offset()
            );
        }
        Err(e) => {
            println!("⚠️  Failed to get reference point offsets: {}", e);
        }
    }

    // Test IP dataports configuration
    match interface.get_ip_dataports_configuration().await {
        Ok(ip_dataports) => {
            println!("🌐 IP Dataports Configuration:");
            println!("   {}", ip_dataports.get_summary());
            for (index, entry) in ip_dataports.get_active_dataports() {
                println!("   Dataport {}: {}", index + 1, entry);
            }
        }
        Err(e) => {
            println!("⚠️  Failed to get IP dataports configuration: {}", e);
        }
    }

    Ok(())
}
