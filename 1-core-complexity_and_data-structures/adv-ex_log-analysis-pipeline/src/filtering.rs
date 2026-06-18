use crate::log::LogEntry;

pub fn filter_valid_logs(logs: Vec<LogEntry>) -> Vec<LogEntry> {
    logs.into_iter()
        .filter(|log| log.response_status < 600)
        .collect()
}
