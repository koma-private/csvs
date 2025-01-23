use crate::args_util::raw_args::RawArgs;
use anyhow::Context;
use std::io::Read;
use tracing::debug;

/// Represents parsed command-line arguments.
#[derive(Debug, Clone)]
pub struct Args {
    pub in_file: Vec<String>,                       // Input file paths
    pub statements: Vec<sqlparser::ast::Statement>, // SQL queries
    pub help: bool,                                 // Display help flag
    pub in_allow_leading_zeros: bool,
    pub in_comment: Option<u8>,
    pub in_delimiter: u8, // CSV delimiter
    pub in_escape: Option<u8>,
    pub in_flexible: bool, // Allow varying fields
    pub in_no_double_quote: bool,
    pub in_no_header: bool, // No header row
    pub in_no_quoting: bool,
    pub in_quote: u8,                   // Quote character
    pub in_terminator: csv::Terminator, // Record terminator
    pub in_trim: csv::Trim,             // Trimming behavior
    pub out_comment: Option<u8>,
    pub out_database: Option<String>, // Output SQLite DB
    pub out_delimiter: u8,            // Output CSV delimiter
    pub out_encoding: String,         // Output encoding
    pub out_escape: Option<u8>,
    pub out_file: Option<String>, // Output file path
    pub out_log: Option<String>,  // Output log path
    pub out_no_double_quote: bool,
    pub out_quote_style: csv::QuoteStyle, // Output quote style
    pub out_quote: u8,
    pub out_terminator: csv::Terminator, // Output terminator
    pub out_without_header: bool,        // Exclude headers
    pub raw_id: String,                  // Primary key column
    pub version: bool,                   // Display version flag
}

impl TryFrom<RawArgs> for Args {
    type Error = anyhow::Error;

    /// Converts `RawArgs` to `Args` with validation.
    fn try_from(value: RawArgs) -> anyhow::Result<Self> {
        // Parse SQL queries
        let statements = query_to_statements(value.query_group.query, value.query_group.source)?;

        // Validate input files
        validate_in_files(&value.in_file)?;

        // Convert single-byte arguments
        let in_comment = parse_optional_byte(&value.in_comment)?;
        let in_delimiter = parse_required_byte(&value.in_delimiter)?;
        let out_delimiter = parse_required_byte(&value.out_delimiter)?;
        let in_escape = parse_optional_byte(&value.in_escape)?;
        let in_quote = parse_required_byte(&value.in_quote)?;
        let out_comment = parse_optional_byte(&value.out_comment)?;
        let out_escape = parse_optional_byte(&value.out_escape)?;
        let out_quote = parse_required_byte(&value.out_quote)?;

        // Parse other arguments
        let in_trim = value.in_trim.into();
        let out_quote_style = value.out_quote_style.into();
        let in_terminator = parse_terminator(&value.in_terminator)?;
        let out_terminator = parse_terminator(&value.out_terminator)?;
        validate_encoding(&value.out_encoding)?;

        Ok(Self {
            in_file: value.in_file,
            statements,
            help: value.help > 0,
            in_allow_leading_zeros: value.in_allow_leading_zeros > 0,
            in_comment,
            in_delimiter,
            in_escape,
            in_flexible: value.in_flexible > 0,
            in_no_double_quote: value.in_no_double_quote > 0,
            in_no_header: value.in_no_header > 0,
            in_no_quoting: value.in_no_quoting > 0,
            in_quote,
            in_terminator,
            in_trim,
            out_comment,
            out_database: value.out_database,
            out_delimiter,
            out_encoding: value.out_encoding,
            out_escape,
            out_file: value.out_file,
            out_log: value.out_log,
            out_no_double_quote: value.out_no_double_quote > 0,
            out_quote_style,
            out_quote,
            out_terminator,
            out_without_header: value.out_without_header > 0,
            raw_id: value.raw_id,
            version: value.version > 0,
        })
    }
}

/// Parses SQL queries from string or file.
fn query_to_statements(
    query: Option<String>,
    source: Option<String>,
) -> anyhow::Result<Vec<sqlparser::ast::Statement>> {
    let query_final = if let Some(query) = query {
        debug!("SQL query provided: {}", query);
        Some(query)
    } else if let Some(source) = source {
        debug!("Reading SQL query from file: {}", source);
        let mut temp_str = String::new();
        std::fs::File::open(source)?.read_to_string(&mut temp_str)?;
        Some(temp_str)
    } else {
        None
    };

    if let Some(query_str) = query_final {
        debug!("Parsing SQL queries.");
        let dialect = sqlparser::dialect::SQLiteDialect {};
        let statements = sqlparser::parser::Parser::parse_sql(&dialect, &query_str)?;
        if statements.is_empty() {
            return Err(anyhow::anyhow!("No valid SQL query provided"));
        }
        Ok(statements)
    } else {
        Ok(vec![])
    }
}

/// Validates existence of input files.
fn validate_in_files(files: &[String]) -> anyhow::Result<()> {
    for file in files {
        std::fs::metadata(file).with_context(|| format!("File not found: {}", file))?;
    }
    Ok(())
}

/// Parses a string into a CSV record terminator.
fn parse_terminator(source: &str) -> anyhow::Result<csv::Terminator> {
    debug!("Parsing terminator: {}", source);
    if source.eq_ignore_ascii_case("crlf") {
        Ok(csv::Terminator::CRLF)
    } else {
        let bytes = smashquote::unescape_bytes(source.as_bytes())?;
        bytes
            .first()
            .map(|&b| csv::Terminator::Any(b))
            .context("Invalid terminator")
    }
}

/// Parses an optional single-byte argument.
fn parse_optional_byte(arg: &Option<String>) -> anyhow::Result<Option<u8>> {
    match arg {
        None => Ok(None),
        Some(value) => {
            let parsed = parse_required_byte(value)?;
            Ok(Some(parsed))
        }
    }
}

/// Parses a required single-byte argument.
fn parse_required_byte(arg: &str) -> anyhow::Result<u8> {
    let bytes = smashquote::unescape_bytes(arg.as_bytes())?;
    let byte = bytes.first().context("Cannot parse single byte")?;
    Ok(*byte)
}

/// Validates the encoding label.
fn validate_encoding(encoding: &str) -> anyhow::Result<()> {
    if encoding.is_empty() {
        Err(anyhow::anyhow!("Encoding cannot be empty"))
    } else {
        Ok(())
    }
}
