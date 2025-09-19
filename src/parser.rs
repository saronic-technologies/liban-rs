extern crate alloc;
use alloc::vec::Vec;

use crate::{AnppPacket, PacketKind};

use crc::{Crc, CRC_16_IBM_3740};

use tracing::debug;

#[derive(Debug)]
pub enum Error {
    InvalidHeaderLRC,
    InvalidCRC16,
    BinRWError(binrw::Error),
}

enum ParseError {
    IncompleteData,
    InvalidHeader,
    InvalidCRC,
    InvalidPayload,
}

type Result<T> = core::result::Result<(T, usize), ParseError>;

// Constants for our parser
const MIN_PACKET_SIZE: usize = 5; // 1 LRC + 1 ID + 1 length + 2 CRC16
const ANPP_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_3740);

/// Calculate the Linear Redundancy Check (LRC) for ANPP header
fn calculate_lrc(packet_id: u8, length: u8, crc16: u16) -> u8 {
    let crc_low = (crc16 & 0xFF) as u8;
    let crc_high = ((crc16 >> 8) & 0xFF) as u8;

    // XOR all bytes together
    packet_id ^ length ^ crc_low ^ crc_high
}

fn parse_packet(input: &[u8]) -> Result<AnppPacket> {
    // Make sure we have enough data for a minimal packet
    if input.len() < MIN_PACKET_SIZE {
        debug!("Incomplete data, don't have enough for minimal packet");
        return Err(ParseError::IncompleteData);
    }

    // ANPP packets don't have a sync sequence, they start directly with LRC
    let header_lrc = input[0];
    let packet_id = input[1];
    let payload_length = input[2];
    let packet_length = payload_length + 5; // length in packet does not include header4

    // Ensure we have the complete packet
    if input.len() < packet_length as usize {
        debug!("Don't have full packet, need {} bytes but have {}", packet_length, input.len());
        return Err(ParseError::IncompleteData);
    }

    // Extract CRC16 (last 2 bytes of header)
    let crc16 = u16::from_le_bytes([input[3], input[4]]);

    // Validate header LRC
    let calculated_lrc = calculate_lrc(packet_id, payload_length, crc16);
    if header_lrc != calculated_lrc {
        debug!("Invalid header LRC for packet ID {}: expected {:#02x}, got {:#02x}",
               packet_id, calculated_lrc, header_lrc);
        return Err(ParseError::InvalidHeader);
    }

    // Extract payload (everything between header and CRC16)
    let payload_start = 5;
    let payload_end = packet_length as usize;
    let payload = input[payload_start..payload_end].to_vec();

    // Validate packet CRC16 (calculated over packet ID + length + payload)
    // let mut crc_data = Vec::with_capacity(2 + payload.len());
    // crc_data.extend_from_slice(&payload);

    // Check if this is a supported packet type
    let packet_kind = PacketKind::from(packet_id);
    if let PacketKind::Unsupported = packet_kind {
        debug!("Unsupported packet ID: {}", packet_id);
        // For unsupported packets, we still return them as Unsupported variant
        return Ok((AnppPacket::Unsupported(payload), payload_length as usize));
    }

    // Validate payload length matches expected length for this packet type
    if let Some(expected_length) = packet_kind.byte_length() {
        let expected_payload_length = expected_length - 5; // Subtract header size
        if payload.len() != expected_payload_length {
            debug!("Payload length mismatch for packet ID {}: expected {} bytes, got {}",
                   packet_id, expected_payload_length, payload.len());
            return Err(ParseError::InvalidPayload);
        }
    }

    let calculated_crc = ANPP_CRC.checksum(&payload);
    if crc16 != calculated_crc {
        debug!("Invalid CRC16 for packet ID {}: expected {:#04x}, got {:#04x}",
               packet_id, calculated_crc, crc16);
        return Err(ParseError::InvalidCRC);
    }

    // Parse the packet payload
    match AnppPacket::from_bytes(packet_id, payload) {
        Ok(packet) => Ok((packet, payload_length as usize)),
        Err(_) => {
            debug!("Failed to parse payload for packet ID {}", packet_id);
            Err(ParseError::InvalidPayload)
        }
    }
}

pub struct AnppParser {
    buf: Vec<u8>,
}

impl AnppParser {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }

    /// Consume bytes and attempt to parse a packet. If we can't
    /// find a complete packet we return None. If we get a packet it doesn't
    /// guarantee the whole internal buffer is drained.
    pub fn consume(&mut self, input: &[u8]) -> Option<AnppPacket> {
        self.buf.extend(input);
        loop {
            debug!("Internal Buffer Size: {}", self.buf.len());
            match parse_packet(&self.buf) {
                Ok((packet, bytes_consumed)) => {
                    debug!("Successfully parsed packet, draining {} bytes from buffer", bytes_consumed);
                    self.buf.drain(0..bytes_consumed);
                    return Some(packet);
                },
                Err(ParseError::IncompleteData) => {
                    debug!("Incomplete data, need more bytes!");
                    return None;
                }
                Err(ParseError::InvalidCRC | ParseError::InvalidHeader | ParseError::InvalidPayload) => {
                    debug!("Parse error, advancing buffer by 1 byte to find next valid packet");
                    if !self.buf.is_empty() {
                        self.buf.drain(0..1);
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    /// Get the current buffer length (for debugging/monitoring)
    pub fn buffer_len(&self) -> usize {
        self.buf.len()
    }

    /// Clear the internal buffer
    pub fn clear(&mut self) {
        self.buf.clear();
    }
}

impl Default for AnppParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestPacket;
    use binrw::BinWrite;

    #[test]
    fn test_lrc_calculation() {
        // Test LRC calculation with known values
        let packet_id = 1;
        let length = 6;
        let crc16 = 0x1234;
        let expected_lrc = packet_id ^ length ^ 0x34 ^ 0x12;
        assert_eq!(calculate_lrc(packet_id, length, crc16), expected_lrc);
    }

    #[test]
    fn test_parse_request_packet() {
        let mut parser = AnppParser::new();

        // Create a request packet manually
        let packet_data = RequestPacket { packet_id: 20 }; // Request system state
        let _packet_bytes = packet_data.write_le(&mut std::io::Cursor::new(Vec::new())).unwrap();

        // This would need proper ANPP framing to test fully
        // For now, test that parser doesn't crash with invalid data
        let result = parser.consume(&[0xFF, 0xFF, 0xFF]);
        assert!(result.is_none());
    }

    #[test]
    fn test_parser_incomplete_data() {
        let mut parser = AnppParser::new();

        // Test with insufficient data
        let result = parser.consume(&[0x01, 0x02]);
        assert!(result.is_none());

        // Buffer should retain the data
        assert_eq!(parser.buffer_len(), 2);
    }

    #[test]
    fn test_parser_clear() {
        let mut parser = AnppParser::new();
        parser.consume(&[0x01, 0x02, 0x03]);
        assert!(parser.buffer_len() > 0);

        parser.clear();
        assert_eq!(parser.buffer_len(), 0);
    }
}