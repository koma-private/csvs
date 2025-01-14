# csvs

**csvs** (**CSV** **S**ql) is a command-line tool that simplifies working with CSV or TSV files by enabling SQL queries
through an embedded [SQLite](https://www.sqlite.org/) engine. It is ideal for data analysts and developers who need
SQL's flexibility to manage text-based data efficiently.

[![GitHub Release](https://img.shields.io/github/actions/workflow/status/koma-private/csvs/release.yml)](https://github.com/koma-private/csvs)
[![GitHub Tag](https://img.shields.io/github/v/tag/koma-private/csvs)](https://github.com/koma-private/csvs)
[![Crates.io Version](https://img.shields.io/crates/v/csvs)](https://crates.io/crates/csvs)

![Banner of executing csvs](assets/usage.banner.png)
![Banner of interactive mode](assets/interactive.banner.png)

- [Features](#features)
    - [SQL Power for CSV Files](#sql-power-for-csv-files)
    - [Automatic Encoding Detection](#automatic-encoding-detection)
    - [Decide Data Type for Each Column](#decide-data-type-for-each-column)
    - [Multi-File Handling](#multi-file-handling)
    - [Customizable Output](#customizable-output)
    - [Interactive Mode](#interactive-mode)
    - [Multi-Statement Query Support](#multi-statement-query-support)
- [Usage](#usage)
    - [Interactive Mode](#interactive-mode-1)
    - [Command Options](#command-options)
- [Example](#example)
- [SQL Query Notes](#sql-query-notes)
    - [Mapping CSVs to Table Names](#mapping-csvs-to-table-names)
    - [Quoting Columns with Special Characters](#quoting-columns-with-special-characters)
    - [`--in-no-header` Option](#--in-no-header-option)
    - [Execute Multiple Statements in a Single Query](#execute-multiple-statements-in-a-single-query)
- [Error Handling](#error-handling)
- [Build](#building-csvs)
- [Limitations](#limitations)
- [Acknowledgments](#acknowledgments)
- [License](#license)

---

## Features

### SQL Power for CSV Files

Run advanced SQL queries, including `JOIN`, `GROUP BY`, `SUM()`, or `COUNT()` on CSV data.
Gain unparalleled flexibility to query, filter, sort, group, and combine data compared to traditional spreadsheet tools.
**csvs** also supports regular expressions in SQL queries. Refer to [Regular Expressions Document](regexp.md).

### Automatic Encoding Detection

Eliminate encoding issues with automatic detection of character encodings. Avoid garbled text and broken queries
effortlessly.

### Decide Data Type for Each Column

**csvs** scans CSV rows to determine the most appropriate data types for SQLite tables. This dynamic analysis ensures
compatibility and precision, even when handling nullable fields.

See [Decide Data Type for Each Column](decide_data_type.md) and [Validating Number Document](validating_number.md) for
details.

### Multi-File Handling

Combine data from multiple CSV or TSV files by creating a temporary SQLite database using `--in-file`. Easily perform
SQL joins across files in seconds.

Common use cases:

- Merging datasets from separate files.
- Cross-referencing data using SQL joins.

### Customizable Output

Export query results as:

- **CSV or TSV**: Ideal for data sharing or further processing.
- **SQLite Database**: Retain results as `.db` files for future queries.

Control delimiters, headers, and quoting styles to suit your needs.

### Interactive Mode

Explore datasets interactively without specifying queries upfront. Features include:

- Browsing imported tables.
- Ad-hoc query execution.
- Previewing and saving query results.

### Multi-Statement Query Support

Execute multiple SQL statements in a single command. Transform and query data across multiple steps, with only the final
result displayed.

---

## Usage

- Display help:

```shell
csvs --help
```

### Interactive Mode

Start **csvs** in *interactive mode* when neither `--query` nor `--source` is specified. This mode allows you to:

- View imported tables.
- Preview table content.
- Save query results to files interactively.

See [Interactive Mode Guide](interactive_mode.md)

![Animation of interactive mode](assets/interactive.gif)

### Command Options

See [Command Options Guide](command_options.md)

![Animation of executing csvs](assets/usage.gif)

---

## Example

- Display version:

```shell
csvs --version
```

- Select specific fields from `./data/address.csv` and save the results to `picked.csv`:

```shell
csvs -i ./data/address.csv -q 'SELECT "city","town","phone" FROM "address.csv"' -o picked.csv
```

- Process CSV data from `STDIN`:

```shell
csvs -q 'SELECT "city","town","phone" FROM "stdin"' < ./data/address.csv > picked.csv
```

or

```shell
cat ./data/address.csv | csvs -q 'SELECT "city","town","phone" FROM "stdin"' > picked.csv
```

- Perform SQL joins across multiple files:

```shell
csvs -i ./left.csv -i ./right.tsv -q 'SELECT * FROM "left.csv" AS l JOIN "right.tsv" AS r ON l."name"=r."name"'
```

- Leverage SQLite functions like `UPPER()`, `COUNT()`, etc., and export results to a SQLite database:

```shell
csvs -i people.csv -q 'SELECT "city",COUNT(*) FROM "people.csv" GROUP BY "city" ORDER BY COUNT(*) DESC' --out-database out.db
```

- Start in *interactive mode*:

```shell
csvs -i MOCK_DATA.csv
```

---

## SQL Query Notes

### Mapping CSVs to Table Names

File names provided with `--in-file` map directly to SQLite tables (e.g. `./sample/address.csv` becomes `"address.csv"`,
and `data.2024.csv` becomes `"data.2024.csv"`).

### Quoting Columns with Special Characters

Columns or table names with spaces, punctuation, or reserved words must be quoted. Example:

```sql
SELECT "first name", "last name"
FROM "contacts.csv"
```

### `--in-no-header` Option

If specified, column names default to "c1", "c2", "c3", etc., for header-less CSV files.

### Execute Multiple Statements in a Single Query

Separate SQL statements with semicolons to execute multiple queries in sequence. Only the result of the final query is
displayed.

Example:

```sql
SELECT "first name"
FROM "contacts.csv";
SELECT "age"
FROM "contacts.csv"; 
```

---

## Error Handling

See [Error Handling Guide](error_handling.md)

---

## Building csvs

See [Build Guide](build.md)

---

## Limitations

- *Interactive mode* cannot be invoked when CSV data is provided via `STDIN`. Use `--in-file` to specify CSV files
  instead.
- Large files may require significant RAM since **csvs** loads entire files into memory when `--out-database` is not
  specified.
- CSV files with names starting with `sqlite_` cannot be used with `--in-file` due to SQLite's reserved naming
  convention.

## Acknowledgments

**csvs** relies on the following open-source projects:

- [SQLite](https://www.sqlite.org/)
- [anyhow](https://github.com/dtolnay/anyhow)
- [chardetng](https://github.com/hsivonen/chardetng)
- [clap](https://github.com/clap-rs/clap)
- [clap-help](https://github.com/Canop/clap-help)
- [csv](https://github.com/BurntSushi/rust-csv)
- [encoding_rs](https://github.com/hsivonen/encoding_rs)
- [encoding_rs_rw](https://github.com/LiosK/encoding_rs_rw)
- [indicatif](https://github.com/console-rs/indicatif)
- [lazy-regex](https://github.com/Canop/lazy-regex)
- [ratatui](https://github.com/ratatui/ratatui)
- [r2d2](https://github.com/sfackler/r2d2)
- [r2d2_sqlite](https://github.com/ivanceras/r2d2-sqlite)
- [rusqlite](https://github.com/rusqlite/rusqlite)
- [smashquote](https://github.com/ionizedgirl/smashquote)
- [sqlparser](https://github.com/apache/datafusion-sqlparser-rs)
- [tracing](https://github.com/tokio-rs/tracing)
- [tracing-logfmt](https://github.com/EmbarkStudios/tracing-logfmt)
- [tuirealm](https://github.com/veeso/tui-realm)
- And many more! See the full list in the source repository.

## License

**csvs** is licensed under the [MIT license](LICENSE).
