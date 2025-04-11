use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Schema mismatch in record {index}")]
    SchemaMismatch {
        index: usize,
        missing: Vec<String>,
        extra: Vec<String>,
    },
}
