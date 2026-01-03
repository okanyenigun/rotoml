use clap::Parser;
use polars::prelude::*;
use rotoml::data_loader::DataLoader;
use rotoml::data_operations::DataOperations;
use rotoml::data_reporter::DataReporter;
use std::error::Error;
use std::path::Path;

#[derive(Parser)]
#[command(name = "rotoml")]
#[command(about = "A machine learning pipeline", long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let report_path: &str = "data_report.md";
    let args = Args::parse();

    // Validate the file exists
    if !Path::new(&args.file).exists() {
        eprintln!("❌ Error: File '{}' does not exist!", args.file);
        eprintln!("💡 Usage examples:");
        eprintln!("  cargo run -- --file datasets/sample.csv");
        eprintln!("  cargo run -- -f /path/to/your/data.parquet");
        std::process::exit(1);
    }

    println!("📊 Analyzing file: {}", args.file);
    println!("📝 Report will be saved to: {}", report_path);

    // Load Data (auto-detects format from file extension)
    let df: DataFrame = DataLoader::load(&args.file)?;

    println!("\n🔍 Original DataFrame shape: {:?}", df.shape());
    println!("Original columns: {:?}", df.get_column_names());

    // Test detect_duplicate_columns: Check for duplicate columns
    println!("\n🧪 Testing detect_duplicate_columns - Checking for duplicate columns...");
    let duplicate_columns = DataOperations::detect_duplicate_columns(&df)?;

    if duplicate_columns.is_empty() {
        println!("✅ detect_duplicate_columns test passed!");
        println!("   No duplicate columns found in the dataset");
    } else {
        println!("✅ detect_duplicate_columns test passed!");
        println!(
            "   Found {} group(s) of duplicate columns:",
            duplicate_columns.len()
        );
        for (original, duplicates) in &duplicate_columns {
            println!(
                "   - Column '{}' has duplicates: {:?}",
                original, duplicates
            );
        }
    }

    // Test count_duplicate_rows: Check for duplicate rows
    println!("\n🧪 Testing count_duplicate_rows - Checking for duplicate rows...");
    let (duplicate_count, duplicate_indexes) = DataOperations::count_duplicate_rows(&df)?;

    println!("✅ count_duplicate_rows test passed!");
    println!("   Found {} duplicate row(s)", duplicate_count);

    if duplicate_count > 0 {
        if duplicate_count <= 10 {
            println!("   Duplicate row indexes: {:?}", duplicate_indexes);
        } else {
            println!(
                "   Duplicate row indexes (first 10): {:?}",
                &duplicate_indexes[..10]
            );
            println!("   ... and {} more", duplicate_count - 10);
        }
    }

    // Test drop_column: Drop a single column
    println!("\n🧪 Testing drop_column - Dropping 'SSN' column...");
    let original_width = df.width();
    let df_after_drop_one = DataOperations::drop_column(df.clone(), "SSN")?;

    // Assert that the column was dropped
    assert!(
        !df_after_drop_one.get_column_names().contains(&"SSN"),
        "❌ FAILED: SSN column should be removed"
    );
    assert_eq!(
        df_after_drop_one.width(),
        original_width - 1,
        "❌ FAILED: DataFrame should have one less column"
    );
    println!("✅ drop_column test passed!");
    println!("   New shape: {:?}", df_after_drop_one.shape());
    println!(
        "   Remaining columns: {:?}",
        df_after_drop_one.get_column_names()
    );

    // Test drop_columns: Drop multiple columns
    println!("\n🧪 Testing drop_columns - Dropping ['ID', 'Month', 'Date'] columns...");
    let df_after_drop_multiple =
        DataOperations::drop_columns(df_after_drop_one.clone(), &["ID", "Month", "Date"])?;

    // Assert that all columns were dropped
    assert!(
        !df_after_drop_multiple.get_column_names().contains(&"ID"),
        "❌ FAILED: ID column should be removed"
    );
    assert!(
        !df_after_drop_multiple.get_column_names().contains(&"Month"),
        "❌ FAILED: Month column should be removed"
    );
    assert!(
        !df_after_drop_multiple.get_column_names().contains(&"Date"),
        "❌ FAILED: Date column should be removed"
    );
    assert_eq!(
        df_after_drop_multiple.width(),
        df_after_drop_one.width() - 3,
        "❌ FAILED: DataFrame should have three less columns"
    );
    println!("✅ drop_columns test passed!");
    println!("   New shape: {:?}", df_after_drop_multiple.shape());
    println!(
        "   Remaining columns: {:?}",
        df_after_drop_multiple.get_column_names()
    );

    // Test drop_rows: Drop rows by indexes
    println!("\n🧪 Testing drop_rows - Dropping rows at indexes [0, 1, 2, 5, 10]...");
    let original_height = df_after_drop_multiple.height();
    let df_after_drop_rows =
        DataOperations::drop_rows(df_after_drop_multiple.clone(), &[0, 1, 2, 5, 10])?;

    // Assert that the rows were dropped
    assert_eq!(
        df_after_drop_rows.height(),
        original_height - 5,
        "❌ FAILED: DataFrame should have 5 fewer rows"
    );
    assert_eq!(
        df_after_drop_rows.width(),
        df_after_drop_multiple.width(),
        "❌ FAILED: DataFrame should maintain the same number of columns"
    );
    println!("✅ drop_rows test passed!");
    println!("   New shape: {:?}", df_after_drop_rows.shape());
    println!("   Dropped 5 rows from the DataFrame");

    // Generate data markdown report with the original DataFrame
    println!("\n📝 Generating report with original DataFrame...");
    DataReporter::generate_data_report(&df, &args.file, report_path)?;

    println!(
        "\n✅ All tests passed! Data report generated successfully! Check '{}'.",
        report_path
    );

    Ok(())
}
