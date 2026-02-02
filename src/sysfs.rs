use glob::glob;
use std::fs;
use std::path::{
    Path,
    PathBuf,
};

/// Find all files matching the glob pattern.
/// Returns an empty vector if no matches are found or if there's an error.
pub(crate) fn find_matches(pattern: &str) -> Vec<PathBuf> {
    glob(pattern)
        .ok()
        .map(|entries| entries.filter_map(|entry| entry.ok()).collect())
        .unwrap_or_default()
}

pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}
