# Command Options

- **csvs** auto-detects file encodings.
- If `--out-file` isn't specified, results are printed to `STDOUT`.

| Short Option | Long Option           | Value               | Description                                                                                                                                             |
|--------------|-----------------------|---------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| -i           | --in-file             | `<IN_FILE>`         | Input files to process. `.tsv` files use tab as the delimiter.                                                                                          |
| -q           | --query               | `<QUERY>`           | SQL query string to execute.                                                                                                                            |
| -s           | --source              | `<SOURCE>`          | Path to a file containing the SQL query.                                                                                                                |
| -h           | --help                |                     | how help message and exit.                                                                                                                              |
|              | --in-comment          | `<IN_COMMENT>`      | Comment character for parsing CSV (single byte). If the start of a record begins with the byte given here, then that line is ignored by the CSV parser. |
|              | --in-delimiter        | `<IN_DELIMITER>`    | Delimiter for input CSV (single byte, e.g., `,` or `\t`)<br>Default: `,`.                                                                               |
|              | --in-escape           | `<IN_ESCAPE>`       | Escape character for parsing CSV (single byte).                                                                                                         |
|              | --in-flexible         |                     | Allow CSV rows with varying field counts.                                                                                                               |
|              | --in-no-double-quote  |                     | Disable escaping of double quotes in CSV fields.                                                                                                        |
|              | --in-no-header        |                     | Treat input CSV as header-less.                                                                                                                         |
|              | --in-no-quoting       |                     | Disable special handling of quotes in CSV parsing.                                                                                                      |
|              | --in-quote            | `<IN_QUOTE>`        | Quote character for parsing CSV (single byte).<br>Default: `"`.                                                                                         |
|              | --in-terminator       | `<IN_TERMINTOR>`    | Record terminator for parsing CSV (single byte, e.g, `\n`). Use `CRLF` for `\r\n`.<br>Default: `CRLF`.                                                  |
|              | --in-trim             | `<IN_TRIM>`         | Trimming behavior for input data. Possible values: [`all`, `fields`, `headers`, `none`]<br>Default: `none`.                                             |
|              | --out-comment         | `<OUT_COMMENT>`     | Comment character for output CSV (single byte).                                                                                                         |
|              | --out-database        | `<OUT_DATABASE>`    | Output SQLite database file.                                                                                                                            |
|              | --out-delimiter       | `<OUT_DELIMITER>`   | Delimiter for output CSV (single byte, e.g., `,` or `\t`)<br>Default: `,`.                                                                              |
|              | --out-encoding        | `<OUT_ENCODING>`    | Character encoding for output files (default: `utf-8`). Refer to the [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/#names-and-labels).    |
|              | --out-escape          | `<OUT_ESCAPE>`      | Escape character for output CSV (single byte) Only used when `--out-no-double-quote` is specified.                                                      |
| -o           | --out-file            | `<OUT_FILE>`        | Output file path. `.tsv` files use tab as the delimiter.                                                                                                |
|              | --out-log             | `<OUT_LOG>`         | Path to the log file.                                                                                                                                   |
|              | --out-no-double-quote |                     | Disable escaping of double quotes in output CSV.                                                                                                        |
|              | --out-quote-style     | `<OUT_QUOTE_STYLE>` | Quoting style for output CSV. Possible values: [`always`, `necessary`, `non-numeric`, `never`]<br>Default: `necessary`.                                 |
|              | --out-quote           | `<OUT_QUOTE>`       | Quote character for output CSV (single byte).<br>Default: `"`.                                                                                          |
|              | --out-terminator      | `<OUT_TERMINATOR>`  | Record terminator for output CSV (single byte, e.g, `\n`). Use `CRLF` for `\r\n`.<br>Default: `CRLF`.                                                   |
|              | --out-without-header  |                     | Exclude header row from output CSV.                                                                                                                     |
|              | --raw-id              | `<RAW_ID>`          | Primary key column name for database tables.<br>Default: `_raw_id`.                                                                                     |
| -V           | --version             |                     | Show version information and exit.                                                                                                                      |

### `--in-trim` Option

The whitespace preservation behaviour when reading CSV data.

| `<IN_TRIM>` | Description                                   |
|-------------|-----------------------------------------------|
| `all`       | Trim whitespace from fields and headers.      |
| `fields`    | Trim whitespace from fields, but not headers. |
| `headers`   | Trim whitespace from headers.                 |
| `none`      | Preserves fields and headers.                 |

### `--out-delimiter` Option

This option controls the delimiter used in the CSV output. The same logic applies to the `--in-delimiter` option for
input.

- If `<OUT_FILE>` ends with `.tsv`, **csvs** forces Tab as the delimiter. You cannot override it in this case.
- If `<OUT_FILE>` ends with `.csv`, **csvs** uses comma by default, but `--out-delimiter` can override this.

| Delimiter        | How to Specify the Option |
|------------------|---------------------------|
| Double quotation | `--out-delimiter '\"'`    |
| Single quotation | `--out-delimiter "\'"`    |
| Backslash        | `--out-delimiter '\\'`    |
| Tab              | `--out-delimiter '\t'`    |
| Comma            | `--out-delimiter ','`     |

### `--out-quote` option

Specifies the quoting style used when writing CSV output. Refer to the
[QuoteStyle documentation in the csv crate](https://docs.rs/csv/latest/csv/enum.QuoteStyle.html) for more details.

| `<OUT_QUOTE>` | Description                                                                   |
|---------------|-------------------------------------------------------------------------------|
| `always`      | Always encloses every field in quotes, regardless of content.                 |
| `necessary`   | Only encloses fields when necessary (e.g., if they contain the delimiter).    |
| `non-numeric` | Quotes any field that is not purely numeric.                                  |
| `never`       | Never writes quotes, even if it produces invalid CSV data (use with caution). |
