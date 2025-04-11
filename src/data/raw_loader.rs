use csv::Reader;
use std::path::Path;

/// Load the dataset with all values as strings
/// Returns (headers, rows)
pub fn load_raw_csv<P: AsRef<Path>>(path: P) -> (Vec<String>, Vec<Vec<String>>) {
    let mut reader = Reader::from_path(path).expect("Failed to open CSV file");

    let headers = reader
        .headers()
        .expect("Failed to read headers")
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>();

    let mut data = Vec::new();
    for result in reader.records() {
        let record = result.expect("Failed to read row");
        let row = record
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        data.push(row);
    }

    (headers, data)
}
