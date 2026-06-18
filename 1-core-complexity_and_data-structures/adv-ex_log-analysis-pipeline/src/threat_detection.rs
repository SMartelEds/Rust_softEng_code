use crate::log::AggregatedData;

pub fn detect_threats(data: &AggregatedData, threshold: usize) -> Vec<String> {
    data.failed_logins
        .iter()
        .filter(|(_, &count)| count >= threshold)
        .map(|(ip, _)| ip.clone())
        .collect()
}
