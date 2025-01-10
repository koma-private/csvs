use crate::args_util::quote_style::MyQuoteStyle;
use crate::args_util::trim::MyTrim;

/// Represents raw command-line arguments parsed by `clap`.
#[derive(clap::Parser, Debug)]
#[command(version, about, disable_help_flag = true, disable_version_flag = true)]
pub struct RawArgs {
    /// Input files to process. `.tsv` files use tab as the delimiter.
    #[clap(short, long)]
    pub in_file: Vec<String>,

    /// SQL query and source file arguments.
    #[clap(flatten)]
    pub query_group: RawArgQueryGroup,

    /// Show help message and exit.
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub help: u8,

    /// Comment character for parsing CSV (single byte).
    #[clap(long)]
    pub in_comment: Option<String>,

    /// Delimiter for input CSV (single byte, e.g., `,` or `\t`).
    #[clap(long, default_value = ",")]
    pub in_delimiter: String,

    /// Escape character for parsing CSV (single byte).
    #[clap(long)]
    pub in_escape: Option<String>,

    /// Allow CSV rows with varying field counts.
    #[clap(long, action = clap::ArgAction::Count)]
    pub in_flexible: u8,

    /// Disable escaping of double quotes in CSV fields.
    #[clap(long, action = clap::ArgAction::Count)]
    pub in_no_double_quote: u8,

    /// Treat input CSV as header-less.
    #[clap(long, action = clap::ArgAction::Count)]
    pub in_no_header: u8,

    /// Disable special handling of quotes in CSV parsing.
    #[clap(long, action = clap::ArgAction::Count)]
    pub in_no_quoting: u8,

    /// Quote character for parsing CSV (single byte).
    #[clap(long, default_value = "\"")]
    pub in_quote: String,

    /// Record terminator for parsing CSV (single byte, e.g, `\n`). Use `CRLF` for `\r\n`.
    #[clap(long, default_value = "CRLF")]
    pub in_terminator: String,

    /// Trimming behavior for input data.
    #[clap(long, default_value = "none")]
    #[arg(value_enum)]
    pub in_trim: MyTrim,

    /// Comment character for output CSV (single byte).
    #[clap(long)]
    pub out_comment: Option<String>,

    /// Output SQLite database file.
    #[clap(long)]
    pub out_database: Option<String>,

    /// Delimiter for output CSV (single byte, e.g., `,` or `\t`).
    #[clap(long, default_value = ",")]
    pub out_delimiter: String,

    /// Character encoding for output files (e.g., `utf-8`).
    #[clap(long, default_value = "utf-8")]
    pub out_encoding: String,

    /// Escape character for output CSV (single byte).
    #[clap(long)]
    pub out_escape: Option<String>,

    /// Output file path. `.tsv` files use tab as the delimiter.
    #[clap(short, long)]
    pub out_file: Option<String>,

    /// Path to the log file.
    #[clap(long)]
    pub out_log: Option<String>,

    /// Disable escaping of double quotes in output CSV.
    #[clap(long, action = clap::ArgAction::Count)]
    pub out_no_double_quote: u8,

    /// Quoting style for output CSV.
    #[clap(long, default_value = "necessary")]
    #[arg(value_enum)]
    pub out_quote_style: MyQuoteStyle,

    /// Quote character for output CSV (single byte).
    #[clap(long, default_value = "\"")]
    pub out_quote: String,

    /// Record terminator for output CSV (single byte, e.g, `\n`). Use `CRLF` for `\r\n`.
    #[clap(long, default_value = "CRLF")]
    pub out_terminator: String,

    /// Exclude header row from output CSV.
    #[clap(long, action = clap::ArgAction::Count)]
    pub out_without_header: u8,

    /// Primary key column name for database tables.
    #[clap(long, default_value = "_raw_id")]
    pub raw_id: String,

    /// Show version information and exit.
    #[clap(short = 'V', long, action = clap::ArgAction::Count)]
    pub version: u8,
}

/// Grouping for SQL query arguments.
#[derive(Debug, clap::Args)]
#[group(required = false, multiple = false)]
pub struct RawArgQueryGroup {
    /// SQL query string to execute.
    #[clap(short, long)]
    pub query: Option<String>,

    /// Path to a file containing the SQL query.
    #[clap(short, long)]
    pub source: Option<String>,
}
