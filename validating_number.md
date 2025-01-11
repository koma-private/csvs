# Validating Number

**csvs** determines whether a data entry is a number or text when importing CSV files into the embedded SQLite database.

## Valid Number Formats

**csvs** interprets the following strings as numbers:

- `0`, `-0`, `0.0`, `-0.0`, `.0`, `-.0`, `0.10`, `-0.10`, `.120`, `-.120`, `1`, `-1`, `1.`,
`-1.`, `1.0`, `-1.0`, `10`, `-10`, `10.`, `-10.`, `10.0`, `-10.0`

## Invalid Number Formats

**csvs** interprets the following strings as text:

- Invalid formats: `1a`, `2..1`, `..2`, `--0`
- Contains spaces: `10` (space at head), `102` (space at tail), `4 5` (space between numbers)

## Leading Zeros

If the `--in-allow-leading-zeros` option is specified, **csvs** interprets the following strings as numbers. Otherwise, they are treated as text:

- `001`, `-00`
