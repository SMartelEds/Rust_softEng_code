use chrono::NaiveDateTime;
use serde::Serialize;
use std::collections::HashMap;

// Represents a single log entry with timestamp, IP address, endpoint, response status, and response time.
#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    pub timestamp: NaiveDateTime, // Timestamp of the log entry.
    pub ip_address: String,       // IP address of the client.
    pub endpoint: String,         // Endpoint accessed by the client.
    pub response_status: u16,     // HTTP response status code.
    pub response_time_ms: u32,    // Response time in milliseconds.
}

// Stores aggregated statistics from log entries.
#[derive(Debug, Default, Serialize)]
pub struct AggregatedData {
    pub requests_per_ip: HashMap<String, usize>, // Count of requests per IP address.
    pub requests_per_endpoint: HashMap<String, usize>, // Count of requests per endpoint.
    pub failed_logins: HashMap<String, usize>, // Count of failed login attempts (status 401) per IP.
}
