use regex::Regex;
use std::collections::HashSet;

/// Parses command line arguments to detect which flags and parameters were passed.
///
/// # Arguments
///
/// * `arguments` - A vector of strings representing command line arguments.
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
///     args.clone(),
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
///
/// let (flags, params) = parse_args(
///     args.clone(),
///     &['p', 'r'],
///     &["bad"],
///     &["file.txt", "file2.txt"]
/// );
///
/// assert!(!flags.contains("p"));
/// assert!(!flags.contains("r"));
/// assert!(!flags.contains("bad"));
/// assert_eq!(params, vec!["file.txt"]);
/// ```
pub fn parse_args(
    arguments: Vec<String>,
    short_flags: &[char],
    long_flags: &[&str],
    params: &[&str],
) -> (HashSet<String>, Vec<String>) {
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

/// Filters parameters from a list of arguments based on regular expression patterns.
///
/// # Arguments
///
/// * `arguments` - A vector of strings representing command line arguments.
/// * `patterns` - A slice of string slices representing regular expression patterns.
///
/// # Returns
///
/// A vector of strings containing arguments that match any of the provided regular expression patterns.
///
/// # Examples
///
/// ```rust
/// use regex::Regex;
/// use std::vec;
/// use coretilus::tools::filter_params_regex;
///
/// let args = vec![
///     "file.txt".to_string(),
///     "image.png".to_string(),
///     "data.csv".to_string(),
///     "notes.md".to_string(),
/// ];
///
/// let result = filter_params_regex(args.clone(), &["txt$"]);
/// assert_eq!(result, vec!["file.txt".to_string()]);
///
/// let result = filter_params_regex(args.clone(), &["png$", "md$"]);
/// assert_eq!(result, vec!["image.png".to_string(), "notes.md".to_string()]);
///
/// let result = filter_params_regex(args.clone(), &[".*"]);
/// assert_eq!(result, vec![
///     "file.txt".to_string(),
///     "image.png".to_string(),
///     "data.csv".to_string(),
///     "notes.md".to_string(),
/// ]);
/// ```
///
/// This function ignores invalid regular expressions and will not include arguments that do not match any valid pattern.
pub fn filter_params_regex(arguments: Vec<String>, patterns: &[&str]) -> Vec<String> {
    let regexes: Vec<Regex> = patterns.iter().filter_map(|p| Regex::new(p).ok()).collect();
    arguments
        .into_iter()
        .filter(|param| regexes.iter().any(|re| re.is_match(param)))
        .collect()
}
