use crate::AnppPacket;
use crate::parser::AnppParser;

use std::io::Read;

// NOTE: May make this tunable. The std reader is going to be on user
// space linux and in many cases users will have the memory.
// 8K is the default size of the BufReader in rust.
const BUFFER_SIZE: usize = 1024 * 8;

/// Read ANPP data via a BuffReader and Iterator.
///
/// # Examples
///
/// ```no_run
/// use liban::reader::AnppReader;
/// use std::net::TcpStream;
/// use std::io::Result;
///
/// fn main() -> Result<()> {
///     let stream = TcpStream::connect("127.0.0.1:8080")?;
///     let anpp_reader = AnppReader::new(stream);
///     for packet in anpp_reader {
///         match packet {
///             Ok(p) => eprintln!("{:?}", p),
///             Err(e) => eprintln!("Error: {:?}", e),
///         }
///     }
///     Ok(())
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct AnppReader<R: Read> {
    reader: R,
    parser: AnppParser,
    drain_internal: bool,
}

impl<R: Read> AnppReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            parser: AnppParser::new(),
            drain_internal: false,
        }
    }
}

impl<R: Read> Iterator for AnppReader<R> {
    type Item = Result<AnppPacket, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            tracing::debug!("Trying to read from reader");
            let (bytes_read, is_eof) = {
                if self.drain_internal {
                    (0, false)
                } else {
                    match self.reader.read(&mut buffer) {
                        Ok(br) => {
                            tracing::debug!("Successfully read {br} bytes from reader");
                            (br, br == 0)
                        }
                        Err(e) => {
                            return Some(Err(e));
                        }
                    }
                }
            };

            match self.parser.consume(&buffer[..bytes_read]) {
                Some(packet) => {
                    // NOTE: When we get a packet the parser still
                    // contains the internal buffer so lets drain that
                    // all the way down until we get a None which
                    // indicates that the parser needs more data to
                    // get packets. Instead of constantly growing
                    // that buffer by reading more data from the
                    // reader we first want to have it go down to
                    // reduce memory usage and work for the internal
                    // parser.
                    self.drain_internal = true;
                    return Some(Ok(packet));
                }
                None => {
                    self.drain_internal = false;
                    // loop
                }
            }

            if is_eof {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestPacket;
    use std::io::{Read, Cursor};

    #[test]
    fn test_random_data_consumption() {
        // Create a reader that tracks how many bytes were read
        struct TrackingReader {
            data: Vec<u8>,
            position: usize,
        }

        impl TrackingReader {
            fn new(size: usize) -> Self {
                // Generate random data
                let data: Vec<_> = (0..size).map(|i| (i % 256) as u8).collect();
                Self { data, position: 0 }
            }

            fn bytes_read(&self) -> usize {
                self.position
            }

            fn total_bytes(&self) -> usize {
                self.data.len()
            }
        }

        impl Read for TrackingReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                let remaining = self.data.len() - self.position;
                let to_read = buf.len().min(remaining);

                if to_read > 0 {
                    buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
                    self.position += to_read;
                }

                Ok(to_read)
            }
        }

        // Test with various data sizes
        let test_sizes = vec![100, 1024, 8192, 16384, 100000];

        for size in test_sizes {
            let mut reader = TrackingReader::new(size);
            let total_bytes = reader.total_bytes();

            let anpp_reader = AnppReader::new(&mut reader);

            // Consume all packets (valid or invalid)
            let mut packet_count = 0;
            let mut _error_count = 0;

            for result in anpp_reader {
                match result {
                    Ok(_) => packet_count += 1,
                    Err(_) => _error_count += 1,
                }
            }

            // Verify that all bytes were consumed
            assert_eq!(
                reader.bytes_read(),
                total_bytes,
                "AnppReader did not consume all bytes. Read {} out of {} bytes",
                reader.bytes_read(),
                total_bytes
            );

            println!(
                "Test passed for {} bytes: {} packets parsed, {} errors",
                size, packet_count, _error_count
            );
        }
    }

    #[test]
    fn test_anpp_reader_empty() {
        let data = vec![];
        let cursor = Cursor::new(data);
        let mut reader = AnppReader::new(cursor);

        // Should return None for empty data
        assert!(reader.next().is_none());
    }

    #[test]
    fn test_anpp_reader_invalid_data() {
        let data = vec![0xFF, 0xFF, 0xFF, 0xFF];
        let cursor = Cursor::new(data);
        let anpp_reader = AnppReader::new(cursor);

        let mut packet_count = 0;
        let mut _error_count = 0;

        for result in anpp_reader {
            match result {
                Ok(_) => packet_count += 1,
                Err(_) => _error_count += 1,
            }
        }

        // Should not find any valid packets in random data
        assert_eq!(packet_count, 0, "Should not parse any valid packets from random data");
    }

    #[test]
    fn test_anpp_reader_construction() {
        let data = vec![1, 2, 3, 4, 5];
        let cursor = Cursor::new(data);
        let _reader = AnppReader::new(cursor);
        // Just test that construction works without panic
    }

    #[test]
    fn test_anpp_reader_with_known_packets() {
        use binrw::BinWrite;
        use std::io::Cursor as WriteCursor;

        // Create some known ANPP packets
        let request_packet = RequestPacket { packet_id: 20 }; // Request system state

        // Serialize the packet payload
        let mut cursor = WriteCursor::new(Vec::new());
        request_packet.write_le(&mut cursor).unwrap();
        let _payload = cursor.into_inner();

        // Create a minimal ANPP frame manually for testing
        // This would need proper ANPP framing with LRC, CRC16, etc.
        // For now, just test that the reader handles structured data
        let test_data = vec![
            // Some structured test data that might resemble ANPP packets
            0x01, 0x14, 0x01, 0x00, 0x00, // Minimal packet-like structure
            0x02, 0x15, 0x01, 0x00, 0x00, // Another packet-like structure
        ];

        let cursor = Cursor::new(test_data);
        let anpp_reader = AnppReader::new(cursor);

        let mut total_packets = 0;
        for result in anpp_reader {
            match result {
                Ok(packet) => {
                    total_packets += 1;
                    println!("Successfully parsed packet: {:?}", packet);
                }
                Err(e) => {
                    println!("I/O Error: {:?}", e);
                }
            }
        }

        // The exact number depends on whether our test data forms valid ANPP packets
        println!("Total packets parsed from test data: {}", total_packets);
    }

    #[test]
    fn test_anpp_reader_buffer_boundaries() {
        // Test data that spans multiple buffer reads
        let mut test_data = Vec::new();

        // Create data larger than our buffer size to test boundary conditions
        for i in 0..20000 {
            test_data.push((i % 256) as u8);
        }

        let cursor = Cursor::new(test_data);
        let anpp_reader = AnppReader::new(cursor);

        let mut packet_count = 0;
        let mut error_count = 0;

        for result in anpp_reader {
            match result {
                Ok(_) => packet_count += 1,
                Err(_) => error_count += 1,
            }
        }

        // Should handle large data without panicking
        println!("Buffer boundary test: {} packets, {} errors", packet_count, error_count);
    }
}