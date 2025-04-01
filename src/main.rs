use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Enum representing different log levels
#[derive(Debug, PartialEq)] // Add PartialEq to enable == comparisons
enum LogLevel {
    Info,
    Warning,
    Error,
    Unknown,
}

/// Function to determine log level from a log entry
fn get_log_level(line: &str) -> LogLevel {
    if line.contains("[INFO]") {
        LogLevel::Info
    } else if line.contains("[WARNING]") {
        LogLevel::Warning
    } else if line.contains("[ERROR]") {
        LogLevel::Error
    } else {
        LogLevel::Unknown
    }
}

/// Function to read and process the log file
fn analyze_logs(filename: &str, filter_level: Option<LogLevel>) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let reader = io::BufReader::new(file);

    let mut info_count = 0;
    let mut warning_count = 0;
    let mut error_count = 0;
    

    println!("Analyzing log file: {}", filename);

    for line in reader.lines() {
        let line = line?;

        match get_log_level(&line) {
            LogLevel::Info => info_count += 1,
            LogLevel::Warning => warning_count += 1,
            LogLevel::Error => error_count += 1,
            LogLevel::Unknown => continue, // Skip unrecognized lines
        }

        // If filtering, only print matching logs
        if let Some(ref level) = filter_level {
            if get_log_level(&line) == *level {
                println!("{}", line);
            }
        }
    }

    println!("\nLog Summary:");
    println!("INFO: {} occurrences", info_count);
    println!("WARNING: {} occurrences", warning_count);
    println!("ERROR: {} occurrences", error_count);

    Ok(())
}

/// Function to parse log level argument from CLI
fn parse_log_level(arg: &str) -> Option<LogLevel> {
    match arg.to_lowercase().as_str() {
        "info" => Some(LogLevel::Info),
        "warning" => Some(LogLevel::Warning),
        "error" => Some(LogLevel::Error),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <log_file> [log_level]");
        return;
    }

    let log_file = &args[1];
    let filter_level = if args.len() > 2 {
        parse_log_level(&args[2])
    } else {
        None
    };

    if let Err(e) = analyze_logs(log_file, filter_level) {
        eprintln!("Error reading log file: {}", e);
    }
}