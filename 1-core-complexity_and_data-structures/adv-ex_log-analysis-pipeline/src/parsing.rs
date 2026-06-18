use crate::log::LogEntry;
use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_logs(file_path: &str) -> Result<Vec<LogEntry>, String> {
    let path = Path::new(file_path);
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut logs = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 5 {
            continue; // Skip malformed lines
        }

        let timestamp = NaiveDateTime::parse_from_str(fields[0], "%Y-%m-%dT%H:%M:%S")
            .map_err(|e| e.to_string())?;
        let log = LogEntry {
            timestamp,
            ip_address: fields[1].to_string(),
            endpoint: fields[2].to_string(),
            response_status: fields[3].parse().map_err(|e| e.to_string())?,
            response_time_ms: fields[4].parse().map_err(|e| e.to_string())?,
        };
    }

    logs.push(log);
}
