use std::collections::{HashMap, HashSet};

use crate::models::{Status, StudentRecord};

#[derive(Debug, Default)]
pub struct Summary {
    pub total_count: usize,
    pub active_count: usize,
    pub average_score: Option<f32>,
    pub status_count: HashMap<Status, usize>,
    pub unique_names: HashSet<String>,
}

pub fn summarize(records: &[StudentRecord]) -> Summary {
    // TODO Exercise 4:
    // - total_count = records.len()
    // - active_count = count active records
    // - average_score = average of present scores only
    // - status_count = HashMap<Status, usize>
    // - unique_names = HashSet<String>
    let _ = records;
    todo!("Implement summary aggregation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_counts_records() {
        let records = vec![
            StudentRecord::new(1, "Alice".to_string(), Some(23), Some(80.0), Status::Active),
            StudentRecord::new(2, "Bob".to_string(), None, Some(60.0), Status::Inactive),
        ];

        let summary = summarize(&records);
        assert_eq!(summary.total_count, 2);
        assert_eq!(summary.active_count, 1);
        assert_eq!(summary.average_score, Some(70.0));
        assert!(summary.unique_names.contains("Alice"));
    }
}
