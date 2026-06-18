use chrono::NaiveDateTime;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub ip_address: String,
    pub endpoint: String,
    pub response_status: u16,
    pub response_time_ms: u32,
}

#[derive(Debug, Default, Serialize)]
pub struct AggregatedData {
    pub requests_per_ip: std::collections::HashMap<String, usize>,
    pub requests_per_endpoint: std::collections::HashMap<String, usize>,
    pub failed_logins: std::collections::HashMap<String, usize>,
}
