use crate::db::is_reserved_table_name::is_reserved_table_name;

/// Retrieves user-defined table names from the SQLite database.
/// Filters out internal SQLite tables like `sqlite_sequence` or index tables.
///
/// # Arguments
/// * `pool` - SQLite database connection pool.
///
/// # Returns
/// A vector of table names or an error if the query fails.
pub fn list_available_tables(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
) -> anyhow::Result<Vec<String>> {
    let conn = pool.get()?;

    // Query the SQLite master table for table names.
    let mut stmt = conn.prepare("PRAGMA table_list")?;
    let column_index_name = stmt.column_index("name")?;
    let mut rows = stmt.query([])?;
    let mut values: Vec<String> = vec![];

    // Iterate through the result rows.
    while let Some(row) = rows.next()? {
        let value: String = row.get(column_index_name)?;

        // Exclude internal SQLite tables.
        if !is_reserved_table_name(&value) {
            values.push(value);
        }
    }
    Ok(values)
}
