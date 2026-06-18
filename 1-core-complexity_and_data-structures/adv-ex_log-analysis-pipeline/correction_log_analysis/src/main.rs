// Import modules for the log analysis pipeline.
mod aggregation;
mod filtering;
mod log;
mod macros;
mod output;
mod parsing;
mod threat_detection;

// Import necessary types and functions from the modules.
use aggregation::aggregate_data;
use filtering::filter_valid_logs;
use output::write_report;
use parsing::parse_logs;
use threat_detection::detect_threats;

fn main() {
    // Parse log entries from the CSV file. If parsing fails, log the error and use an empty vector.
    let logs = parse_logs("server_logs.csv").unwrap_or_else(|e| {
        log_error!("Failed to parse logs: {}", e);
        Vec::new()
    });
    // Filter out invalid log entries.
    let valid_logs = filter_valid_logs(logs);
    // Aggregate the log data into statistics.
    let data = aggregate_data(&valid_logs);
    // Detect potential threats (IPs with >= 3 failed logins).
    let threats = detect_threats(&data, 3);
    log_debug!("Potential threats: {:?}", threats);
    // Write the aggregated data to a JSON report. If writing fails, log the error.
    write_report(&data, "report.json").unwrap_or_else(|e| {
        log_error!("Failed to write report: {}", e);
    });
}
