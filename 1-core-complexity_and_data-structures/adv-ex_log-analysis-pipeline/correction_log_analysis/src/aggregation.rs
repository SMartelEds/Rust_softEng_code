use crate::log::{AggregatedData, LogEntry};

// Aggregates log data into statistics: requests per IP, requests per endpoint, and failed logins.
// Returns an `AggregatedData` struct containing the aggregated statistics.
pub fn aggregate_data(logs: &[LogEntry]) -> AggregatedData {
    let mut data = AggregatedData::default();
    for log in logs {
        // Increment the count of requests for the IP address.
        *data
            .requests_per_ip
            .entry(log.ip_address.clone())
            .or_insert(0) += 1;
        // Increment the count of requests for the endpoint.
        *data
            .requests_per_endpoint
            .entry(log.endpoint.clone())
            .or_insert(0) += 1;
        // If the response status is 401 (Unauthorized), increment the count of failed logins for the IP.
        if log.response_status == 401 {
            *data
                .failed_logins
                .entry(log.ip_address.clone())
                .or_insert(0) += 1;
        }
    }
    data
}
