/// Enum for SQL-quoted values.
/// Handles SQL-specific quoting for fields (e.g., column names) and text (e.g., string literals).
pub enum SqliteQuoted {
    /// SQL-quoted field (e.g., column/table name).
    Field(String),
    /// SQL-quoted text value (e.g., string literal).
    Text(String),
}

impl SqliteQuoted {
    /// Returns the SQL-quoted version of the value.
    ///
    /// # Returns
    /// A `String` with the SQL-quoted value:
    /// - `Field`: Uses double quotes (") or backticks (`) based on content.
    /// - `Text`: Escapes single quotes (') and wraps the value with single quotes.
    pub fn get(&self) -> String {
        match self {
            Self::Field(value) => {
                if value.contains("\"") {
                    format!("`{}`", value.replace("`", "``"))
                } else {
                    format!("\"{}\"", value)
                }
            }
            Self::Text(value) => {
                format!("'{}'", value.replace("'", "''"))
            }
        }
    }
}

#[test]
fn test_escape_sql_quotes() {
    let patterns = vec!["d\"", "e'", "g\"\"", "h''", "i\"'"];

    for pattern in patterns {
        println!(
            "Origin {}\tField {}\tText {}",
            pattern,
            SqliteQuoted::Field(pattern.to_string()).get(),
            SqliteQuoted::Text(pattern.to_string()).get()
        );
    }
}
