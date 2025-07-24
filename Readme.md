# RotoML

A native Rust AutoML toolkit for machine learning prototyping and model development.

🚧 **Early Development**: Currently implements data analysis pipeline. Full automated ML pipeline coming soon.

## Current Features

- Fast CSV data loading with Polars
- Automated data quality analysis
- Missing value detection and reporting
- Data profiling and statistics

## Future Vision

RotoML will become a complete automated machine learning pipeline that handles:

- Data preprocessing and feature engineering
- Model selection and hyperparameter tuning
- Model training and evaluation
- Pipeline orchestration

## Installation

```bash
cargo install rotoml
```

## Usage

```bash
# Analyze your data
rotoml --file your_data.csv
```

Generates a comprehensive data report in Markdown format.

## License

MIT
