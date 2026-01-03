# DATA ANALYSIS REPORT

## DATA

**File Name:** `datasets/sample.csv`

**DataFrame Shape:** 1200 rows, 27 columns

### Column Types

| Column Name | Data Type |
|-------------|----------|
| ID | Int64 |
| Customer_ID | Utf8 |
| Month | Utf8 |
| Name | Utf8 |
| SSN | Utf8 |
| Occupation | Utf8 |
| Annual_Income | Float64 |
| Monthly_Inhand_Salary | Float64 |
| Num_Bank_Accounts | Int64 |
| Num_Credit_Card | Int64 |
| Interest_Rate | Int64 |
| Num_of_Loan | Float64 |
| Type_of_Loan | Utf8 |
| Delay_from_due_date | Int64 |
| Num_Credit_Inquiries | Float64 |
| Credit_Mix | Utf8 |
| Outstanding_Debt | Float64 |
| Credit_Utilization_Ratio | Float64 |
| Credit_History_Age | Utf8 |
| Payment_of_Min_Amount | Utf8 |
| Total_EMI_per_month | Float64 |
| Amount_invested_monthly | Float64 |
| Payment_Behaviour | Utf8 |
| Monthly_Balance | Float64 |
| Credit_Score | Utf8 |
| Date | Utf8 |
| ID_copy | Int64 |

### Missing Values Analysis

| Column Name | Missing Count | Missing Percentage | Non-null Count |
|-------------|---------------|-------------------|----------------|
| ID | 0 | 0.00% | 1200 |
| Customer_ID | 0 | 0.00% | 1200 |
| Month | 0 | 0.00% | 1200 |
| Name | 123 | 10.25% | 1077 |
| SSN | 0 | 0.00% | 1200 |
| Occupation | 0 | 0.00% | 1200 |
| Annual_Income | 80 | 6.67% | 1120 |
| Monthly_Inhand_Salary | 176 | 14.67% | 1024 |
| Num_Bank_Accounts | 0 | 0.00% | 1200 |
| Num_Credit_Card | 0 | 0.00% | 1200 |
| Interest_Rate | 0 | 0.00% | 1200 |
| Num_of_Loan | 52 | 4.33% | 1148 |
| Type_of_Loan | 194 | 16.17% | 1006 |
| Delay_from_due_date | 0 | 0.00% | 1200 |
| Num_Credit_Inquiries | 28 | 2.33% | 1172 |
| Credit_Mix | 0 | 0.00% | 1200 |
| Outstanding_Debt | 13 | 1.08% | 1187 |
| Credit_Utilization_Ratio | 0 | 0.00% | 1200 |
| Credit_History_Age | 117 | 9.75% | 1083 |
| Payment_of_Min_Amount | 0 | 0.00% | 1200 |
| Total_EMI_per_month | 0 | 0.00% | 1200 |
| Amount_invested_monthly | 107 | 8.92% | 1093 |
| Payment_Behaviour | 0 | 0.00% | 1200 |
| Monthly_Balance | 23 | 1.92% | 1177 |
| Credit_Score | 0 | 0.00% | 1200 |
| Date | 0 | 0.00% | 1200 |
| ID_copy | 0 | 0.00% | 1200 |

### Data Quality Summary

- **Total Cells:** 32400
- **Missing Cells:** 913
- **Data completeness:** 97.18%
- **Numeric Columns:** 15
- **Categorical Columns:** 12

### Duplicate Columns Analysis

⚠️ **Found 1 group(s) of duplicate columns:**

- Column `ID` is duplicated by:
  - `ID_copy`

### Duplicate Rows Analysis

⚠️ **Found 200 duplicate row(s):**

- **Total duplicate rows:** 200
- **Percentage of duplicates:** 16.67%
- **Duplicate row indexes (first 20):** [1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1019]
  - ... and 180 more

