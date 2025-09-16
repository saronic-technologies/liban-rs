use crate::error::{AnError, Result};
use bytes::{Buf, BufMut, BytesMut};
use crc::{Crc, Algorithm};

/// Advanced Navigation Packet Protocol implementation
pub struct AnppProtocol;

impl AnppProtocol {
    /// CRC16-CCITT algorithm definition
    const CRC16_CCITT: Algorithm<u16> = Algorithm {
        width: 16,
        poly: 0x1021,
        init: 0xFFFF,
        refin: false,
        refout: false,
        xorout: 0x0000,
        check: 0x29B1,
        residue: 0x0000,
    };

    /// CRC16-CCITT calculator instance
    const CRC16: Crc<u16> = Crc::<u16>::new(&Self::CRC16_CCITT);

    /// Calculate CRC16-CCITT checksum for packet data
    ///
    /// Uses polynomial 0x1021 with initial value 0xFFFF
    pub fn calculate_crc16(data: &[u8]) -> u16 {
        Self::CRC16.checksum(data)
    }

    /// Create an ANPP packet with proper header and checksum
    ///
    /// Packet format: [Header LRC][ID][Length][CRC16][Data]
    /// Header LRC = (PacketID + Length + CRC0 + CRC1) XOR 0xFF + 1
    pub fn create_packet(packet_id: u8, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() > 255 {
            return Err(AnError::PacketTooLong(data.len()));
        }

        let length = data.len() as u8;
        let crc = Self::calculate_crc16(data);

        // Convert CRC to little-endian bytes
        let mut crc_buf = BytesMut::with_capacity(2);
        crc_buf.put_u16_le(crc);
        let crc_bytes = crc_buf.freeze();
        let crc0 = crc_bytes[0];
        let crc1 = crc_bytes[1];

        // Calculate Header LRC: (PacketID + PacketLength + crc0 + crc1) XOR 0xFF + 1
        let mut header_lrc =
            ((packet_id as u16 + length as u16 + crc0 as u16 + crc1 as u16) ^ 0xFF) + 1;
        if header_lrc > 255 {
            header_lrc %= 256;
        }

        // Build packet: Header LRC + ID + Length + CRC16 + Data
        let mut packet = Vec::with_capacity(5 + data.len());
        packet.push(header_lrc as u8);
        packet.push(packet_id);
        packet.push(length);
        packet.extend_from_slice(&crc_bytes);
        packet.extend_from_slice(data);

        // Packet created successfully

        Ok(packet)
    }

    /// Parse an ANPP packet and validate checksums
    ///
    /// Returns (packet_id, data) on success
    pub fn parse_packet(packet: &[u8]) -> Result<(u8, Vec<u8>)> {
        if packet.len() < 5 {
            return Err(AnError::InvalidPacket(
                "Packet too short (minimum 5 bytes)".to_string(),
            ));
        }

        let header_lrc = packet[0];
        let packet_id = packet[1];
        let length = packet[2];

        // Extract CRC from packet
        let mut crc_buf = &packet[3..5];
        let received_crc = crc_buf.get_u16_le();
        let crc_bytes = &packet[3..5];

        // Verify Header LRC
        let crc0 = crc_bytes[0];
        let crc1 = crc_bytes[1];
        let mut expected_lrc =
            ((packet_id as u16 + length as u16 + crc0 as u16 + crc1 as u16) ^ 0xFF) + 1;
        if expected_lrc > 255 {
            expected_lrc %= 256;
        }

        if header_lrc != expected_lrc as u8 {
            return Err(AnError::InvalidPacket(format!(
                "Header LRC mismatch: expected 0x{:02X}, got 0x{:02X}",
                expected_lrc, header_lrc
            )));
        }

        // Check packet length consistency
        let expected_packet_length = 5 + length as usize; // Header + data
        if packet.len() != expected_packet_length {
            return Err(AnError::InvalidLength {
                expected: expected_packet_length,
                actual: packet.len(),
            });
        }

        // Extract data payload
        let data = if length > 0 {
            packet[5..5 + length as usize].to_vec()
        } else {
            Vec::new()
        };

        // Verify CRC16 checksum
        let calculated_crc = Self::calculate_crc16(&data);
        if received_crc != calculated_crc {
            return Err(AnError::InvalidChecksum);
        }

        // Packet parsed successfully

        Ok((packet_id, data))
    }

    /// Create a request packet for the specified packet ID
    pub fn create_request_packet(requested_packet_id: u8) -> Result<Vec<u8>> {
        let request_data = vec![requested_packet_id];
        Self::create_packet(1, &request_data) // Packet ID 1 is Request
    }

    /// Validate packet structure without parsing data
    pub fn validate_packet_structure(packet: &[u8]) -> Result<()> {
        if packet.len() < 5 {
            return Err(AnError::InvalidPacket(
                "Packet too short for header".to_string(),
            ));
        }

        let length = packet[2];
        let expected_length = 5 + length as usize;

        if packet.len() != expected_length {
            return Err(AnError::InvalidLength {
                expected: expected_length,
                actual: packet.len(),
            });
        }

        Ok(())
    }

    /// Extract packet ID from raw packet without full parsing
    pub fn extract_packet_id(packet: &[u8]) -> Result<u8> {
        if packet.len() < 2 {
            return Err(AnError::InvalidPacket(
                "Packet too short to contain ID".to_string(),
            ));
        }
        Ok(packet[1])
    }

    /// Extract packet length from raw packet
    pub fn extract_packet_length(packet: &[u8]) -> Result<u8> {
        if packet.len() < 3 {
            return Err(AnError::InvalidPacket(
                "Packet too short to contain length".to_string(),
            ));
        }
        Ok(packet[2])
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
        let packet_id = 20;

        // Create a packet
        let packet = AnppProtocol::create_packet(packet_id, &test_data).unwrap();

        // Parse it back
        let (parsed_id, parsed_data) = AnppProtocol::parse_packet(&packet).unwrap();

        assert_eq!(parsed_id, packet_id);
        assert_eq!(parsed_data, test_data);
    }

    #[test]
    fn test_request_packet_creation() {
        let request_packet = AnppProtocol::create_request_packet(3).unwrap();

        // Should be a request packet (ID 1) containing the requested packet ID
        let (packet_id, data) = AnppProtocol::parse_packet(&request_packet).unwrap();
        assert_eq!(packet_id, 1); // Request packet ID
        assert_eq!(data, vec![3]); // Requested packet ID
    }
}
