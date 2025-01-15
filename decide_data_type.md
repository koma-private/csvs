# Decide Data Type for Each Column

**csvs** scans each row of the input CSV to determine the final data type for creating a table on SQLite.

The following steps outline how **csvs** ensures the most accurate data type assignment for columns when importing CSV
files into SQLite.

Refer to [Validating Number Document](validating_number.md) for details of how **csvs** interprets input data.

## Process

1. Detect the data type of cell in the current row.
2. If processing the first row of the CSV, save the detected data type.
3. Proceed to the next row and detect the data type of the cell in the same column.
4. Compare the previously detected data type with the newly detected type based on the rules in the table below:
    - If the previously detected data type is `INTEGER` and the newly detected type is `REAL`, **csvs** updates the
      column's data type to `REAL` (decimal number)
    - If the previously detected data type is `REAL` and the newly detected type is `INTEGER`, **csvs** retains `REAL` as
      the column's data type.

### Conversion Table
The table below indicates whether a column's data type can convert from one type to another during the import process.

| From / To | `INTEGER` | `REAL` | `TEXT` |
|:---------:|:---------:|:------:|:------:|
| `INTEGER` |    YES    |  YES   |  YES   |
|  `REAL`   |    NO     |  YES   |  YES   |
|  `TEXT`   |    NO     |   NO   |  YES   |

## Example

Below is an example demonstrating the input CSV and the final result.

If any empty cells are encountered in a column, it will be marked as nullable, allowing `NULL` values in the database.

### Input CSV

|   | Keep Integer | Keep Real | Keep Text | Integer to Real | Integer to Text | Real to Text |
|---|:------------:|:---------:|:---------:|:---------------:|:---------------:|:------------:|
| 1 |      3       |    1.0    |     a     |        2        |       56        |     3.4      |
| 2 |      -0      |   -1.1    |     2     |        1        |                 |      A       |
| 3 |      2       |    99     |   -0.9    |       0.3       |        C        |     -2.3     |

### Final Data Type on SQLite Database

| Column Name     | SQLite Type | Allows `NULL` |
|-----------------|:-----------:|:-------------:|
| Keep Integer    |  `INTEGER`  |  `NOT NULL`   |
| Keep Real       |   `REAL`    |  `NOT NULL`   |
| Keep Text       |   `TEXT`    |  `NOT NULL`   |
| Integer to Real |   `REAL`    |  `NOT NULL`   |
| Integer to Text |   `TEXT`    |               |
| Real to Text    |   `TEXT`    |  `NOT NULL`   |


## Range of valid number

**csvs** imports integer number as 64-bit signed type and imports decimal number as 64-bit floating-point type.
If the number to be imported is overflow or underflow, **csvs** imports it as `TEXT` data.

| Data type | Smallest finite         | Largest finite           |
|-----------|-------------------------|--------------------------|
| `INTEGER` | -2E+63                  | 2E+63−1                  |
| `REAL`    | 1.7976931348623157E+308 | -1.7976931348623157E+308 |



## Conclusion

By dynamically analyzing each column's content, **csvs** ensures accurate data type mapping while maintaining flexibility for nullable fields.

This behavior guarantees compatibility with SQLite and optimizes data handling for various use cases.
