# liban

A sans-IO Rust library for [ANPP](https://docs.advancednavigation.com/certus/ANPP/Advanced%20Navigation%20Packet.htm),
the Advanced Navigation Packet Protocol.

## Usage

```rust
use liban::{reader::AnppReader, Packet};
use std::net::TcpStream;

let stream = TcpStream::connect("192.168.42.42:16718")?;
for packet in AnppReader::new(stream).flatten() {
    if let Packet::SystemState(s) = packet {
        println!("{:.6} {:.6}", s.latitude.to_degrees(), s.longitude.to_degrees());
    }
}
```

`AnppReader` wraps any `io::Read` as a blocking iterator. Use `parse_datagram`
for a single framed buffer, or feed byte slices to `AnppParser` when you own
the buffering. `AnppParser::consume` returns one packet per call even when the
bytes you fed it completed several, so keep calling it with an empty slice
until it yields `None`.

## Packet coverage

78 packet types are modeled as `Packet` variants, spanning system IDs 0
through 14, state 20 through 93, and configuration 180 through 203.
Unrecognized IDs parse into the `Packet::Unsupported` variant.

## Examples

Each defaults to `192.168.42.42:16718`:

```bash
cargo run --example parse_packets   # decode a TCP stream
cargo run --example read_config     # request and dump config packets
cargo run --example send_config     # write packet rates and filter options
cargo run --example udp_reader      # decode datagrams, binds with --bind rather than --ip
```

## License

MPL-2.0. See [LICENSE](LICENSE).
