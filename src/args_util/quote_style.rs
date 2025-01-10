/// Quoting styles for CSV output.
/// Determines how fields are quoted in the output.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum MyQuoteStyle {
    /// Always quote all fields.
    Always,
    /// Quote only when necessary (e.g., special characters).
    Necessary,
    /// Quote non-numeric fields only.
    NonNumeric,
    /// Never quote fields.
    Never,
}

impl From<MyQuoteStyle> for csv::QuoteStyle {
    /// Converts `MyQuoteStyle` to `csv::QuoteStyle`.
    fn from(value: MyQuoteStyle) -> Self {
        match value {
            MyQuoteStyle::Always => csv::QuoteStyle::Always,
            MyQuoteStyle::Necessary => csv::QuoteStyle::Necessary,
            MyQuoteStyle::NonNumeric => csv::QuoteStyle::NonNumeric,
            MyQuoteStyle::Never => csv::QuoteStyle::Never,
        }
    }
}
