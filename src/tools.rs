use std::collections::HashSet;

/// Parses command line arguments to detect which flags and parameters were passed.
///
/// # Arguments
///
/// * `short_flags` - A slice of characters representing short command line flags (e.g., 'f', 'v').
/// * `long_flags` - A slice of strings representing long command line flags (e.g., "help", "version").
/// * `params` - A slice of strings representing expected parameters (non-flag arguments).
///
/// # Returns
///
/// A tuple containing:
/// - `HashSet<String>`: A set of detected flags (both short and long).
/// - `Vec<String>`: A vector of detected parameters.
///
/// # Example
///
/// ```rust
/// use std::collections::HashSet;
/// use coretilus::tools::parse_args;
///
/// let args = vec![
///     "prog".to_string(),
///     "-f".to_string(),
///     "--help".to_string(),
///     "file.txt".to_string(),
/// ];
///
/// let (flags, params) = parse_args(
///     args,
///     &['f', 'v'],
///     &["help", "version"],
///     &["file.txt", "file2.txt"]
/// );
///
/// assert!(flags.contains("f"));
/// assert!(flags.contains("help"));
/// assert!(!flags.contains("v"));
/// assert!(!flags.contains("version"));
/// assert_eq!(params, vec!["file.txt"]);
/// ```
pub fn parse_args(
    arguments2: Vec<String>,
    short_flags: &[char],
    long_flags: &[&str],
    params: &[&str],
) -> (HashSet<String>, Vec<String>)
where
{
    let arguments: Vec<String> = arguments2;
    let mut detected_flags = HashSet::new();
    let mut detected_params = Vec::new();

    for argument in arguments {
        if let Some(name) = argument.strip_prefix("--") {
            // Flag long
            //let name = &argument[2..];
            if long_flags.contains(&name) {
                detected_flags.insert(name.to_string());
            }
        } else if argument.starts_with('-') && argument.len() > 1 {
            // Flags courts combinés : -fr
            for ch in argument.chars().skip(1) {
                if short_flags.contains(&ch) {
                    detected_flags.insert(ch.to_string());
                }
            }
        } else if params.contains(&argument.as_str()) {
            detected_params.push(argument.clone());
        }
    }

    (detected_flags, detected_params)
}
