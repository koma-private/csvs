/// Defines trimming behavior for CSV processing.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum MyTrim {
    /// Trim whitespace from headers and fields.
    All,
    /// Trim whitespace only from fields.
    Fields,
    /// Trim whitespace only from headers.
    Headers,
    /// Do not trim whitespace.
    None,
}

impl From<MyTrim> for csv::Trim {
    /// Converts `MyTrim` to `csv::Trim`.
    fn from(value: MyTrim) -> Self {
        match value {
            MyTrim::All => csv::Trim::All,
            MyTrim::Fields => csv::Trim::Fields,
            MyTrim::Headers => csv::Trim::Headers,
            MyTrim::None => csv::Trim::None,
        }
    }
}
