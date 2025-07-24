use polars::prelude::*;
use std::error::Error;

pub struct DataLoader;

impl DataLoader {
    pub fn load_csv(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
        let df: DataFrame = CsvReader::from_path(file_path)?.has_header(true).finish()?;
        Ok(df)
    }
}
