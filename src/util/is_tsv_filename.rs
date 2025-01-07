/// Checks if a filename has a `.tsv` extension (case-insensitive).
///
/// # Arguments
/// * `filename` - The file name or path to check.
///
/// # Returns
/// `true` if the file has a `.tsv` extension, `false` otherwise.
pub fn is_tsv_filename(filename: &str) -> bool {
    // Convert to Path object
    let path = std::path::Path::new(filename);

    // Check for extension
    if let Some(extension) = path.extension() {
        // Validate extension as str
        if let Some(extension) = extension.to_str() {
            // Compare case-insensitively.
            let is_tsv = extension.to_lowercase().eq("tsv");
            return is_tsv;
        }
    }
    // No `.tsv` extension found
    false
}
