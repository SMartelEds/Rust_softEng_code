use crate::log::AggregatedData;
use serde_json;
use std::fs::File;
use std::io::Write;

pub fn write_report(data: &AggregatedData, file_path: &str) -> Result<(), String> {
    let report = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write_all(report.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}
