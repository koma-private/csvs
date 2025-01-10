/// Checks if a table name is reserved by SQLite.
///
/// # Arguments
/// * `table_name` - The name of the table to check.
///
/// # Returns
/// * `true` if the table name is reserved, otherwise `false`.
///
/// Reserved table names are defined by SQLite to start with "sqlite_".
/// See: https://www.sqlite.org/fileformat2.html
/// > New internal schema objects names, always beginning with "sqlite_",
/// > may be added to the SQLite file format in future releases.
pub fn is_reserved_table_name(table_name: &str) -> bool {
    table_name.starts_with("sqlite_")
}
