use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    /// Checks if conversion between types is valid based on SQLite rules.
    ///
    /// Conversion Matrix:
    /// - `Integer` can convert to any type.
    /// - `Real` can convert to `Real` or `Text`.
    /// - `Text` can only remain `Text`.
    pub fn can_convert_into(&self, to: SqliteDataType) -> bool {
        match self {
            SqliteDataType::Integer => true,
            SqliteDataType::Real => to != SqliteDataType::Integer,
            SqliteDataType::Text => to == SqliteDataType::Text,
        }
    }

    /// Detects the type of given string based on its format.
    ///
    /// # Arguments
    /// * `s` - Input string to analyze.
    /// * `allow_leading_zeros` - Whether numbers with leading zeros are allowed.
    ///
    /// # Returns
    /// * `SqliteDataType` - The inferred data type.
    pub fn detect_type(s: &str, allow_leading_zeros: bool) -> Self {
        if is_valid_number(s, allow_leading_zeros) {
            if i64::from_str(s).is_ok() {
                Self::Integer
            } else if f64::from_str(s).is_ok() {
                Self::Real
            } else {
                Self::Text
            }
        } else {
            Self::Text
        }
    }
}

/// Validates if the input string is a number based on the specified format.
///
/// # Arguments
/// * `source` - The string to validate.
/// * `allow_leading_zeros` - Flag to allow leading zeros in numbers.
///
/// # Returns
/// * `bool` - `true` if valid, otherwise `false`.
fn is_valid_number(source: &str, allow_leading_zeros: bool) -> bool {
    let re = if allow_leading_zeros {
        lazy_regex::regex!(r#"^-?((\d+)|(\d+\.\d*)|(\.\d+))$"#)
    } else {
        lazy_regex::regex!(r#"^-?(([1-9]\d*)|([1-9]\d*\.\d*)|(0?\.\d+)|0)$"#)
    };
    re.is_match(source)
}

#[test]
fn test_is_valid_number() {
    let positive_cases = vec![
        "0", "-0", "0.0", "-0.0", ".0", "-.0", "0.10", "-0.10", ".120", "-.120", "1", "-1", "1.",
        "-1.", "1.0", "-1.0", "10", "-10", "10.", "-10.", "10.0", "-10.0",
    ];
    let leading_zeros_cases = vec!["001", "-00"];
    let negative_cases = vec![
        "1a", "2..1", "..2", "--0", // Contains space
        " 10", "102 ", "4 5",
    ];

    for allow_leading_zeros_for_number in [true, false] {
        println!(
            "allow_leading_zeros_for_number:{}",
            allow_leading_zeros_for_number
        );
        for case in positive_cases.clone() {
            println!(
                "[positive]      {} - {}",
                case,
                is_valid_number(case, allow_leading_zeros_for_number)
            );
        }
        for case in leading_zeros_cases.clone() {
            println!(
                "[leading_zeros] {} - {}",
                case,
                is_valid_number(case, allow_leading_zeros_for_number)
            );
        }
        for case in negative_cases.clone() {
            println!(
                "[negative]      {} - {}",
                case,
                is_valid_number(case, allow_leading_zeros_for_number)
            );
        }
    }
}
