use super::{DataError, DataValue, Dataset};
use csv::Reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

pub fn load_csv<P: AsRef<Path>>(path: P) -> Result<Dataset, DataError> {
    let mut reader = Reader::from_path(path)?;
    let headers = reader
        .headers()?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let header_set: HashSet<_> = headers.iter().collect();

    let records = reader
        .records()
        .enumerate()
        .map(|(i, result)| {
            let record = result?;
            let row: HashMap<String, DataValue> = headers
                .iter()
                .zip(record.iter())
                .map(|(h, v)| (h.clone(), parse_field(v)))
                .collect();

            let keys: HashSet<_> = row.keys().collect();
            if keys != header_set {
                let missing = header_set.difference(&keys).cloned().cloned().collect();
                let extra = keys.difference(&header_set).cloned().cloned().collect();
                return Err(DataError::SchemaMismatch {
                    index: i,
                    missing,
                    extra,
                });
            }

            Ok(row)
        })
        .collect::<Result<Vec<_>, DataError>>()?;

    Ok(Dataset::new(headers, records))
}

fn parse_field(field: &str) -> DataValue {
    if field.is_empty() {
        return DataValue::Null;
    }

    // Attempt numeric parsing first
    if let Ok(num) = field.parse::<f64>() {
        return DataValue::Numeric(num);
    }

    // Then try integer parsing
    if let Ok(int) = field.parse::<i64>() {
        return DataValue::Integer(int);
    }

    // Check for boolean values
    match field.to_lowercase().as_str() {
        "true" => DataValue::Boolean(true),
        "false" => DataValue::Boolean(false),
        _ => DataValue::Text(field.to_string()),
    }
}
