# Error Handling

### Return Codes
- On successful execution, **csvs** exits with code `0`.
- Non-zero exit codes indicate critical errors encountered during execution.

### Error Conditions and Solutions
| Error Message or Situation                                                                | Cause                                                                                               | Solution                                                                                    |
|-------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|
| `Error: near **: syntax error in CREATE TABLE` or `Error: duplicate column name: **`      | The CSV data does not have a header row.                                                            | Use `--in-no-header` to treat the CSV file as header-less.                                  |
| `Error: Database type at pos ** not found`                                                | The file specified by `--in-file` contains an empty column name in the header row.                  | Rename or remove any empty column names in the header row.                                  |
| `Error: duplicate column name: _raw_id`                                                   | The `--in-file` contains a column named `_raw_id`, conflicting with **csvs**'s default primary key. | Use `--raw-id` to specify a different primary key column name.                              |
| `Error: No valid CSV data inputted`                                                       | No CSV data is provided (no `--in-file` option used and no data passed through `STDIN`).            | Provide CSV data via `--in-file` or `STDIN`                                                 |
| `Error: CSV error: ** found record with ** fields, but the previous record has ** fields` | Inconsistent field counts the CSV data due to formatting issues.                                    | Correct the CSV file to ensure consistent field counts or use `--in-flexible` to bypass it. |
| `Error: sql parser error: **`                                                             | Invalid SQL query syntax not compatible with SQLite’s dialect.                                      | Correct the SQL syntax and ensure compatibility with SQLite.                                |
| Garbled characters are displayed                                                          | The `--in-file` has an undetectable or unsupported character encoding.                              | Verify and correct the file's encoding to a supported format like UTF-8.                    |

### General Notes
- **csvs** provides detailed error messages whenever possible to help diagnose issues.
- For advanced debugging, check the output logs if `--out-log` is specified.
