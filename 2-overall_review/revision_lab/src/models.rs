#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    Active,
    Inactive,
    Unknown,
}

impl Status {
    pub fn from_str(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "active" => Status::Active,
            "inactive" => Status::Inactive,
            _ => Status::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudentRecord {
    pub id: u32,
    pub name: String,
    pub age: Option<u8>,
    pub score: Option<f32>,
    pub status: Status,
}

impl StudentRecord {
    pub fn new(
        id: u32,
        name: String,
        age: Option<u8>,
        score: Option<f32>,
        status: Status,
    ) -> Self {
        Self {
            id,
            name,
            age,
            score,
            status,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, Status::Active)
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }
}

pub trait Validate {
    fn is_valid(&self) -> bool;

    fn validation_message(&self) -> &'static str {
        if self.is_valid() {
            "valid"
        } else {
            "invalid"
        }
    }
}

impl Validate for StudentRecord {
    fn is_valid(&self) -> bool {
        // TODO Exercise 5:
        // - name must not be empty
        // - age must be <= 120 if present
        // - status must not be Unknown
        todo!("Implement validation logic")
    }
}

pub trait HasScore {
    fn score_value(&self) -> Option<f32>;
}

impl HasScore for StudentRecord {
    fn score_value(&self) -> Option<f32> {
        self.score
    }
}

pub fn has_score_above<T: HasScore>(item: &T, threshold: f32) -> bool {
    // TODO Exercise 5: return true if item has a score and score > threshold.
    todo!("Implement generic score helper")
}

pub fn best_by_score<'a>(a: &'a StudentRecord, b: &'a StudentRecord) -> &'a StudentRecord {
    // TODO Exercise 5:
    // Compare a.score and b.score, using 0.0 when score is None.
    // Return a reference to the best record.
    todo!("Implement lifetime exercise")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_str_is_case_insensitive() {
        assert_eq!(Status::from_str("ACTIVE"), Status::Active);
        assert_eq!(Status::from_str("inactive"), Status::Inactive);
        assert_eq!(Status::from_str("anything"), Status::Unknown);
    }
}
