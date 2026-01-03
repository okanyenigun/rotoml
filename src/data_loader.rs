use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct DataLoader;

impl DataLoader {
    /// Load a CSV file into a DataFrame
    ///
    /// # Arguments
    /// * `file_path` - Path to the CSV file
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>` - The loaded DataFrame
    pub fn load_csv(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
        let df: DataFrame = CsvReader::from_path(file_path)?.has_header(true).finish()?;
        Ok(df)
    }

    /// Load a Parquet file into a DataFrame
    ///
    /// # Arguments
    /// * `file_path` - Path to the Parquet file
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>` - The loaded DataFrame
    pub fn load_parquet(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
        let df: DataFrame = ParquetReader::new(File::open(file_path)?).finish()?;
        Ok(df)
    }

    /// Load a file automatically detecting the format based on file extension
    ///
    /// # Arguments
    /// * `file_path` - Path to the file (CSV or Parquet)
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>` - The loaded DataFrame
    ///
    /// # Supported formats
    /// - `.csv` - CSV files
    /// - `.parquet` - Parquet files
    pub fn load(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
        let path = Path::new(file_path);
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or("Unable to determine file extension")?;

        match extension.to_lowercase().as_str() {
            "csv" => Self::load_csv(file_path),
            "parquet" => Self::load_parquet(file_path),
            _ => Err(format!("Unsupported file format: .{}", extension).into()),
        }
    }
}
