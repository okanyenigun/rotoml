use polars::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};

pub struct DataOperations;

impl DataOperations {
    /// Drop a single column from the DataFrame
    ///
    /// # Arguments
    /// * `df` - The DataFrame to operate on
    /// * `column_name` - The name of the column to drop
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>` - A new DataFrame with the column removed
    pub fn drop_column(mut df: DataFrame, column_name: &str) -> Result<DataFrame, Box<dyn Error>> {
        // Check if the column exists
        if !df.get_column_names().contains(&column_name) {
            return Err(format!("Column '{}' does not exist in the DataFrame", column_name).into());
        }
        let _ = df.drop_in_place(column_name)?;
        Ok(df)
    }

    /// Drop multiple columns from the DataFrame
    ///
    /// # Arguments
    /// * `df` - The DataFrame to operate on
    /// * `column_names` - A slice of column names to drop
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>` - A new DataFrame with the columns removed
    pub fn drop_columns(
        mut df: DataFrame,
        column_names: &[&str],
    ) -> Result<DataFrame, Box<dyn Error>> {
        // snapshot of existing columns
        let existing: HashSet<&str> = df.get_column_names().into_iter().collect();

        // Validate
        let mut unique: Vec<&str> = Vec::new();
        let mut seen: HashSet<&str> = HashSet::new();

        for &col in column_names {
            if !existing.contains(col) {
                return Err(format!("Column '{}' does not exist in the DataFrame", col).into());
            }

            if seen.insert(col) {
                unique.push(col);
            }
        }

        for col in unique {
            let _ = df.drop_in_place(col)?;
        }

        Ok(df)
    }

    /// Drop rows by their indexes from the DataFrame
    ///
    /// # Arguments
    /// * `df` - The DataFrame to operate on
    /// * ìndexes` - indexes to drop
    ///
    /// # Returns
    /// * `Result<DataFrame, Box<dyn Error>>`
    pub fn drop_rows(df: DataFrame, indexes: &[usize]) -> Result<DataFrame, Box<dyn Error>> {
        let total_rows = df.height();

        if indexes.is_empty() {
            return Ok(df);
        }

        let mut drop_set: HashSet<usize> = HashSet::with_capacity(indexes.len());
        for &idx in indexes {
            if idx >= total_rows {
                return Err(format!(
                    "Index '{}' is out of bounds (DataFrame has {} rows)",
                    idx, total_rows
                )
                .into());
            }
            drop_set.insert(idx);
        }
        let mask: Vec<bool> = (0..total_rows).map(|i| !drop_set.contains(&i)).collect();
        let mask_series = BooleanChunked::new("mask", &mask);
        let filtered_df = df.filter(&mask_series)?;

        Ok(filtered_df)
    }

    /// Detect duplicate columns
    ///
    /// # Arguments
    /// * `df` - The DataFrame to analyze
    ///
    /// # Returns
    /// * `Result<Vec<(String, Vec<String>)>, Box<dyn Error>>` - A vector of tuples where
    /// - the first element is the original column name
    /// - the second element is a vector of duplicate column names that have the same data
    pub fn detect_duplicate_columns(
        df: &DataFrame,
    ) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
        let mut duplicate_columns: Vec<(String, Vec<String>)> = Vec::new();
        let mut seen_columns: HashMap<u64, String> = HashMap::new();
        let mut duplicate_map: HashMap<String, Vec<String>> = HashMap::new();

        for column_name in df.get_column_names() {
            let column = df.column(column_name)?;
            let mut hasher = DefaultHasher::new();

            for i in 0..column.len() {
                let value = column.get(i)?;
                format!("{:?}", value).hash(&mut hasher);
            }

            let column_hash = hasher.finish();

            if let Some(original_column) = seen_columns.get(&column_hash) {
                if let Some(duplicates) = duplicate_map.get_mut(original_column) {
                    duplicates.push(column_name.to_string());
                } else {
                    duplicate_map.insert(original_column.clone(), vec![column_name.to_string()]);
                }
            } else {
                seen_columns.insert(column_hash, column_name.to_string());
            }
        }

        for (original, duplicates) in duplicate_map {
            duplicate_columns.push((original, duplicates));
        }

        Ok(duplicate_columns)
    }

    /// Count duplicate rows in the DataFrame
    ///     
    /// # Arguments
    /// * `df` - The DataFrame to analyze
    ///
    /// # Returns
    /// * `Result<(usize, Vec<usize>), Box<dyn Error>>` - A tuple where
    ///   - the first element is the count of duplicate rows
    ///   - the second element is a vector of indexes of duplicate rows
    pub fn count_duplicate_rows(df: &DataFrame) -> Result<(usize, Vec<usize>), Box<dyn Error>> {
        let total_rows = df.height();
        let mut seen_rows: HashMap<u64, usize> = HashMap::new();
        let mut duplicate_indexes: Vec<usize> = Vec::new();

        for row_idx in 0..total_rows {
            let mut hasher = DefaultHasher::new();

            for column_name in df.get_column_names() {
                let column = df.column(column_name)?;
                let value = column.get(row_idx)?;
                format!("{:?}", value).hash(&mut hasher);
            }

            let row_hash = hasher.finish();

            if seen_rows.contains_key(&row_hash) {
                duplicate_indexes.push(row_idx);
            } else {
                seen_rows.insert(row_hash, row_idx);
            }
        }
        let duplicate_count = duplicate_indexes.len();
        Ok((duplicate_count, duplicate_indexes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_column() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[4, 5, 6],
            "c" => &[7, 8, 9],
        }
        .unwrap();

        let result = DataOperations::drop_column(df.clone(), "b").unwrap();

        assert_eq!(result.width(), 2);
        assert!(result.get_column_names().contains(&"a"));
        assert!(result.get_column_names().contains(&"c"));
        assert!(!result.get_column_names().contains(&"b"));
    }

    #[test]
    fn test_drop_column_not_exists() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[4, 5, 6],
        }
        .unwrap();

        let result = DataOperations::drop_column(df, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_drop_columns() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[4, 5, 6],
            "c" => &[7, 8, 9],
            "d" => &[10, 11, 12],
        }
        .unwrap();

        let result = DataOperations::drop_columns(df.clone(), &["b", "d"]).unwrap();

        assert_eq!(result.width(), 2);
        assert!(result.get_column_names().contains(&"a"));
        assert!(result.get_column_names().contains(&"c"));
        assert!(!result.get_column_names().contains(&"b"));
        assert!(!result.get_column_names().contains(&"d"));
    }

    #[test]
    fn test_drop_columns_one_not_exists() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[4, 5, 6],
        }
        .unwrap();

        let result = DataOperations::drop_columns(df, &["b", "nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_drop_rows() {
        let df = df! {
            "a" => &[1, 2, 3, 4, 5],
            "b" => &[10, 20, 30, 40, 50],
        }
        .unwrap();

        // Drop rows at indexes 1 and 3
        let result = DataOperations::drop_rows(df.clone(), &[1, 3]).unwrap();

        assert_eq!(result.height(), 3); // Should have 3 rows left
        assert_eq!(result.width(), 2); // Should still have 2 columns

        // Check remaining values
        let col_a = result.column("a").unwrap();
        let a_values: Vec<Option<i32>> = col_a.i32().unwrap().into_iter().collect();
        assert_eq!(a_values, vec![Some(1), Some(3), Some(5)]);
    }

    #[test]
    fn test_drop_rows_single() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[10, 20, 30],
        }
        .unwrap();

        let result = DataOperations::drop_rows(df.clone(), &[0]).unwrap();

        assert_eq!(result.height(), 2);
        let col_a = result.column("a").unwrap();
        let a_values: Vec<Option<i32>> = col_a.i32().unwrap().into_iter().collect();
        assert_eq!(a_values, vec![Some(2), Some(3)]);
    }

    #[test]
    fn test_drop_rows_out_of_bounds() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[10, 20, 30],
        }
        .unwrap();

        let result = DataOperations::drop_rows(df, &[5]);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_duplicate_columns() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[1, 2, 3],  // duplicate of 'a'
            "c" => &[4, 5, 6],
            "d" => &[1, 2, 3],  // duplicate of 'a'
        }
        .unwrap();

        let result = DataOperations::detect_duplicate_columns(&df).unwrap();

        assert_eq!(result.len(), 1); // Should find one group of duplicates

        let (original, duplicates) = &result[0];
        assert_eq!(original, "a");
        assert_eq!(duplicates.len(), 2);
        assert!(duplicates.contains(&"b".to_string()));
        assert!(duplicates.contains(&"d".to_string()));
    }

    #[test]
    fn test_detect_duplicate_columns_no_duplicates() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[4, 5, 6],
            "c" => &[7, 8, 9],
        }
        .unwrap();

        let result = DataOperations::detect_duplicate_columns(&df).unwrap();

        assert_eq!(result.len(), 0); // No duplicates found
    }

    #[test]
    fn test_detect_duplicate_columns_multiple_groups() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[1, 2, 3],  // duplicate of 'a'
            "c" => &[4, 5, 6],
            "d" => &[4, 5, 6],  // duplicate of 'c'
        }
        .unwrap();

        let result = DataOperations::detect_duplicate_columns(&df).unwrap();

        assert_eq!(result.len(), 2); // Should find two groups of duplicates
    }

    #[test]
    fn test_count_duplicate_rows() {
        let df = df! {
            "a" => &[1, 2, 1, 3, 2],
            "b" => &[10, 20, 10, 30, 20],
        }
        .unwrap();

        let (count, indexes) = DataOperations::count_duplicate_rows(&df).unwrap();

        assert_eq!(count, 2); // Two duplicate rows
        assert_eq!(indexes.len(), 2);
        assert!(indexes.contains(&2)); // Row at index 2 is duplicate of row 0
        assert!(indexes.contains(&4)); // Row at index 4 is duplicate of row 1
    }

    #[test]
    fn test_count_duplicate_rows_no_duplicates() {
        let df = df! {
            "a" => &[1, 2, 3],
            "b" => &[10, 20, 30],
        }
        .unwrap();

        let (count, indexes) = DataOperations::count_duplicate_rows(&df).unwrap();

        assert_eq!(count, 0);
        assert_eq!(indexes.len(), 0);
    }

    #[test]
    fn test_count_duplicate_rows_all_duplicates() {
        let df = df! {
            "a" => &[1, 1, 1],
            "b" => &[10, 10, 10],
        }
        .unwrap();

        let (count, indexes) = DataOperations::count_duplicate_rows(&df).unwrap();

        assert_eq!(count, 2); // Two rows are duplicates (indexes 1 and 2)
        assert_eq!(indexes, vec![1, 2]);
    }
}
