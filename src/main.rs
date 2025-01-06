use clap::Parser;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

/// A tool to parse ITM Instrumentation output from probe-rs
///
/// This tool reads and decodes ITM messages from `probe-rs itm swo` output.
/// It requires you to specify the target chip, trace duration, clock speed, and baud rate.
/// Optionally, yo0u can specify the probe (VID:PID) to use.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// (Optional) Probe identifier in the format VID:PID (e.g., 0483:374b)
    ///
    /// If not provided, the default probe will be used.
    #[arg(long, help = "Specify the debug probe VID:PID (optional)")]
    probe: Option<String>,

    /// Target chip identifier (e.g STM32F303CC)
    ///
    /// This specifies the chip you are working with.it is a required argumnt
    #[arg(long, help = "Specify the target chip identifier (required)")]
    chip: String,

    /// Duration of the trace in milliseconds
    ///
    /// Specifies how long to capture ITM data.
    #[arg(help = "Trace duration in milliseconds (required)")]
    duration: u32,

    /// Clock speed feeding the TPIU/SWO module in Hz
    ///
    /// Specifies the frequency of the clock feeding the SWO module
    #[arg(help = "Clock speed in Hz (required)")]
    clock: u32,

    /// desired baudrate for SWO output
    ///
    /// specifies the baud rate for transmitting SWO data
    #[arg(help = "SWO baud rate in Hz (required)")]
    baud: u32,
}

fn main() {
    // parse command-line arguments
    let args = Args::parse();

    // build the probe-rs command
    let mut cmd = Command::new("probe-rs");
    cmd.arg("itm");

    // Add optional --probe argument if provided
    if let Some(probe) = args.probe {
        cmd.arg("--probe").arg(probe);
    }

    cmd.arg("--chip")
        .arg(args.chip)
        .arg("swo")
        .arg(args.duration.to_string())
        .arg(args.clock.to_string())
        .arg(args.baud.to_string());

    // execute the probe-rs command and capture its output
    let probe_rs = cmd.stdout(Stdio::piped()).spawn().expect("Failed to start probe-rs");

    let stdout = probe_rs.stdout.expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);
    let mut line_buffer = String::new();

    // Parse and process the ITM output
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(payload) = parse_instrumentation(&line) {
                for c in payload.chars() {
                    if c == '\n' {
                        println!("{}", line_buffer);
                        line_buffer.clear();
                    } else {
                        line_buffer.push(c);
                    }
                }
            }
        }
    }

    // flush any remaining cntent in the buffer
    if !line_buffer.is_empty() {
        println!("{}", line_buffer);
    }
}

fn parse_instrumentation(line: &str) -> Option<String> {
    if line.starts_with("Ok(Instrumentation") {
        if let Some(start) = line.find("payload: [") {
            let payload_start = start + "payload: [".len();
            if let Some(end) = line[payload_start..].find(']') {
                let payload_str = &line[payload_start..payload_start + end];
                return Some(
                    payload_str
                        .split(',')
                        .filter_map(|num| num.trim().parse::<u8>().ok())
                        .map(|byte| byte as char)
                        .collect(),
                );
            }
        }
    }
    None
}
