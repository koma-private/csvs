/// Captures the result of an executed SQL statement.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct StatementResult {
    /// Column names from the query result.
    pub header: Vec<String>,

    /// Rows returned by the query, each row as a vector of strings.
    pub rows: Vec<Vec<String>>,

    /// Time taken to execute the SQL statement.
    pub elapsed: std::time::Duration,
}

/// Paged representation of an SQL execution result.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct StatementPagedResult {
    /// Column names from the query result.
    pub header: Vec<String>,

    /// Rows returned by the query, each row as a vector of strings.
    pub rows: Vec<Vec<String>>,

    /// Time taken to execute the SQL statement.
    pub elapsed: std::time::Duration,

    /// Current page index in the result set.
    pub current_page_index: usize,

    /// Row position at the start of the current page.
    pub initial_row_position: usize,

    /// Maximum number of pages in the result set.
    pub page_upper_limit: usize,

    /// Size of a single page of results.
    pub page_size: usize,

    /// Total rows in the result set.
    pub total_rows: usize,

    /// Optional total number of columns.
    pub total_columns: Option<usize>,
}
