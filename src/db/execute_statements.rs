use crate::db::statement_result::StatementResult;

use tracing::debug;

/// Executes SQL statements and returns their results.
///
/// Handles queries, PRAGMA settings, and other operations (e.g., INSERT, UPDATE).
/// Logs execution details including columns, rows, and timing.
///
/// # Arguments
/// * `pool` - SQLite connection pool.
/// * `statements` - SQL statements to execute.
/// * `raw_id` - Optional identifier for internal column filtering.
///
/// # Returns
/// A vector of `StatementResult` with execution details.
pub fn execute_statements(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    statements: Vec<sqlparser::ast::Statement>,
    raw_id: Option<String>,
) -> anyhow::Result<Vec<StatementResult>> {
    let conn = pool.get()?;

    debug!("Executing {} SQL statements.", statements.len());
    let mut statement_results: Vec<StatementResult> = vec![];
    let raw_id = raw_id.unwrap_or("_raw_id".to_string());

    for statement in statements {
        let mut statement_result = StatementResult::default();

        match statement {
            sqlparser::ast::Statement::Query(_)
            | sqlparser::ast::Statement::Pragma { value: None, .. } => {
                debug!("Running query/pragma: {}", statement);

                let start = std::time::Instant::now();
                let mut stmt = conn.prepare(&statement.to_string())?;
                let temp_column_names = stmt.column_names();

                let mut i = 0;
                let mut column_names_map: std::collections::HashMap<usize, String> =
                    std::collections::HashMap::new();
                temp_column_names.iter().for_each(|v| {
                    column_names_map.insert(i, v.to_string());
                    i += 1;
                });

                let excluded_index: Vec<usize> = column_names_map
                    .iter()
                    .filter(|v| v.1.eq(&raw_id))
                    .map(|v| *v.0)
                    .collect();
                let column_count = temp_column_names.len();

                let mut column_names: Vec<String> = vec![];
                for index in 0..column_count {
                    if excluded_index.contains(&index) {
                        continue;
                    }
                    if let Some(found) = column_names_map.get(&index) {
                        column_names.push(found.to_string());
                    }
                }
                statement_result.header = column_names;

                let mut rows = stmt.query([])?;
                let mut row_index: usize = 0;

                while let Some(row) = rows.next()? {
                    let mut values: Vec<String> = vec![];

                    for index in 0..column_count {
                        if excluded_index.contains(&index) {
                            continue;
                        }

                        let null = "".to_string();

                        let value: rusqlite::Result<Option<String>> = row.get(index);
                        if let Ok(value) = value {
                            match value {
                                None => {
                                    values.push(null);
                                }
                                Some(value) => {
                                    values.push(value);
                                }
                            }
                            continue;
                        }

                        let value: rusqlite::Result<Option<isize>> = row.get(index);
                        if let Ok(value) = value {
                            match value {
                                None => {
                                    values.push(null);
                                }
                                Some(value) => {
                                    values.push(value.to_string());
                                }
                            }
                            continue;
                        }

                        let value: rusqlite::Result<Option<f64>> = row.get(index);
                        if let Ok(value) = value {
                            match value {
                                None => {
                                    values.push(null);
                                }
                                Some(value) => {
                                    values.push(value.to_string());
                                }
                            }
                            continue;
                        }

                        anyhow::bail!(
                            "Invalid data type at row:{} col:{}",
                            row_index.saturating_add(1),
                            index.saturating_add(1)
                        );
                    }
                    statement_result.rows.push(values);
                    row_index = row_index.saturating_add(1);
                }
                statement_result.elapsed = start.elapsed();

                debug!(
                    "Query completed. Rows: {}, Time: {:?}",
                    row_index, statement_result.elapsed
                );
            }
            other_statement => {
                debug!("Executing non-query statement: {}", other_statement);
                let affected_rows_enabled = matches!(
                    other_statement,
                    sqlparser::ast::Statement::Insert(_)
                        | sqlparser::ast::Statement::Update { .. }
                        | sqlparser::ast::Statement::Delete(_)
                );

                let start = std::time::Instant::now();

                let result = conn.execute(&other_statement.to_string(), [])?;

                let mut header = vec!["executed_statement".to_string()];
                let mut row = vec![other_statement.to_string()];

                if affected_rows_enabled {
                    header.push("affected_rows".to_string());
                    row.push(result.to_string());
                }

                statement_result.header = header;
                statement_result.rows = vec![row];
                statement_result.elapsed = start.elapsed();

                debug!(
                    "Non-query completed. Rows affected: {}, Time: {:?}",
                    result, statement_result.elapsed
                );
            }
        }
        statement_results.push(statement_result);
    }

    debug!(
        "Executed all statements. Total results: {}",
        statement_results.len()
    );
    Ok(statement_results)
}
