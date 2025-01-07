use std::fmt::{Display, Formatter};

/// SQLite data types for mapping CSV data to SQLite-compatible types.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SqliteDataType {
    Integer,
    Real,
    Text,
}

impl Display for SqliteDataType {
    /// Converts `SqliteDataType` to its string representation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SqliteDataType::Integer => "INTEGER",
            SqliteDataType::Real => "REAL",
            SqliteDataType::Text => "TEXT",
        };
        write!(f, "{}", str)
    }
}

impl SqliteDataType {
    /// Checks if conversion between types is valid.
    pub fn can_convert(&self, to: SqliteDataType) -> bool {
        match to {
            SqliteDataType::Integer => {
                // Conversion to Integer is not allowed
                false
            }
            SqliteDataType::Real => {
                // Integer to Real is allowed
                *self == SqliteDataType::Integer
            }
            SqliteDataType::Text => {
                // All can convert to Text
                *self != SqliteDataType::Text
            }
        }
    }

    /// Detects the type of a given string based on its format.
    pub fn detect_type(s: &str) -> Self {
        let re = lazy_regex::regex!(r#"^-?[1-9]\d*\.?\d*$"#);

        if re.is_match(s) {
            if s.contains(".") {
                Self::Real
            } else {
                Self::Integer
            }
        } else {
            Self::Text
        }
    }
}
