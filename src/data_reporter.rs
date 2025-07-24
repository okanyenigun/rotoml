use polars::prelude::*;
use std::error::Error;
use std::fs;

pub struct DataReporter;

impl DataReporter {
    pub fn generate_data_report(
        df: &DataFrame,
        file_name: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut report = String::new();

        // Report header
        report.push_str("# DATA ANALYSIS REPORT\n\n");
        report.push_str("## DATA\n\n");

        // file information
        report.push_str(&format!("**File Name:** `{}`\n\n", file_name));

        // DataFrame shape
        let shape: (usize, usize) = df.shape();
        report.push_str(&format!(
            "**DataFrame Shape:** {} rows, {} columns\n\n",
            shape.0, shape.1
        ));

        // column types
        report.push_str("### Column Types\n\n");
        report.push_str("| Column Name | Data Type |\n");
        report.push_str("|-------------|----------|\n");

        for (name, dtype) in df.get_column_names().iter().zip(df.dtypes()) {
            report.push_str(&format!("| {} | {:?} |\n", name, dtype));
        }
        report.push_str("\n");

        // Missing Values
        report.push_str("### Missing Values Analysis\n\n");
        report.push_str("| Column Name | Missing Count | Missing Percentage | Non-null Count |\n");
        report.push_str("|-------------|---------------|-------------------|----------------|\n");

        let null_counts: DataFrame = df.null_count();
        let total_rows: f64 = shape.0 as f64;

        for (i, column_name) in df.get_column_names().iter().enumerate() {
            let null_count: AnyValue = null_counts.get_columns()[i].get(0).unwrap();
            let missing_count: u64 = match null_count {
                AnyValue::UInt32(val) => val as u64,
                AnyValue::UInt64(val) => val,
                _ => 0u64,
            };

            let missing_percentage: f64 = (missing_count as f64 / total_rows) * 100.0;
            let non_null_count: usize = shape.0 - missing_count as usize;

            report.push_str(&format!(
                "| {} | {} | {:.2}% | {} |\n",
                column_name, missing_count, missing_percentage, non_null_count
            ));
        }
        report.push_str("\n");

        // data quality
        let total_missing: u64 = (0..df.get_column_names().len())
            .map(|i| {
                let null_count: AnyValue = null_counts.get_columns()[i].get(0).unwrap();
                match null_count {
                    AnyValue::UInt32(val) => val as u64,
                    AnyValue::UInt64(val) => val,
                    _ => 0u64,
                }
            })
            .sum();

        let total_cells: u64 = (shape.0 as u64) * (shape.1 as u64);
        let completeness_percentage: f64 =
            ((total_cells - total_missing) as f64 / total_cells as f64) * 100.0;

        report.push_str("### Data Quality Summary\n\n");
        report.push_str(&format!("- **Total Cells:** {}\n", total_cells));
        report.push_str(&format!("- **Missing Cells:** {}\n", total_missing));
        report.push_str(&format!(
            "- **Data completeness:** {:.2}%\n",
            completeness_percentage
        ));
        report.push_str(&format!(
            "- **Numeric Columns:** {}\n",
            Self::count_numeric_columns(df)
        ));
        report.push_str(&format!(
            "- **Categorical Columns:** {}\n",
            Self::count_categorical_columns(df)
        ));

        // write report to file
        fs::write(output_path, report)?;
        println!("Data Report Saved to: {}", output_path);

        Ok(())
    }

    // Count numeric columns
    fn count_numeric_columns(df: &DataFrame) -> usize {
        df.dtypes()
            .iter()
            .filter(|dtype| {
                matches!(
                    dtype,
                    DataType::Int64 | DataType::Float64 | DataType::Int32 | DataType::Float32
                )
            })
            .count()
    }

    // Count categorical columns
    fn count_categorical_columns(df: &DataFrame) -> usize {
        df.dtypes()
            .iter()
            .filter(|dtype| matches!(dtype, DataType::Utf8))
            .count()
    }
}
