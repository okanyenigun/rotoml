# RotoML

A native Rust AutoML toolkit for machine learning pipelines with powerful data analysis and manipulation capabilities.

[![Crates.io](https://img.shields.io/crates/v/rotoml.svg)](https://crates.io/crates/rotoml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

🚀 **Version 0.1.2**: Enhanced data operations and comprehensive analysis features.

## Features

### Data Loading

- **Multi-format support**: Load CSV and Parquet files
- **Auto-detection**: Automatically detects file format from extension
- **Fast processing**: Built on Polars for high-performance data operations

### Data Operations

- **Column operations**: Drop single or multiple columns
- **Row operations**: Drop rows by index
- **Duplicate detection**: Identify duplicate columns and rows
- **Data validation**: Comprehensive error handling and validation

### Data Analysis & Reporting

- **Automated analysis**: Generate comprehensive data reports in Markdown
- **Quality metrics**: Missing values, data completeness, type analysis
- **Duplicate analysis**: Detect and report duplicate columns and rows
- **Statistical insights**: Numeric and categorical column counts

## Installation

```bash
cargo install rotoml
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
rotoml = "0.1.2"
```

## Usage

### Command Line

```bash
# Analyze CSV file
rotoml --file data.csv

# Analyze Parquet file
rotoml --file data.parquet
```

### As a Library

```rust
use rotoml::data_loader::DataLoader;
use rotoml::data_operations::DataOperations;
use rotoml::data_reporter::DataReporter;

// Load data
let df = DataLoader::load("data.csv")?;

// Detect duplicates
let (dup_count, dup_indexes) = DataOperations::count_duplicate_rows(&df)?;
let duplicate_columns = DataOperations::detect_duplicate_columns(&df)?;

// Drop columns
let df = DataOperations::drop_columns(df, &["col1", "col2"])?;

// Drop rows
let df = DataOperations::drop_rows(df, &[0, 5, 10])?;

// Generate report
DataReporter::generate_data_report(&df, "data.csv", "report.md")?;
```

## API Documentation

### DataLoader

- `load(file_path)` - Auto-detect and load CSV or Parquet
- `load_csv(file_path)` - Load CSV file
- `load_parquet(file_path)` - Load Parquet file

### DataOperations

- `drop_column(df, column_name)` - Drop a single column
- `drop_columns(df, column_names)` - Drop multiple columns
- `drop_rows(df, indexes)` - Drop rows by index
- `detect_duplicate_columns(df)` - Find duplicate columns
- `count_duplicate_rows(df)` - Count and list duplicate rows

### DataReporter

- `generate_data_report(df, file_name, output_path)` - Generate comprehensive analysis report

## Output Example

The generated report includes:

- DataFrame shape and column types
- Missing values analysis with percentages
- Data quality metrics
- Duplicate columns detection
- Duplicate rows analysis with indexes

## Future Vision

RotoML is evolving into a complete automated machine learning pipeline:

- Feature engineering and selection
- Model selection and hyperparameter tuning
- Automated training and evaluation
- Pipeline orchestration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Author

Okan Yenigün (okanyenigun@gmail.com)
