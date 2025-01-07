#[derive(Default, Debug)]
/// Manages input history with navigation capabilities.
pub struct InputHistory {
    /// Stores history entries.
    data: Vec<String>,
    /// Current position in the history.
    cursor: usize,
}

impl InputHistory {
    /// Moves the cursor backward in the history.
    /// Returns the current entry, if available.
    pub fn back(&mut self) -> Option<&String> {
        if self.cursor_back() {
            self.get_data()
        } else {
            None
        }
    }

    /// Moves the cursor forward in the history.
    /// Returns the current entry, if available.
    pub fn forward(&mut self) -> Option<&String> {
        if self.cursor_forward() {
            self.get_data()
        } else {
            None
        }
    }

    /// Checks if a specific string exists in the history.
    pub fn contains(&self, str: &str) -> bool {
        self.data.contains(&str.to_string())
    }

    /// Adds a new entry to the history and moves the cursor to the latest position.
    pub fn push(&mut self, str: String) {
        self.data.push(str);
        self.cursor_tail();
    }

    /// Moves the cursor one step backward.
    /// Returns true if the move is successful.
    fn cursor_back(&mut self) -> bool {
        let lower_limit = self.get_cursor_limit().0;
        if self.cursor > lower_limit {
            self.cursor -= 1;
            true
        } else {
            self.cursor = lower_limit;
            false
        }
    }

    /// Moves the cursor one step forward.
    /// Returns true if the move is successful.
    fn cursor_forward(&mut self) -> bool {
        let upper_limit = self.get_cursor_limit().1;
        if self.cursor < upper_limit {
            self.cursor += 1;
            true
        } else {
            self.cursor = upper_limit;
            false
        }
    }

    /// Gets the current limits for the cursor position.
    fn get_cursor_limit(&self) -> (usize, usize) {
        let upper_limit = if self.data.is_empty() {
            0
        } else {
            self.data.len().saturating_sub(1)
        };
        (0, upper_limit)
    }

    /// Retrieves the current entry at the cursor position.
    fn get_data(&self) -> Option<&String> {
        let result = self.data.get(self.cursor);
        result
    }

    /// Moves the cursor to the latest entry in the history.
    pub fn cursor_tail(&mut self) {
        self.cursor = self.get_cursor_limit().1;
    }
}
