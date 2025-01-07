use crate::args::Args;
use crate::util::is_tsv_filename::is_tsv_filename;
use anyhow::Context;
use tracing::debug;

/// Handles writing CSV data to stdout or a file with configurable settings.
pub struct CsvWriter {
    writer_stdout: Option<
        csv::Writer<
            encoding_rs_rw::EncodingWriter<encoding_rs_rw::misc::DefaultBuffer<std::io::Stdout>>,
        >,
    >,
    writer_file: Option<
        csv::Writer<
            encoding_rs_rw::EncodingWriter<encoding_rs_rw::misc::DefaultBuffer<std::fs::File>>,
        >,
    >,
}

impl CsvWriter {
    /// Creates a new CsvWriter based on the provided arguments.
    pub fn new(args: &Args) -> anyhow::Result<Self> {
        debug!("Initializing CsvWriter with arguments: {:?}", args);

        // Resolve encoding
        let encoding_label =
            encoding_rs::Encoding::for_label(args.out_encoding.to_lowercase().as_bytes())
                .context(anyhow::anyhow!("Invalid encoding: {}", args.out_encoding))?;
        let encoder = encoding_label.new_encoder();

        let mut writer = csv::WriterBuilder::new();
        let mut writer = writer
            .delimiter(args.out_delimiter)
            .terminator(args.out_terminator)
            .quote_style(args.out_quote_style)
            .quote(args.out_quote)
            .double_quote(!args.out_no_double_quote)
            .comment(args.out_comment);

        if let Some(out_escape) = args.out_escape {
            writer = writer.escape(out_escape);
        }

        let writer_stdout;
        let writer_file;

        // Determine output target: stdout or file
        match &args.out_file {
            None => {
                debug!("Output to stdout.");
                let encoding_writer =
                    encoding_rs_rw::EncodingWriter::new(std::io::stdout(), encoder);

                writer_stdout = Some(writer.from_writer(encoding_writer));
                writer_file = None;
            }
            Some(output_file) => {
                debug!("Output to file: {}", output_file);
                if is_tsv_filename(output_file) {
                    debug!("Detected TSV file; setting delimiter to '\\t'.");
                    writer.delimiter(b'\t');
                }
                let file = std::fs::File::create(output_file)?;
                let encoding_writer = encoding_rs_rw::EncodingWriter::new(file, encoder);

                writer_stdout = None;
                writer_file = Some(writer.from_writer(encoding_writer));
            }
        };

        Ok(Self {
            writer_stdout,
            writer_file,
        })
    }

    /// Writes a single record to the output.
    pub fn write_record<I, T>(&mut self, record: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]>,
    {
        if let Some(writer) = &mut self.writer_stdout {
            writer
                .write_record(record)
                .context("Failed to write record to stdout")?;
        } else if let Some(writer) = &mut self.writer_file {
            writer
                .write_record(record)
                .context("Failed to write record to file")?;
        } else {
            anyhow::bail!("No available writer for output");
        }
        Ok(())
    }

    /// Writes multiple records in batch mode.
    pub fn write_records<I, T>(&mut self, records: &Vec<I>) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T> + Clone,
        T: AsRef<[u8]>,
    {
        if let Some(writer) = &mut self.writer_stdout {
            for record in records {
                writer
                    .write_record(record.clone())
                    .context("Failed to write record to stdout")?;
            }
        } else if let Some(writer) = &mut self.writer_file {
            for record in records {
                writer
                    .write_record(record.clone())
                    .context("Failed to write record to file")?;
            }
        } else {
            anyhow::bail!("No available writer for output");
        }
        Ok(())
    }

    /// Flushes the writer to ensure all data is written.
    pub fn flush(&mut self) -> anyhow::Result<()> {
        if let Some(writer) = &mut self.writer_stdout {
            writer.flush().context("Failed to flush stdout writer")?;
        } else if let Some(writer) = &mut self.writer_file {
            writer.flush().context("Failed to flush file writer")?;
        } else {
            anyhow::bail!("No writer available to flush");
        }

        Ok(())
    }
}
