use crate::db::statement_result::{StatementPagedResult, StatementResult};
use crate::tui::database_event_listener::DatabaseEventListener;

impl DatabaseEventListener {
    /// Retrieves a paginated subset of SQL results.
    ///
    /// # Arguments
    /// * `query_result` - Full result of the SQL query.
    /// * `initial_row_position` - Initial cursor position.
    /// * `current_page_index` - Index of the page to retrieve.
    ///
    /// # Returns
    /// `StatementPagedResult` containing the paginated rows, or `None` if the page index is invalid.
    pub fn sql_result_by_page(
        query_result: &StatementResult,
        initial_row_position: usize,
        current_page_index: usize,
    ) -> Option<StatementPagedResult> {
        let page_size = 100;

        let rows_len = query_result.rows.len();
        let page_upper_limit = rows_len.div_ceil(page_size).saturating_sub(1);

        if current_page_index <= page_upper_limit {
            let mut row_upper_limit = current_page_index
                .saturating_add(1)
                .saturating_mul(page_size);
            if row_upper_limit > rows_len {
                row_upper_limit = rows_len;
            }
            let row_lower_limit = current_page_index.saturating_mul(page_size);

            let partial_rows: Vec<Vec<String>> =
                query_result.rows[row_lower_limit..row_upper_limit].to_vec();

            let total_columns = query_result.rows.first().map(|first| first.len());

            Some(StatementPagedResult {
                current_page_index,
                elapsed: query_result.elapsed,
                header: query_result.header.clone(),
                page_size,
                page_upper_limit,
                initial_row_position,
                rows: partial_rows,
                total_columns,
                total_rows: query_result.rows.len(),
            })
        } else {
            None
        }
    }
}
