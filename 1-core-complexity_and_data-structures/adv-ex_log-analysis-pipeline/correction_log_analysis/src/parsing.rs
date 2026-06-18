use crate::log::LogEntry;
use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Parses a CSV file of log entries into a vector of `LogEntry` structs.
// Returns `Result<Vec<LogEntry>, String>` to handle potential errors during file reading or parsing.
pub fn parse_logs(file_path: &str) -> Result<Vec<LogEntry>, String> {
    // Open the file at the given path. If an error occurs, convert it to a string and return it.
    let path = Path::new(file_path);
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut logs = Vec::new();
    // Iterate over each line in the file.
    for line in reader.lines() {
        // If reading a line fails, convert the error to a string and return it.
        let line = line.map_err(|e| e.to_string())?;
        // Split the line into fields using commas as delimiters.
        let fields: Vec<&str> = line.split(',').collect();
        // Skip malformed lines that do not have exactly 5 fields.
        if fields.len() != 5 {
            continue;
        }
        // Parse the timestamp from the first field.
        let timestamp = NaiveDateTime::parse_from_str(fields[0], "%Y-%m-%dT%H:%M:%S")
            .map_err(|e| e.to_string())?;
        // Create a `LogEntry` from the parsed fields.
        let log = LogEntry {
            timestamp,
            ip_address: fields[1].to_string(),
            endpoint: fields[2].to_string(),
            // Parse the response status as a `u16`. If parsing fails, convert the error to a string.
            response_status: fields[3].parse::<u16>().map_err(|e| e.to_string())?,
            // Parse the response time as a `u32`. If parsing fails, convert the error to a string.
            response_time_ms: fields[4].parse::<u32>().map_err(|e| e.to_string())?,
        };
        logs.push(log);
    }
    Ok(logs)
}
