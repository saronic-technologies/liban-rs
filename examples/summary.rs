use anyhow::Result;
use clap::Parser;
use liban::{
    Packet,
    reader::{AnppReader, UdpReader},
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{self, Read},
    net::TcpStream,
    time::{Duration, Instant},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Summarize ANPP packet counts and rates
#[derive(Parser, Debug)]
struct Args {
    /// Input source: file path, TCP address (`host:port`), or UDP port number
    ///
    /// Examples:
    ///
    ///   - `/path/to/recording.bin` — read from file, print final summary
    ///
    ///   - `192.168.42.42:16718` — connect via TCP, print summary each interval
    ///
    ///   - `16718` — listen on UDP port, print summary each interval
    input: String,

    /// Summary interval in seconds; ignored in verbose mode (TCP/UDP only)
    #[arg(short, long, default_value_t = 10.0)]
    interval: f64,

    /// UDP read timeout in seconds; defaults to half the interval (UDP only)
    #[arg(short, long)]
    timeout: Option<f64>,

    /// Print full packet debug output; suppresses interval summaries
    #[arg(short, long)]
    verbose: bool,
}

const NAME_WIDTH: usize = 27;

struct PacketStats {
    name: &'static str,
    count: u64,
    first_ts: Option<(u32, u32)>,
    last_ts: Option<(u32, u32)>,
}

impl PacketStats {
    fn new(packet: &Packet) -> Self {
        let ts = packet.timestamp();
        Self {
            name: packet.type_name(),
            count: 1,
            first_ts: ts,
            last_ts: ts,
        }
    }

    fn update(&mut self, packet: &Packet) {
        self.count += 1;
        if let Some(ts) = packet.timestamp() {
            self.last_ts = Some(ts);
        }
    }
}

fn ts_to_secs(ts: (u32, u32)) -> f64 {
    ts.0 as f64 + ts.1 as f64 / 1e6
}

impl Display for PacketStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:<NAME_WIDTH$} {:>8}", self.name, self.count)?;
        if self.count >= 2
            && let (Some(first), Some(last)) = (self.first_ts, self.last_ts)
            && first != last
        {
            let t = ts_to_secs(last) - ts_to_secs(first);
            if t > 0.0 {
                let rate = (self.count - 1) as f64 / t;
                let uncertainty = (rate / t).sqrt();
                write!(f, " ({rate:>7.2} ± {uncertainty:>5.2} Hz)")?;
            }
        }
        Ok(())
    }
}

fn print_summary(
    stats: &BTreeMap<&'static str, PacketStats>,
    elapsed_secs: f64,
    min_ts: Option<(u32, u32)>,
    max_ts: Option<(u32, u32)>,
) {
    let total: u64 = stats.values().map(|s| s.count).sum();
    if let (Some(min), Some(max)) = (min_ts, max_ts) {
        let duration = ts_to_secs(max) - ts_to_secs(min);
        info!("Stats for the last {duration:.1}s:");
    } else {
        info!("Stats:");
    }
    info!("Total: {total} messages");
    for s in stats.values() {
        info!("  {s}");
    }
    info!("Wall-clock time: {elapsed_secs:.1}s");
}

fn run(reader: AnppReader<impl Read>, interval: Option<f64>, verbose: bool) -> Result<()> {
    let mut stats: BTreeMap<&'static str, PacketStats> = BTreeMap::new();
    let mut window_start = Instant::now();
    let mut window_min_ts: Option<(u32, u32)> = None;
    let mut window_max_ts: Option<(u32, u32)> = None;

    for result in reader {
        let packet = result?;
        if verbose {
            println!("{packet:?}");
        }
        if let Some(ts) = packet.timestamp() {
            window_min_ts = Some(window_min_ts.map_or(ts, |m| m.min(ts)));
            window_max_ts = Some(window_max_ts.map_or(ts, |m| m.max(ts)));
        }
        stats
            .entry(packet.type_name())
            .and_modify(|s| s.update(&packet))
            .or_insert_with(|| PacketStats::new(&packet));

        let elapsed = window_start.elapsed().as_secs_f64();
        if interval.is_some_and(|i| elapsed >= i) {
            print_summary(&stats, elapsed, window_min_ts, window_max_ts);
            stats.clear();
            window_min_ts = None;
            window_max_ts = None;
            window_start = Instant::now();
        }
    }

    if !stats.is_empty() {
        print_summary(
            &stats,
            window_start.elapsed().as_secs_f64(),
            window_min_ts,
            window_max_ts,
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(io::stderr)
        .init();

    let args = Args::parse();

    let interval = (!args.verbose).then_some(args.interval);

    // Pure digits -> UDP, host:port (no slash before colon) -> TCP, otherwise file
    let reader: Box<dyn Read> = if args.input.chars().all(|c| c.is_ascii_digit()) {
        let port: u16 = args.input.parse()?;
        let timeout = args.timeout.unwrap_or(args.interval / 2.0);
        info!("Listening on UDP port {port} (timeout {timeout:.1}s)");
        Box::new(UdpReader::try_new(port, Duration::from_secs_f64(timeout))?)
    } else if args
        .input
        .find(':')
        .is_some_and(|c| !args.input[..c].contains('/'))
    {
        info!("Connecting to TCP: {}", args.input);
        Box::new(TcpStream::connect(&args.input)?)
    } else {
        info!("Reading from file: {}", args.input);
        Box::new(File::open(&args.input)?)
    };

    run(AnppReader::new(reader), interval, args.verbose)
}
