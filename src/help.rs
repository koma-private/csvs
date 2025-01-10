use crate::args_util::raw_args::RawArgs;
use clap::{crate_name, crate_version, CommandFactory};

/// Displays the help message.
/// Provides an overview of the application and its options.
fn show_help() {
    clap_help::Printer::new(RawArgs::command())
        .without("author") // Exclude author information
        .with("introduction", INTRODUCTION) // Add introduction section
        .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE) // Add command options
        .print_help();
}

/// Displays version information.
/// Prints the application name, version, and SQLite library version.
fn show_version() {
    println!(
        "{} {}\nSQLite version: {}",
        crate_name!(),       // Application name from Cargo metadata
        crate_version!(),    // Application version from Cargo metadata
        rusqlite::version()  // SQLite version used
    );
}

/// Introduction section for the help message.
/// Briefly explains the purpose of the application and provides usage examples.
static INTRODUCTION: &str = r#"
**csvs** allows SQL queries on CSV or TSV files using an embedded SQLite database.

### Usage:
- Without `--query` or `--source`, **csvs** runs in interactive mode.
- If `--out-file` is omitted, results are printed to `STDOUT` by default.

### Examples:
1. Query a file and save the result:
```
csvs -i ./address.csv \
     -q 'SELECT "city", "town", "phone" FROM "address.csv"' \
     > picked.csv
```

2. Join two files and display results:
```
csvs -i ./left.csv \
     -i ./right.tsv \
     -q 'SELECT * FROM "left.csv" AS l JOIN "right.tsv" AS r ON l."name" = r."name"'
```

3. Start in interactive mode:
```
csvs -i MOCK_DATA.csv
```
"#;

/// Safely displays the help message.
/// Prevents crashes using a panic handler.
pub fn safe_show_help() {
    if let Err(err) = std::panic::catch_unwind(show_help) {
        eprintln!("Failed to display help: {:?}", err);
    }
}

/// Safely displays the version information.
/// Prevents crashes using a panic handler.
pub fn safe_show_version() {
    if let Err(err) = std::panic::catch_unwind(show_version) {
        eprintln!("Failed to display version: {:?}", err);
    }
}
