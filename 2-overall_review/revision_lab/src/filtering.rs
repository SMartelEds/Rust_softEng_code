use crate::models::{Status, StudentRecord};

pub fn filter_valid(records: Vec<StudentRecord>) -> Vec<StudentRecord> {
    // TODO Exercise 4:
    // Keep records where:
    // - name is not empty
    // - age is <= 120 if age exists
    // - status is not Unknown
    let _ = records;
    todo!("Implement valid record filtering")
}

pub fn active_names(records: &[StudentRecord]) -> Vec<String> {
    // TODO Exercise 4:
    // Borrow records, keep only active students, return their names as Vec<String>.
    let _ = records;
    todo!("Implement active_names with iterators")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record(name: &str, age: Option<u8>, status: Status) -> StudentRecord {
        StudentRecord::new(1, name.to_string(), age, Some(80.0), status)
    }

    #[test]
    fn removes_invalid_records() {
        let records = vec![
            sample_record("Alice", Some(23), Status::Active),
            sample_record("", Some(23), Status::Active),
            sample_record("Old", Some(121), Status::Active),
            sample_record("Unknown", Some(20), Status::Unknown),
        ];

        let valid = filter_valid(records);
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "Alice");
    }
}
