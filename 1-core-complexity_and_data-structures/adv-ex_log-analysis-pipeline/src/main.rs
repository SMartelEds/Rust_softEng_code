mod aggregation;
mod filtering;
mod log;
mod macros;
mod output;
mod parsing;
mod threat_detection;

use aggregation::aggregate_data;
use filtering::filter_valid_logs;
use log::AggregatedData;
use output::write_report;
use parsing::parse_logs;
use threat_detection::detect_threats;

fn main() {
    let logs = parse_logs("server_logs.csv").unwrap_or_else(|e| {
        log_error!("Failed to parse logs: {}", e);
        Vec::new()
    });
    let valid_logs = filter_valid_logs(logs);
    let data = aggregate_data(&valid_logs);
    let threats = detect_threats(&data, 3); // Threshold: 3 failed logins
    log_debug!("Potential threats: {:?}", threats);
    write_report(&data, "report.json").unwrap_or_else(|e| {
        log_error!("Failed to write report: {}", e);
    });
}
