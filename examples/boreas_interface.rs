//! Example implementation of a TCP interface for Advanced Navigation Boreas D90
//!
//! This example demonstrates how to build an I/O-aware interface on top of the
//! sans-io liban core library for communicating with Boreas D90 devices.

use binrw::{BinRead, BinWrite};
use liban::{AnError, Result};
use liban::packet::PacketId;
use liban::packet::system::{
    AcknowledgePacket, DeviceInformationPacket, ResetPacket, RestoreFactorySettingsPacket,
};
use liban::packet::state::{
    StatusPacket, SystemStatePacket, UnixTimePacket,
};
use liban::packet::config::{
    FilterOptionsPacket, InstallationAlignmentPacket, IpDataportsConfigurationPacket,
    OdometerConfigurationPacket, PacketTimerPeriodPacket, ReferencePointOffsetsPacket,
};
use liban::protocol::AnppProtocol;
use bytes::Buf;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration as TokioDuration};
use tracing::{debug, info, warn};

/// Example TCP interface for communicating with Advanced Navigation Boreas D90 devices
pub struct BoreasInterface {
    stream: Option<TcpStream>,
    address: SocketAddr,
    timeout: Duration,
    connected: bool,
}

impl BoreasInterface {
    /// Create a new Boreas interface
    ///
    /// # Arguments
    /// * `host` - Hostname or IP address of the Boreas device
    /// * `port` - TCP port number (typically 16718 for Boreas D90)
    /// * `timeout_duration` - Timeout for network operations
    ///
    /// # Example
    /// ```rust,no_run
    /// # use boreas_interface::BoreasInterface;
    /// # use std::time::Duration;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut interface = BoreasInterface::new(
    ///     "192.168.1.100",
    ///     16718,
    ///     Duration::from_secs(5)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(host: &str, port: u16, timeout_duration: Duration) -> Result<Self> {
        let address: SocketAddr = format!("{}:{}", host, port).parse().map_err(|e| {
            AnError::Network(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;

        let mut interface = Self {
            stream: None,
            address,
            timeout: timeout_duration,
            connected: false,
        };

        interface.connect().await?;
        Ok(interface)
    }

    /// Establish TCP connection to the Boreas device
    pub async fn connect(&mut self) -> Result<()> {
        info!("Connecting to Boreas device at {}", self.address);

        let stream = timeout(
            TokioDuration::from(self.timeout),
            TcpStream::connect(self.address),
        )
        .await
        .map_err(|_| AnError::Timeout)?
        .map_err(AnError::Network)?;

        self.stream = Some(stream);
        self.connected = true;

        info!("Successfully connected to Boreas device");
        Ok(())
    }

    /// Disconnect from the Boreas device
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            info!("Disconnecting from Boreas device");
            stream.shutdown().await.map_err(AnError::Network)?;
        }
        self.connected = false;
        Ok(())
    }

    /// Check if connected to the device
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Send a raw packet and receive response
    pub async fn send_packet(&mut self, packet_id: PacketId, data: &[u8]) -> Result<(u8, Vec<u8>)> {
        if !self.connected {
            return Err(AnError::NotConnected);
        }

        let stream = self.stream.as_mut().unwrap();

        // Create and send packet using the sans-io protocol
        let packet = AnppProtocol::create_packet(packet_id, data)?;

        debug!("Sending {} bytes to Boreas device", packet.len());
        timeout(TokioDuration::from(self.timeout), stream.write_all(&packet))
            .await
            .map_err(|_| AnError::Timeout)?
            .map_err(AnError::Network)?;

        // Read response
        let mut response_buffer = vec![0u8; 1024];
        let bytes_read = timeout(
            TokioDuration::from(self.timeout),
            stream.read(&mut response_buffer),
        )
        .await
        .map_err(|_| AnError::Timeout)?
        .map_err(AnError::Network)?;

        if bytes_read == 0 {
            warn!("No response received from device");
            return Err(AnError::Device("No response from device".to_string()));
        }

        response_buffer.truncate(bytes_read);
        debug!("Received {} bytes from Boreas device", bytes_read);

        // Parse response packet using the sans-io protocol
        let (header, data) = AnppProtocol::parse_packet(&response_buffer)?;
        Ok((header.packet_id.as_u8(), data))
    }

    /// Request a specific packet from the device
    pub async fn request_packet(&mut self, packet_id: PacketId) -> Result<(u8, Vec<u8>)> {
        let request_data = vec![packet_id.as_u8()];
        self.send_packet(PacketId::new(1), &request_data)
            .await
    }

    /// Generic helper to request a packet and parse the response
    async fn request_and_parse<T>(&mut self, packet_id: PacketId) -> Result<T>
    where
        T: for<'a> BinRead<Args<'a> = ()>,
    {
        let (response_id, data) = self.request_packet(packet_id).await?;

        if response_id != packet_id.as_u8() {
            return Err(AnError::Device(format!(
                "Unexpected response packet ID: expected {}, got {}",
                packet_id.as_u8(),
                response_id
            )));
        }

        let mut cursor = std::io::Cursor::new(&data);
        T::read_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize packet: {}", e)))
    }

    /// Generic helper to send a packet and parse acknowledge response
    async fn send_and_acknowledge<T>(
        &mut self,
        packet_id: PacketId,
        config: T,
    ) -> Result<AcknowledgePacket>
    where
        T: for<'a> BinWrite<Args<'a> = ()>,
    {
        let mut cursor = std::io::Cursor::new(Vec::new());
        config.write_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize config: {}", e)))?;
        let data = cursor.into_inner();

        let (response_id, response_data) = self.send_packet(packet_id, &data).await?;

        if response_id != 0 {
            return Err(AnError::Device(
                "Expected acknowledge packet for configuration".to_string(),
            ));
        }

        let mut cursor = std::io::Cursor::new(&response_data);
        AcknowledgePacket::read_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize AcknowledgePacket: {}", e)))
    }

    /// Get device information
    pub async fn get_device_information(&mut self) -> Result<DeviceInformationPacket> {
        self.request_and_parse(PacketId::new(3)).await
    }

    /// Get system state information
    pub async fn get_system_state(&mut self) -> Result<SystemStatePacket> {
        self.request_and_parse(PacketId::new(20)).await
    }

    /// Get status information
    pub async fn get_status(&mut self) -> Result<StatusPacket> {
        self.request_and_parse(PacketId::new(23)).await
    }

    /// Reset the device
    pub async fn reset_device(&mut self) -> Result<()> {
        info!("Resetting Boreas device");

        let reset_packet = ResetPacket::new();
        let mut cursor = std::io::Cursor::new(Vec::new());
        reset_packet.write_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize ResetPacket: {}", e)))?;
        let packet_data = cursor.into_inner();
        self.send_packet(PacketId::new(5), &packet_data)
            .await?;

        // Device will disconnect after reset
        self.connected = false;
        self.stream = None;

        // Wait for device to reboot
        tokio::time::sleep(Duration::from_secs(30)).await;

        info!("Device reset completed");
        Ok(())
    }

    /// Restore factory settings
    ///
    /// Note: A Factory Reset will re-enable the DHCP Client and lose any static IP address settings.
    pub async fn restore_factory_settings(&mut self) -> Result<()> {
        info!("Restoring factory settings - this will re-enable DHCP and lose static IP settings");

        let factory_reset_packet = RestoreFactorySettingsPacket::new();
        let mut cursor = std::io::Cursor::new(Vec::new());
        factory_reset_packet.write_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize RestoreFactorySettingsPacket: {}", e)))?;
        let packet_data = cursor.into_inner();
        self.send_packet(PacketId::new(4), &packet_data)
            .await?;

        // Device will typically reboot after factory reset
        self.connected = false;
        self.stream = None;

        info!("Factory settings restored");
        Ok(())
    }

    /// Configure IP settings
    pub async fn configure_ip(&mut self, ip_config: &[u8]) -> Result<AcknowledgePacket> {
        let (response_id, data) = self
            .send_packet(PacketId::new(11), ip_config)
            .await?;

        if response_id != 0 {
            return Err(AnError::Device(
                "Expected acknowledge packet for IP configuration".to_string(),
            ));
        }

        if data.len() < 4 {
            return Err(AnError::InvalidPacket(
                "Acknowledge packet too short".to_string(),
            ));
        }

        let mut buf = &data[..];
        Ok(AcknowledgePacket {
            packet_id: buf.get_u8(),
            packet_crc: buf.get_u16_le(),
            result: buf.get_u8(),
        })
    }

    /// Set configuration parameter
    pub async fn set_configuration(&mut self, config_data: &[u8]) -> Result<AcknowledgePacket> {
        // This would send configuration commands specific to Boreas D90
        // The actual packet format would depend on the specific configuration being set
        let (response_id, data) = self.send_packet(PacketId::new(20), config_data).await?; // Using system state packet ID as example

        if response_id != 0 {
            return Err(AnError::Device(
                "Expected acknowledge packet for configuration".to_string(),
            ));
        }

        if data.len() < 4 {
            return Err(AnError::InvalidPacket(
                "Acknowledge packet too short".to_string(),
            ));
        }

        let mut buf = &data[..];
        Ok(AcknowledgePacket {
            packet_id: buf.get_u8(),
            packet_crc: buf.get_u16_le(),
            result: buf.get_u8(),
        })
    }

    /// Get Unix time
    pub async fn get_unix_time(&mut self) -> Result<UnixTimePacket> {
        self.request_and_parse(PacketId::new(21)).await
    }

    // Configuration packets (180-203)
    /// Get packet timer period configuration
    pub async fn get_packet_timer_period(&mut self) -> Result<PacketTimerPeriodPacket> {
        self.request_and_parse(PacketId::new(180)).await
    }

    /// Set packet timer period configuration
    pub async fn set_packet_timer_period(
        &mut self,
        config: PacketTimerPeriodPacket,
    ) -> Result<AcknowledgePacket> {
        self.send_and_acknowledge(PacketId::new(180), config)
            .await
    }

    /// Get installation alignment configuration
    pub async fn get_installation_alignment(&mut self) -> Result<InstallationAlignmentPacket> {
        self.request_and_parse(PacketId::new(185))
            .await
    }

    /// Get filter options configuration
    pub async fn get_filter_options(&mut self) -> Result<FilterOptionsPacket> {
        self.request_and_parse(PacketId::new(186)).await
    }

    /// Get odometer configuration
    pub async fn get_odometer_configuration(&mut self) -> Result<OdometerConfigurationPacket> {
        self.request_and_parse(PacketId::new(192))
            .await
    }

    /// Get reference point offsets configuration
    pub async fn get_reference_point_offsets(&mut self) -> Result<ReferencePointOffsetsPacket> {
        self.request_and_parse(PacketId::new(194))
            .await
    }

    /// Get IP dataports configuration
    pub async fn get_ip_dataports_configuration(
        &mut self,
    ) -> Result<IpDataportsConfigurationPacket> {
        self.request_and_parse(PacketId::new(202))
            .await
    }

    // Generic packet request for other packet types
    /// Generic function to request any packet type by ID
    pub async fn request_generic_packet(&mut self, packet_id: PacketId) -> Result<(u8, Vec<u8>)> {
        self.request_packet(packet_id).await
    }
}

impl Drop for BoreasInterface {
    fn drop(&mut self) {
        if self.connected {
            // Best effort disconnect
            if let Some(mut stream) = self.stream.take() {
                let _ = futures::executor::block_on(stream.shutdown());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Example usage of the BoreasInterface
    let mut interface = BoreasInterface::new(
        "192.168.0.42",
        16720,
        Duration::from_secs(5)
    ).await?;

    // Get device information
    match interface.get_device_information().await {
        Ok(device_info) => {
            println!("Device ID: {}", device_info.device_id);
            println!("Hardware Revision: {}", device_info.hardware_revision);
            println!("Software Version: {}", device_info.software_version);
        }
        Err(e) => eprintln!("Failed to get device information: {}", e),
    }

    // Get system state
    match interface.get_system_state().await {
        Ok(system_state) => {
            println!("System state: {:?}", system_state);
        }
        Err(e) => eprintln!("Failed to get system state: {}", e),
    }

    // Disconnect
    interface.disconnect().await?;

    Ok(())
}