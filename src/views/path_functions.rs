use std::path::PathBuf;

pub fn check_binary_exists(binary_name: &str) -> Option<PathBuf> {
    // which() returns a Result<PathBuf, Error>
    which::which(binary_name).ok()
}
