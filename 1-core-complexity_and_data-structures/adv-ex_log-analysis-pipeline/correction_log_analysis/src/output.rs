use crate::log::AggregatedData;
use std::fs::File;
use std::io::Write;

// Writes the aggregated data to a JSON file.
// Returns `Result<(), String>` to handle potential errors during file writing.
pub fn write_report(data: &AggregatedData, file_path: &str) -> Result<(), String> {
    // Serialize the aggregated data to a JSON string.
    let report = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    // Create or overwrite the file at the given path.
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    // Write the JSON string to the file.
    file.write_all(report.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}
