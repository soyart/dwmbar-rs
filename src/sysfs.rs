use glob::glob;
use std::path::PathBuf;

/// Find the first file matching the glob pattern.
/// Returns None if no matches are found or if there's an error.
pub(crate) fn find_first_match(pattern: &str) -> Option<PathBuf> {
    glob(pattern)
        .ok()?
        .filter_map(|entry| entry.ok())
        .next()
}

/// Find all files matching the glob pattern.
/// Returns an empty vector if no matches are found or if there's an error.
pub(crate) fn find_all_matches(pattern: &str) -> Vec<PathBuf> {
    glob(pattern)
        .ok()
        .map(|entries| entries.filter_map(|entry| entry.ok()).collect())
        .unwrap_or_default()
}
