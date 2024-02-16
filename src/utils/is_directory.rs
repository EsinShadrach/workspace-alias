use std::fs;

/// Returns a boolean based on if a string is a directory or not
/// # Arguments
/// - `path` A string representing the directory path to be checked for
///
/// # Example
/// ```
/// let my_dir = "secret-dir";
/// if is_directory(my_dir) {
///     // Do something
/// } else {
///     // do Alternative
/// }
/// ```
///
/// # Panics
///
/// This function doesn't panic
///
/// # Safety
///
/// This function is safe
pub fn is_directory(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(x) => {
            return x.is_dir();
        }
        Err(_) => return false,
    }
}
