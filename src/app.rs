use crate::args_util::args::Args;
use crate::args_util::is_tsv_filename::is_tsv_filename;
use crate::csv::csv_parser::CsvParser;
use crate::csv::csv_writer::CsvWriter;
use crate::db::execute_statements::execute_statements;
use crate::db::list_available_table::list_available_tables;
use crate::format::STYLE_BAR;
use crate::tui::tui_main::tui_main;

use crate::db::initialize_database::initialize_database;
use tracing::{debug, error};

/// Main application logic
pub fn app(args: Args) -> anyhow::Result<()> {
    debug!("Starting application with arguments: {:?}", args);

    // Initialize SQLite database connection
    let pool = initialize_database(&args.out_database)?;

    // Create CSV parser based on input arguments
    let parser = CsvParser::new(&args);

    // Process data from standard input if provided
    process_stdin(&parser, &pool, args.in_delimiter)?;

    // Process data from input files
    process_input_files(&parser, &pool, &args.in_file, args.in_delimiter)?;

    // Ensure there are tables in the database
    let available_table = list_available_tables(&pool)?;
    if available_table.is_empty() {
        return Err(anyhow::anyhow!("No valid CSV data inputted. Specify files with --in-file or pass content through STDIN."));
    }

    // Execute SQL statements or start interactive mode
    if !args.statements.is_empty() {
        command_line_mode(pool, args)?;
    } else if available_table.contains(&"stdin".to_string()) {
        return Err(anyhow::anyhow!(
            "Interactive mode cannot be invoked when inputting CSV content through STDIN. Use --in-file instead."
        ));
    } else {
        debug!("Launching interactive mode.");
        tui_main(pool, args)?;
    }
    Ok(())
}

/// Process CSV data from standard input
fn process_stdin(
    parser: &CsvParser,
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    delimiter: u8,
) -> anyhow::Result<()> {
    let conn = pool.get()?;
    
    debug!("Processing CSV data from stdin.");
    let (buf_name, statements) = parser.parse_stdin(None, delimiter)?;

    let statements_len = statements.len();
    let pb = indicatif::ProgressBar::new(statements_len as u64);
    pb.set_style(STYLE_BAR.clone());
    pb.set_message(format!("Importing data from {}", buf_name));
    for (index, statement) in statements.iter().enumerate().take(statements_len) {
        if index % 10 == 0 {
            pb.set_position(index as u64);
        }
        conn.execute(statement, ())?;
    }
    Ok(())
}

/// Process CSV data from specified input files
fn process_input_files(
    parser: &CsvParser,
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    input_files: &[String],
    default_delimiter: u8,
) -> anyhow::Result<()> {
    let conn = pool.get()?;

    for input_file in input_files {
        let delimiter = if is_tsv_filename(input_file) {
            b'\t'
        } else {
            default_delimiter
        };

        debug!(
            "Processing file: {} with delimiter: {}",
            input_file, delimiter
        );
        match parser.parse_file(input_file, None, delimiter) {
            Ok((buf_name, statements)) => {
                let statements_len = statements.len();
                let pb = indicatif::ProgressBar::new(statements_len as u64);
                pb.set_style(STYLE_BAR.clone());
                pb.set_message(format!("Importing data from file: {}", buf_name));
                for (index, statement) in statements.iter().enumerate().take(statements_len) {
                    if index % 10 == 0 {
                        pb.set_position(index as u64);
                    }
                    conn.execute(statement, ())?;
                }
            }
            Err(err) => {
                error!("Error processing file {}: {}", input_file, err);
                return Err(err);
            }
        }
    }
    Ok(())
}

/// Command-line mode for executing SQL statements
fn command_line_mode(
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    args: Args,
) -> anyhow::Result<()> {
    let query_result =
        execute_statements(&pool, args.statements.clone(), Some(args.raw_id.clone()))?;

    if let Some(last) = query_result.last() {
        if args.out_file.is_some() {
            println!("Saving query results to {}", args.out_file.clone().unwrap());
        }
        let mut writer = CsvWriter::new(&args)?;
        if !args.out_without_header {
            writer.write_record(&last.header)?;
        }
        writer.write_records(&last.rows)?;
        writer.flush()?;
    } else {
        return Err(anyhow::anyhow!("No results from the SQL query."));
    }
    Ok(())
}
