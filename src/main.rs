use clap::Parser;
use polars::prelude::*;
use rotoml::data_loader::DataLoader;
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
        eprintln!("  cargo run -- -f /path/to/your/data.csv");
        std::process::exit(1);
    }

    println!("📊 Analyzing file: {}", args.file);
    println!("📝 Report will be saved to: {}", report_path);

    // Load Data
    let df: DataFrame = DataLoader::load_csv(&args.file)?;

    // Generate data markdown report
    DataReporter::generate_data_report(&df, &args.file, report_path)?;

    println!(
        "\n✅ Data report generated successfully! Check '{}'.",
        report_path
    );

    Ok(())
}
