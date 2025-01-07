use crate::args::Args;
use crate::format::STYLE_BAR;
use crate::sqlite_data_type::SqliteDataType;
use crate::util::sqlite_quoted::SqliteQuoted;
use anyhow::Context;
use std::collections::HashMap;
use std::io::{IsTerminal, Read};

use tracing::debug;

/// Parses CSV data and generates SQL statements
#[derive(Debug)]
pub struct CsvParser<'a> {
    /// Reference to application arguments
    args: &'a Args,
}
type CsvReaderType<T> = csv::Reader<encoding_rs_rw::DecodingReader<std::io::BufReader<T>>>;

impl<'a> CsvParser<'a> {
    /// Creates a new `CsvParser` instance
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    /// Parses CSV from standard input and generates SQL statements
    pub fn parse_stdin(
        &self,
        encoding: Option<String>,
        delimiter: u8,
    ) -> anyhow::Result<(String, Vec<sqlparser::ast::Statement>)> {
        debug!(
            "Parsing stdin with encoding: {:?}, delimiter: {}",
            encoding, delimiter
        );

        let mut stdin_content = Vec::new();
        {
            let mut stdin = std::io::stdin().lock();
            if stdin.is_terminal() {
                // If stdin is interactive, return an empty result
                return Ok(("".to_string(), Vec::new()));
            }
            stdin
                .read_to_end(&mut stdin_content)
                .context("Failed to read from stdin")?;
        }

        let encoding_detected = encoding.unwrap_or_else(|| {
            self.detect_encoding(&mut std::io::BufReader::new(stdin_content.as_slice()))
                .unwrap()
        });
        debug!("Detected encoding: {}", encoding_detected);

        let buf_reader = std::io::BufReader::new(stdin_content.as_slice());
        let csv_reader = self.get_csv_reader(buf_reader, encoding_detected, delimiter)?;

        self.parse(csv_reader, "stdin", None)
    }

    /// Parses CSV from a file and generates SQL statements
    pub fn parse_file(
        &self,
        file_path: &str,
        encoding: Option<String>,
        delimiter: u8,
    ) -> anyhow::Result<(String, Vec<sqlparser::ast::Statement>)> {
        debug!(
            "Parsing file: {} with encoding: {:?}, delimiter: {}",
            file_path, encoding, delimiter
        );

        let encoding_detected = encoding.unwrap_or_else(|| {
            self.detect_encoding(&mut std::io::BufReader::new(
                std::fs::File::open(file_path).unwrap(),
            ))
            .unwrap()
        });

        debug!(
            "Detected encoding: {} for file: {}",
            encoding_detected, file_path
        );

        // Create a buffered reader for the file content
        let file = std::fs::File::open(file_path)?;
        let buf_reader = std::io::BufReader::new(file);
        let csv_reader = self.get_csv_reader(buf_reader, encoding_detected, delimiter)?;

        let path = std::path::Path::new(file_path);
        let file_name = path
            .file_name()
            .context(format!("{} not found", file_path))?
            .to_str()
            .context("Cannot convert OsStr to Str")?;
        let buf_size = path.metadata()?.len();

        // Parse the CSV content into SQL statements
        self.parse(csv_reader, file_name, Some(buf_size))
    }

    /// Processes the CSV reader and generates SQL statements
    fn parse<T: Read>(
        &self,
        mut csv_reader: CsvReaderType<T>,
        buf_name: &str,
        buf_size: Option<u64>,
    ) -> anyhow::Result<(String, Vec<sqlparser::ast::Statement>)> {
        debug!("Parsing table: {}, size: {:?}", buf_name, buf_size);

        let pb = if let Some(buf_size) = buf_size {
            indicatif::ProgressBar::new(buf_size)
        } else {
            indicatif::ProgressBar::no_length()
        };
        pb.set_style(STYLE_BAR.clone());

        let mut headers = if self.args.in_no_header {
            vec![]
        } else {
            csv_reader
                .headers()?
                .iter()
                .map(|v| v.to_string())
                .collect()
        };

        let mut temp_types: HashMap<usize, SqliteDataType> = HashMap::new();
        let mut temp_nullable: HashMap<usize, bool> = HashMap::new();
        let mut temp_records: Vec<Vec<Option<String>>> = vec![];

        for result in csv_reader.records() {
            let record = result?;
            if self.args.in_no_header && headers.is_empty() {
                for header_index in 0..record.len() {
                    headers.push(format!("c{}", header_index + 1));
                }
            }

            let mut temp_record: Vec<Option<String>> = vec![];

            for header_index in 0..headers.len() {
                let data = record.get(header_index);
                if data.is_some_and(|v| !v.is_empty()) {
                    let data = data.context(format!("Cannot get a value at {}", header_index))?;
                    let detected = SqliteDataType::detect_type(data);

                    if temp_types
                        .get(&header_index)
                        .is_none_or(|v| v.can_convert(detected))
                    {
                        temp_types.insert(header_index, detected);
                    }

                    temp_record.push(Some(data.to_string()));
                } else {
                    temp_nullable.entry(header_index).or_insert(true);
                    temp_record.push(None);
                };
            }

            temp_records.push(temp_record);
            if let Some(position) = record.position() {
                pb.set_position(position.byte());
            }
        }

        pb.set_length(temp_records.len() as u64);
        pb.set_message("Generating SQL statements");
        pb.set_position(0);

        debug!("Finished parsing. Total rows: {}", temp_records.len());
        let mut statements: Vec<sqlparser::ast::Statement> = Vec::new();
        let dialect = sqlparser::dialect::SQLiteDialect {};

        let buf_name_quoted = SqliteQuoted::Field(buf_name.to_string()).get();
        let sql_drop_table = format!("DROP TABLE IF EXISTS {};", buf_name_quoted);

        match sqlparser::parser::Parser::parse_sql(&dialect, &sql_drop_table) {
            Ok(mut parsed) => {
                statements.append(&mut parsed);
            }
            Err(err) => {
                return Err(anyhow::anyhow!("{} {}", err, sql_drop_table));
            }
        }

        let mut sql_create: Vec<String> = vec![];
        sql_create.push(format!("CREATE TABLE {} (", buf_name_quoted));
        sql_create.push(format!(
            "{} INTEGER PRIMARY KEY AUTOINCREMENT,",
            SqliteQuoted::Field(self.args.raw_id.clone()).get()
        ));

        let mut sub_sql_create: Vec<String> = vec![];
        let mut sub_sql_insert: Vec<String> = vec![];

        for header_index in 0..headers.len() {
            let header = headers
                .get(header_index)
                .context(format!("Header at pos {} not found", header_index))?;

            let sqlite_type = temp_types
                .get(&header_index)
                .context(format!("Database type at pos {} not found", header_index))?;

            let not_null = if temp_nullable.contains_key(&header_index) {
                ""
            } else {
                "NOT NULL"
            };

            let header_quoted = SqliteQuoted::Field(header.to_string()).get();
            sub_sql_create.push(format!("{} {} {}", header_quoted, sqlite_type, not_null));
            sub_sql_insert.push(header_quoted);
        }

        sql_create.push(format!("{});", sub_sql_create.join(",")));
        let sql_create_final = sql_create.join("");

        match sqlparser::parser::Parser::parse_sql(&dialect, &sql_create_final) {
            Ok(mut parsed) => {
                statements.append(&mut parsed);
            }
            Err(err) => {
                return Err(anyhow::anyhow!("{} {}", err, sql_create_final));
            }
        }

        // Insert values into the database table
        let sql_insert = format!(
            "INSERT INTO {} ({})",
            buf_name_quoted,
            sub_sql_insert.join(", ")
        );

        let chunk_size = 50;
        let temp_records_iter = temp_records.chunks(chunk_size);
        for records in temp_records_iter {
            let mut sql_insert_values: Vec<String> = vec![];

            for record in records {
                let mut sql_insert_value: Vec<String> = vec![];

                for element_index in 0..record.len() {
                    let element = record
                        .get(element_index)
                        .context(format!("Element at pos {} not found", element_index))?;
                    let sqlite_type = temp_types
                        .get(&element_index)
                        .context(format!("Database type at pos {} not found", element_index))?;

                    match element {
                        None => {
                            sql_insert_value.push("NULL".to_string());
                        }
                        Some(element) => {
                            if *sqlite_type == SqliteDataType::Text {
                                let text_quoted = SqliteQuoted::Text(element.to_string()).get();
                                sql_insert_value.push(text_quoted);
                            } else {
                                sql_insert_value.push(element.to_string());
                            }
                        }
                    }
                }

                sql_insert_values.push(format!("({})", sql_insert_value.join(", ")));
            }

            let sql_final_insert =
                format!("{} VALUES {};", sql_insert, sql_insert_values.join(", "));

            match sqlparser::parser::Parser::parse_sql(&dialect, &sql_final_insert) {
                Ok(mut parsed) => {
                    statements.append(&mut parsed);
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("{} {}", err, sql_final_insert));
                }
            }

            pb.inc(chunk_size as u64);
        }

        pb.finish_and_clear();
        debug!("Finished parsing and loading data into table: {}", buf_name);
        Ok((buf_name.to_string(), statements))
    }

    /// Creates a CSV reader with specified options
    fn get_csv_reader<T: Read>(
        &self,
        buf_reader: std::io::BufReader<T>,
        encoding: String,
        delimiter: u8,
    ) -> anyhow::Result<CsvReaderType<T>> {
        let encoding_label = encoding_rs::Encoding::for_label(encoding.to_lowercase().as_bytes())
            .context(anyhow::anyhow!("Invalid encoding: {}", encoding))?;

        let decode_reader =
            encoding_rs_rw::DecodingReader::new(buf_reader, encoding_label.new_decoder());

        let reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(!self.args.in_no_header)
            .flexible(self.args.in_flexible)
            .trim(self.args.in_trim)
            .terminator(self.args.in_terminator)
            .quote(self.args.in_quote)
            .escape(self.args.in_escape)
            .double_quote(!self.args.in_no_double_quote)
            .quoting(!self.args.in_no_quoting)
            .comment(self.args.in_comment)
            .from_reader(decode_reader);

        Ok(reader)
    }

    /// Detects encoding from input
    fn detect_encoding<T: Read>(
        &self,
        buf_reader: &mut std::io::BufReader<T>,
    ) -> anyhow::Result<String> {
        let mut buf = [0; 1024];

        let _ = buf_reader.read(&mut buf)?;
        let mut detector = chardetng::EncodingDetector::new();

        detector.feed(&buf, false);
        let name = detector.guess(None, true).name().to_string();

        Ok(name)
    }
}
