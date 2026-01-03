# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-01-02

### Added

- **Data Operations Module**: New `data_operations.rs` module with comprehensive data manipulation functions
  - `drop_column`: Drop a single column from a DataFrame
  - `drop_columns`: Drop multiple columns from a DataFrame
  - `drop_rows`: Drop rows by their indexes from a DataFrame
  - `detect_duplicate_columns`: Detect columns with identical data and return duplicates as tuples
  - `count_duplicate_rows`: Count and identify duplicate rows with their indexes
- **Unit Tests**: 13 comprehensive unit tests covering:
  - Column dropping operations (single and multiple)
  - Row dropping operations with validation
  - Duplicate column detection (no duplicates, single group, multiple groups)
  - Duplicate row detection (no duplicates, some duplicates, all duplicates)
  - Error handling for invalid operations

### Changed

- **Data Reporter**: Enhanced to include duplicate analysis in generated reports
  - Duplicate columns detection and reporting
  - Duplicate rows detection with count, percentage, and index listing
  - Improved data quality insights with duplicate detection
- **Main Function**: Updated to test all data operations with comprehensive assertions
  - Tests for drop_column, drop_columns, and drop_rows operations
  - Tests for detect_duplicate_columns and count_duplicate_rows
  - Visual feedback with emojis and detailed output
  - Report generation now uses original DataFrame to capture all data issues

## [0.1.1] - Initial Release

### Added

- Initial project setup with Polars integration
- Data loading from CSV files
- Basic data reporting with:
  - DataFrame shape information
  - Column types analysis
  - Missing values analysis
  - Data quality summary
- Command-line interface using clap
