use crate::log::LogEntry;

// Filters out log entries with invalid HTTP status codes (>= 600).
// Returns a vector of valid log entries.
pub fn filter_valid_logs(logs: Vec<LogEntry>) -> Vec<LogEntry> {
    logs.into_iter()
        // Keep only logs with a response status less than 600.
        .filter(|log| log.response_status < 600)
        .collect()
}
