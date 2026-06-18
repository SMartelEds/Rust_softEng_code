use crate::models::{Status, StudentRecord};

pub fn parse_optional_u8(input: &str) -> Option<u8> {
    // TODO Exercise 3:
    // - empty string => None
    // - valid u8 => Some(value)
    // - invalid number => None
    todo!("Implement optional u8 parsing")
}

pub fn parse_optional_f32(input: &str) -> Option<f32> {
    // TODO Exercise 3: same idea as parse_optional_u8, but for f32.
    todo!("Implement optional f32 parsing")
}

pub fn parse_line(line: &str) -> Result<StudentRecord, String> {
    // TODO Exercise 3:
    // 1. split by comma
    // 2. require exactly 5 fields
    // 3. parse id as u32, map the error to String, and use ?
    // 4. parse optional age and optional score
    // 5. convert status using Status::from_str
    // 6. return Ok(StudentRecord::new(...))
    let _ = line;
    todo!("Implement line parsing")
}

pub fn parse_many(input: &str) -> Vec<StudentRecord> {
    // TODO Exercise 6:
    // Convert all valid lines into StudentRecord and skip invalid lines.
    // Hint: input.lines().filter_map(|line| parse_line(line).ok()).collect()
    let _ = input;
    todo!("Implement parse_many")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_optional_values() {
        assert_eq!(parse_optional_u8("23"), Some(23));
        assert_eq!(parse_optional_u8(""), None);
        assert_eq!(parse_optional_u8("not a number"), None);
    }

    #[test]
    fn parses_valid_line() {
        let record = parse_line("1,Alice,23,88.5,active").unwrap();
        assert_eq!(record.id, 1);
        assert_eq!(record.name, "Alice");
        assert_eq!(record.age, Some(23));
        assert_eq!(record.score, Some(88.5));
        assert_eq!(record.status, Status::Active);
    }

    #[test]
    fn rejects_bad_field_count() {
        assert!(parse_line("bad,line").is_err());
    }
}
