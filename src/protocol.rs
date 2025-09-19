use crate::error::{AnError, Result};
use crate::packet::{PacketId, AnppHeader, AnppPacket};
use crc::{Crc, Algorithm};
use binrw::{BinRead, BinWrite};
use std::io::Cursor;

/// Advanced Navigation Packet Protocol implementation
pub struct AnppProtocol;

impl AnppProtocol {
    /// CRC16-CCITT calculator instance
    const CRC16: Crc<u16> = Crc::<u16>::new(&Algorithm {
        width: 16,
        poly: 0x1021,
        init: 0xFFFF,
        refin: false,
        refout: false,
        xorout: 0x0000,
        check: 0x29B1,
        residue: 0x0000,
    });

    /// Calculate CRC16-CCITT checksum for packet data
    pub fn calculate_crc16(data: &[u8]) -> u16 {
        Self::CRC16.checksum(data)
    }

    /// Create an ANPP packet from structured components
    pub fn create_packet(packet_id: PacketId, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() > 255 {
            return Err(AnError::PacketTooLong(data.len()));
        }

        let length = data.len() as u8;
        let crc16 = Self::calculate_crc16(data);

        // Calculate Header LRC
        let crc_bytes = crc16.to_le_bytes();
        let mut header_lrc = ((packet_id.as_u8() as u16 + length as u16
                             + crc_bytes[0] as u16 + crc_bytes[1] as u16) ^ 0xFF) + 1;
        if header_lrc > 255 {
            header_lrc %= 256;
        }

        // Build header
        let header = AnppHeader {
            header_lrc: header_lrc as u8,
            packet_id,
            length,
            crc16,
        };

        // Serialize header and append data
        let mut packet = Self::serialize_header(&header)?;
        packet.extend_from_slice(data);

        Ok(packet)
    }

    /// Parse an ANPP packet and return structured header with data
    pub fn parse_packet(packet: &[u8]) -> Result<(AnppHeader, Vec<u8>)> {
        if packet.len() < 5 {
            return Err(AnError::InvalidPacket(
                "Packet too short (minimum 5 bytes)".to_string(),
            ));
        }

        // Extract header
        let header = Self::deserialize_header(&packet[..5])?;

        // Validate packet length
        let expected_length = 5 + header.length as usize;
        if packet.len() != expected_length {
            return Err(AnError::InvalidLength {
                expected: expected_length,
                actual: packet.len(),
            });
        }

        // Extract data
        let data = if header.length > 0 {
            packet[5..expected_length].to_vec()
        } else {
            Vec::new()
        };

        // Validate header against data
        Self::validate_header(&header, &data)?;

        Ok((header, data))
    }

    /// Serialize header to bytes using binrw
    pub fn serialize_header(header: &AnppHeader) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());
        header.write_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to serialize header: {}", e)))?;
        Ok(cursor.into_inner())
    }

    /// Deserialize header from bytes using binrw
    pub fn deserialize_header(bytes: &[u8]) -> Result<AnppHeader> {
        let mut cursor = Cursor::new(bytes);
        AnppHeader::read_le(&mut cursor)
            .map_err(|e| AnError::InvalidPacket(format!("Failed to deserialize header: {}", e)))
    }

    /// Validate header against data payload
    pub fn validate_header(header: &AnppHeader, data: &[u8]) -> Result<()> {
        // Validate length
        if header.length as usize != data.len() {
            return Err(AnError::InvalidLength {
                expected: header.length as usize,
                actual: data.len(),
            });
        }

        // Validate CRC
        let calculated_crc = Self::calculate_crc16(data);
        if header.crc16 != calculated_crc {
            return Err(AnError::InvalidChecksum);
        }

        // Validate header LRC
        let crc_bytes = header.crc16.to_le_bytes();
        let mut expected_lrc = ((header.packet_id.as_u8() as u16 + header.length as u16
                               + crc_bytes[0] as u16 + crc_bytes[1] as u16) ^ 0xFF) + 1;
        if expected_lrc > 255 {
            expected_lrc %= 256;
        }

        if header.header_lrc != expected_lrc as u8 {
            return Err(AnError::InvalidPacket(format!(
                "Header LRC mismatch: expected 0x{:02X}, got 0x{:02X}",
                expected_lrc, header.header_lrc
            )));
        }

        Ok(())
    }

    /// Create an ANPP packet from AnppPacket enum
    pub fn create_anpp_packet(packet: AnppPacket) -> Result<Vec<u8>> {
        let packet_id = PacketId::new(packet.packet_id());
        let data = packet.to_bytes()?;
        Self::create_packet(packet_id, &data)
    }

    /// Parse raw bytes into AnppPacket enum
    pub fn parse_anpp_packet(packet: &[u8]) -> Result<AnppPacket> {
        let (header, data) = Self::parse_packet(packet)?;
        AnppPacket::from_bytes(header.packet_id.as_u8(), &data)
    }

    /// Create a request packet for the specified packet ID
    pub fn create_request_packet(requested_packet_id: PacketId) -> Result<Vec<u8>> {
        let request_data = vec![requested_packet_id.as_u8()];
        Self::create_packet(PacketId::new(1), &request_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_calculation() {
        // Test with empty data
        let crc_empty = AnppProtocol::calculate_crc16(&[]);
        assert_eq!(crc_empty, 0xFFFF);

        // Test with known data
        let test_data = b"123456789";
        let crc = AnppProtocol::calculate_crc16(test_data);
        // CRC16-CCITT(false) of "123456789" should be 0x29B1
        assert_eq!(crc, 0x29B1);
    }

    #[test]
    fn test_packet_creation_and_parsing() {
        let test_data = vec![0x01, 0x02, 0x03, 0x04];
        let packet_id = PacketId::new(20);

        // Create a packet
        let packet = AnppProtocol::create_packet(packet_id, &test_data).unwrap();

        // Parse it back
        let (header, parsed_data) = AnppProtocol::parse_packet(&packet).unwrap();

        assert_eq!(header.packet_id.as_u8(), 20);
        assert_eq!(header.length, test_data.len() as u8);
        assert_eq!(parsed_data, test_data);
    }

    #[test]
    fn test_request_packet_creation() {
        let requested_id = PacketId::new(3);
        let request_packet = AnppProtocol::create_request_packet(requested_id).unwrap();

        // Should be a request packet (ID 1) containing the requested packet ID
        let (header, data) = AnppProtocol::parse_packet(&request_packet).unwrap();
        assert_eq!(header.packet_id.as_u8(), 1); // Request packet ID
        assert_eq!(data, vec![3]); // Requested packet ID
    }

    #[test]
    fn test_header_serialization() {
        let header = AnppHeader {
            header_lrc: 0xAB,
            packet_id: PacketId::new(20),
            length: 4,
            crc16: 0x1234,
        };

        // Serialize and deserialize
        let bytes = AnppProtocol::serialize_header(&header).unwrap();
        let deserialized = AnppProtocol::deserialize_header(&bytes).unwrap();

        assert_eq!(header.header_lrc, deserialized.header_lrc);
        assert_eq!(header.packet_id.as_u8(), deserialized.packet_id.as_u8());
        assert_eq!(header.length, deserialized.length);
        assert_eq!(header.crc16, deserialized.crc16);
    }

    #[test]
    fn test_header_validation() {
        let test_data = vec![0x01, 0x02, 0x03, 0x04];
        let packet_id = PacketId::new(20);

        // Create a packet
        let packet = AnppProtocol::create_packet(packet_id, &test_data).unwrap();

        // Parse and validate
        let (header, parsed_data) = AnppProtocol::parse_packet(&packet).unwrap();
        AnppProtocol::validate_header(&header, &parsed_data).unwrap();

        assert_eq!(header.packet_id.as_u8(), 20);
        assert_eq!(header.length, test_data.len() as u8);
        assert_eq!(parsed_data, test_data);
    }
}
