mod data;

use anyhow::Result;
use data::load_csv;
use data::raw_loader::load_raw_csv;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("1st approach: Load CSV with raw loader");
    let path = PathBuf::from("datasets/sample.csv");

    let (headers, data) = load_raw_csv(&path);

    println!(
        "Loaded dataset with {} rows and {} columns!",
        data.len(),
        headers.len()
    );
    println!("------------------");
    println!("Headers: {:?}", headers);
    println!("------------------");

    for row in data.iter().take(2) {
        println!("{:?}", row);
        println!("------------------");
    }
    println!("??????????????????????????????????????");
    println!("??????????????????????????????????????");
    println!("??????????????????????????????????????");
    println!("2nd approach: Load CSV with INFER");
    let dataset = load_csv("datasets/sample.csv")?;

    println!("Successfully loaded dataset!");
    println!(
        "Shape: {} rows × {} columns",
        dataset.shape().0,
        dataset.shape().1
    );

    if let Some(first_row) = dataset.records.first() {
        println!("\nFirst row preview:");
        for (header, value) in first_row {
            println!("  {:15}: {:?}", header, value);
        }
    }

    Ok(())
}
