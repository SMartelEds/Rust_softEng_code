use crate::log::{AggregatedData, LogEntry};

pub fn aggregate_data(logs: &[LogEntry]) -> AggregatedData {
    let mut data = AggregatedData::default();
    for log in logs {
        *data
            .requests_per_ip
            .entry(log.ip_address.clone())
            .or_insert(0) += 1;
        *data
            .requests_per_endpoint
            .entry(log.endpoint.clone())
            .or_insert(0) += 1;
        if log.response_status == 401 {
            *data
                .failed_logins
                .entry(log.ip_address.clone())
                .or_insert(0) += 1;
        }
    }
    data
}
