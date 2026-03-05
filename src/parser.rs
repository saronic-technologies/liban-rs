extern crate alloc;
use alloc::vec::Vec;

use crate::packet::{Packet, PacketKind};
use crate::protocol::AnppProtocol;

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

fn parse_packet(input: &[u8]) -> Result<Packet> {
    // Make sure we have enough data for a minimal packet
    if input.len() < MIN_PACKET_SIZE {
        debug!("Incomplete data, don't have enough for minimal packet");
        return Err(ParseError::IncompleteData);
    }

    // ANPP packets don't have a sync sequence, they start directly with LRC
    let header_lrc = input[0];
    let packet_id = input[1];
    let payload_length = input[2];

    let packet_length = payload_length as usize + 5; // length in packet does not include header

    // Ensure we have the complete packet
    if input.len() < packet_length {
        debug!("Don't have full packet, need {} bytes but have {}", packet_length, input.len());
        return Err(ParseError::IncompleteData);
    }

    // Extract CRC16 (last 2 bytes of header)
    let crc16 = u16::from_le_bytes([input[3], input[4]]);

    // Validate header LRC
    let calculated_lrc = AnppProtocol::calculate_lrc(packet_id, payload_length, crc16);
    if header_lrc != calculated_lrc {
        debug!("Invalid header LRC for packet ID {}: expected {:#02x}, got {:#02x}",
               packet_id, calculated_lrc, header_lrc);
        return Err(ParseError::InvalidHeader);
    }

    // Extract payload (everything between header and CRC16)
    let payload = &input[5..packet_length];

    // Validate packet CRC16 (calculated over payload)
    let calculated_crc = AnppProtocol::calculate_crc16(payload);
    if crc16 != calculated_crc {
        debug!("Invalid CRC16 for packet ID {}: expected {:#04x}, got {:#04x}",
               packet_id, calculated_crc, crc16);
        return Err(ParseError::InvalidCRC);
    }

    // Validate payload length matches expected length for known packet types
    let packet_kind = PacketKind::from(packet_id);
    if let Some(expected_length) = packet_kind.byte_length() {
        if payload_length as usize != expected_length {
            debug!("Payload length mismatch for packet ID {}: expected {} bytes, got {}",
                   packet_id, expected_length, payload_length);
            return Err(ParseError::InvalidPayload);
        }
    }

    // Parse the packet payload
    match Packet::from_bytes(packet_id, payload) {
        Ok(packet) => Ok((packet, packet_length)),
        Err(_) => {
            debug!("Failed to parse payload for packet ID {}", packet_id);
            Err(ParseError::InvalidPayload)
        }
    }
}

#[derive(Debug)]
pub enum DatagramError {
    IncompleteData,
    InvalidHeader,
    InvalidCrc,
    InvalidPayload,
}

/// Parse a single ANPP packet from a datagram. Expects the packet to
/// start at byte 0 — no scanning.
pub fn parse_datagram(datagram: &[u8]) -> core::result::Result<Packet, DatagramError> {
    match parse_packet(datagram) {
        Ok((packet, _len)) => Ok(packet),
        Err(ParseError::IncompleteData) => Err(DatagramError::IncompleteData),
        Err(ParseError::InvalidHeader) => Err(DatagramError::InvalidHeader),
        Err(ParseError::InvalidCRC) => Err(DatagramError::InvalidCrc),
        Err(ParseError::InvalidPayload) => Err(DatagramError::InvalidPayload),
    }
}

/// Stateful stream parser for TCP or other byte-stream transports.
///
/// Buffers incoming bytes and scans for valid ANPP packets. When a parse
/// attempt fails, advances by one byte and retries — necessary because
/// TCP provides no packet boundaries.
pub struct AnppParser {
    buf: Vec<u8>,
    buf_start: usize, // Start position of valid data in buffer
}

impl AnppParser {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            buf_start: 0,
        }
    }

    /// Consume bytes and attempt to parse a packet. If we can't
    /// find a complete packet we return None. If we get a packet it doesn't
    /// guarantee the whole internal buffer is drained.
    pub fn consume(&mut self, input: &[u8]) -> Option<Packet> {
        // Append new data to buffer
        self.buf.extend(input);

        loop {
            let available_data = &self.buf[self.buf_start..];

            if available_data.is_empty() {
                return None;
            }

            match parse_packet(available_data) {
                Ok((packet, bytes_consumed)) => {
                    // Advance buffer start position instead of draining
                    self.buf_start += bytes_consumed;

                    // Compact buffer if it gets too fragmented
                    if self.buf_start > self.buf.len() / 2 {
                        self.buf.drain(0..self.buf_start);
                        self.buf_start = 0;
                    }

                    return Some(packet);
                },
                Err(ParseError::IncompleteData) => {
                    return None;
                }
                Err(ParseError::InvalidCRC | ParseError::InvalidHeader | ParseError::InvalidPayload) => {
                    // Advance by 1 byte to find next valid packet
                    self.buf_start += 1;

                    // Ensure we don't go past buffer end
                    if self.buf_start >= self.buf.len() {
                        self.buf.clear();
                        self.buf_start = 0;
                        return None;
                    }
                }
            }
        }
    }

    /// Get the current buffer length (for debugging/monitoring)
    pub fn buffer_len(&self) -> usize {
        self.buf.len() - self.buf_start
    }

    /// Clear the internal buffer
    pub fn clear(&mut self) {
        self.buf.clear();
        self.buf_start = 0;
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
    use crate::packet::{PacketId, system::Request};
    use binrw::BinWrite;

    #[test]
    fn test_lrc_calculation() {
        // Test LRC calculation with known values
        let packet_id: u8 = 1;
        let length: u8 = 6;
        let crc16: u16 = 0x1234;

        // Calculate expected LRC using the actual ANPP algorithm
        let crc_low = (crc16 & 0xFF) as u8;  // 0x34
        let crc_high = ((crc16 >> 8) & 0xFF) as u8; // 0x12
        let sum = packet_id.wrapping_add(length).wrapping_add(crc_low).wrapping_add(crc_high);
        let expected_lrc = (sum ^ 0xFF).wrapping_add(1);

        assert_eq!(AnppProtocol::calculate_lrc(packet_id, length, crc16), expected_lrc);
    }

    #[test]
    fn test_parse_request_packet() {
        let mut parser = AnppParser::new();

        // Create a request packet manually
        let packet_data = Request { requested_packet: PacketKind::SystemState };
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

    #[test]
    fn test_round_trip_request_packet() {
        // Build a valid ANPP frame for a Request packet (ID=1, payload=[20])
        let frame = AnppProtocol::get_packet_bytes(PacketId::new(1), &[20]).unwrap();

        let mut parser = AnppParser::new();
        let packet = parser.consume(&frame).expect("should parse a valid packet");

        match packet {
            Packet::Request(req) => assert_eq!(req.requested_packet, PacketKind::SystemState),
            other => panic!("expected Request packet, got {:?}", other),
        }

        // Buffer should be fully consumed
        assert_eq!(parser.buffer_len(), 0);
    }

    #[test]
    fn test_unsupported_packet_skipped_cleanly() {
        // Build a valid ANPP frame for an unsupported packet ID (e.g. 255)
        let payload = [0xAA, 0xBB, 0xCC];
        let frame = AnppProtocol::get_packet_bytes(PacketId::new(255), &payload).unwrap();

        let mut parser = AnppParser::new();
        let packet = parser.consume(&frame).expect("should parse unsupported packet");

        assert!(matches!(packet, Packet::Unsupported(_)));
        assert_eq!(parser.buffer_len(), 0);
    }

    #[test]
    fn test_two_packets_back_to_back() {
        let frame1 = AnppProtocol::get_packet_bytes(PacketId::new(1), &[20]).unwrap();
        let frame2 = AnppProtocol::get_packet_bytes(PacketId::new(1), &[21]).unwrap();

        let mut combined = frame1;
        combined.extend_from_slice(&frame2);

        let mut parser = AnppParser::new();
        let p1 = parser.consume(&combined).expect("should parse first packet");
        match p1 {
            Packet::Request(req) => assert_eq!(req.requested_packet, PacketKind::SystemState),
            other => panic!("expected Request(20), got {:?}", other),
        }

        // Drain second packet without re-appending
        let p2 = parser.consume(&[]).expect("should parse second packet");
        match p2 {
            Packet::Request(req) => assert_eq!(req.requested_packet, PacketKind::UnixTime),
            other => panic!("expected Request(21), got {:?}", other),
        }

        assert_eq!(parser.buffer_len(), 0);
    }

    #[test]
    fn test_parse_datagram_valid() {
        let frame = AnppProtocol::get_packet_bytes(PacketId::new(1), &[20]).unwrap();
        let packet = parse_datagram(&frame).expect("should parse a valid datagram");

        match packet {
            Packet::Request(req) => assert_eq!(req.requested_packet, PacketKind::SystemState),
            other => panic!("expected Request packet, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_datagram_unsupported() {
        let frame = AnppProtocol::get_packet_bytes(PacketId::new(255), &[0xAA]).unwrap();
        let packet = parse_datagram(&frame).expect("should parse unsupported datagram");
        assert!(matches!(packet, Packet::Unsupported(_)));
    }

    #[test]
    fn test_parse_datagram_incomplete() {
        assert!(matches!(parse_datagram(&[0x01, 0x02]), Err(DatagramError::IncompleteData)));
    }

    #[test]
    fn test_parse_datagram_invalid_lrc() {
        let mut frame = AnppProtocol::get_packet_bytes(PacketId::new(1), &[20]).unwrap();
        frame[0] ^= 0xFF; // corrupt LRC
        assert!(matches!(parse_datagram(&frame), Err(DatagramError::InvalidHeader)));
    }

    #[test]
    fn test_parse_datagram_invalid_crc() {
        let mut frame = AnppProtocol::get_packet_bytes(PacketId::new(1), &[20]).unwrap();
        // corrupt payload byte (changes CRC but not LRC)
        let last = frame.len() - 1;
        frame[last] ^= 0xFF;
        assert!(matches!(parse_datagram(&frame), Err(DatagramError::InvalidCrc)));
    }
}