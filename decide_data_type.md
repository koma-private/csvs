# Decide Data Type for Each Column

**csvs** scans each row of the input CSV to determine the appropriate data type for columns in the SQLite database.

Refer to [Validating Number Document](validating_number.md) for details of how **csvs** interprets input data.

## Process Overview

1. **Detect Data Type**: Each cell in the current row is analyzed to determine its data type.
2. **Initial Assignment**: For the first row, the detected data type is saved.
3. **Comparison and Adjustment**: For subsequent rows, the detected data type is compared to the previously saved type,
   and adjustments are made as needed:
    - If the prior type is `INTEGER` and a `REAL` value is detected, the column type is updated to `REAL`.
    - If the prior type is `REAL` and an `INTEGER` value is detected, the column type remains `REAL`.
    - If a text value is encountered, the column type is set to `TEXT`.

### Data Type Conversion Table

The table below outlines type conversions during the import process:

| From / To | `INTEGER` | `REAL` | `TEXT` |
|:---------:|:---------:|:------:|:------:|
| `INTEGER` |    YES    |  YES   |  YES   |
|  `REAL`   |    NO     |  YES   |  YES   |
|  `TEXT`   |    NO     |   NO   |  YES   |

## Example: Input CSV and Final SQLite Types

### Input CSV

|   | Keep Integer | Keep Real | Keep Text | Integer to Real | Integer to Text | Real to Text |
|---|:------------:|:---------:|:---------:|:---------------:|:---------------:|:------------:|
| 1 |      3       |    1.0    |     a     |        2        |       56        |     3.4      |
| 2 |      -0      |   -1.1    |     2     |        1        |                 |      A       |
| 3 |      2       |    99     |   -0.9    |       0.3       |        C        |     -2.3     |

### Final Data Types

| Column Name     | SQLite Type | Allows `NULL` |
|-----------------|:-----------:|:-------------:|
| Keep Integer    |  `INTEGER`  |  `NOT NULL`   |
| Keep Real       |   `REAL`    |  `NOT NULL`   |
| Keep Text       |   `TEXT`    |  `NOT NULL`   |
| Integer to Real |   `REAL`    |  `NOT NULL`   |
| Integer to Text |   `TEXT`    |               |
| Real to Text    |   `TEXT`    |  `NOT NULL`   |

## Special Cases

### Handling Empty Cells

Columns with empty cells are treated as nullable, allowing `NULL` values in the database.

### Number Ranges

- `INTEGER`: Imported as a 64-bit signed type. Valid range is `-2^63` to `2^63-1`.
- `REAL`: Imported as a 64-bit floating-point type. Valid range is approximately ± `1.7976931348623157E+308`.
- Overflow or underflow values are imported as `TEXT`.

## Valid and Invalid Numbers

### Valid Number Formats

- Examples: `0`, `-0`, `0.0`, `.120`, `10`, `-10.0`

### Invalid Number Formats

- Examples: `1a`, `2..1`, `10` (leading space), `4 5` (spaces between digits)

### Leading Zeros

- Use the `--in-allow-leading-zeros` option to interpret values like `001` or `-00` as valid numbers.

By dynamically analyzing column content, csvs ensures accurate data type assignments and optimal compatibility with
SQLite databases.

