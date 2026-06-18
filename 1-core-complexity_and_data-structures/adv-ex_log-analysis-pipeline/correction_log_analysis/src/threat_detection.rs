use crate::log::AggregatedData;

// Detects potential security threats based on failed login attempts.
// Returns a vector of IP addresses with failed login attempts exceeding the threshold.
pub fn detect_threats(data: &AggregatedData, threshold: usize) -> Vec<String> {
    data.failed_logins
        .iter()
        // Filter IPs with failed login attempts greater than or equal to the threshold.
        .filter(|(_, count)| **count >= threshold)
        // Extract the IP addresses.
        .map(|(ip, _)| ip.clone())
        .collect()
}
