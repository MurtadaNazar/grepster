//! # Grepster
//!
//! `grepster` is a simple command-line utility for searching text in files.
//! It allows case-sensitive and case-insensitive searches controlled via environment variables.
//! The enhanced version adds support for multiple files, regex pattern matching, and line number output.
//!
//! ## Features
//!
//! - Search for text patterns in files
//! - Support for case-sensitive and case-insensitive searches
//! - Search multiple files at once
//! - Regular expression pattern matching
//! - Display line numbers in search results
//! - Simple and intuitive command-line interface
//!
//! ## Example
//!
//! ```no_run
//! use std::env;
//! use grepster::{Config, run};
//!
//! let config = Config::build(env::args()).unwrap_or_else(|err| {
//!     eprintln!("Problem parsing arguments: {err}");
//!     std::process::exit(1);
//! });
//!
//! if let Err(e) = run(config) {
//!     eprintln!("Application error: {e}");
//!     std::process::exit(1);
//! }
//! ```

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

/// Configuration for the `grepster` application.
///
/// This structure holds all the parameters needed to perform a search operation,
/// including the query string, file path(s), and various search options.
#[derive(Debug)]
pub struct Config {
    /// The text pattern to search for in the file(s)
    pub query: String,

    /// The path(s) to the file(s) that will be searched
    pub file_paths: Vec<String>,

    /// Whether the search should be case-insensitive
    pub ignore_case: bool,

    /// Whether to use regular expression matching
    pub use_regex: bool,

    /// Whether to display line numbers in search results
    pub show_line_numbers: bool,
}

/// Represents a search result with line content and metadata
#[derive(Debug, PartialEq)]
pub struct SearchResult<'a> {
    /// The file path where the match was found
    pub file_path: &'a str,

    /// The line number where the match was found (1-indexed)
    pub line_number: usize,

    /// The content of the matching line
    pub line_content: &'a str,
}

impl Config {
    /// Creates a new `Config` instance from command-line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator over the command-line arguments.
    ///   The first argument is assumed to be the program name and is skipped.
    ///   The second argument should be the search query.
    ///   The third and subsequent arguments should be file paths.
    ///
    /// # Returns
    ///
    /// * `Result<Config, &'static str>` - A `Config` instance if all required arguments are provided,
    ///   or an error message if any required argument is missing.
    ///
    /// # Environment Variables
    ///
    /// * `IGNORE_CASE` - If set (to any value), the search will be case-insensitive.
    /// * `USE_REGEX` - If set (to any value), the query will be treated as a regular expression.
    /// * `SHOW_LINE_NUMBERS` - If set (to any value), line numbers will be displayed in the results.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::env;
    /// use grepster::Config;
    ///
    /// let args = vec![
    ///     String::from("program_name"),
    ///     String::from("query"),
    ///     String::from("file1.txt"),
    ///     String::from("file2.txt")
    /// ];
    ///
    /// let config = Config::build(args.into_iter()).unwrap();
    /// assert_eq!(config.query, "query");
    /// assert_eq!(config.file_paths, vec![String::from("file1.txt"), String::from("file2.txt")]);
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_paths: Vec<String> = args.collect();
        if file_paths.is_empty() {
            return Err("Didn't get a file path");
        }

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        let use_regex: bool = env::var("USE_REGEX").is_ok();
        let show_line_numbers: bool = env::var("SHOW_LINE_NUMBERS").is_ok();

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            use_regex,
            show_line_numbers,
        })
    }
}

/// Runs the search operation with the provided configuration.
///
/// # Arguments
///
/// * `config` - A `Config` instance containing the search parameters.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - `Ok(())` if the operation was successful,
///   or an error if the file could not be read or the regex pattern was invalid.
///
/// # Examples
///
/// ```no_run
/// use grepster::{Config, run};
///
/// let config = Config {
///     query: String::from("pattern"),
///     file_paths: vec![String::from("file.txt")],
///     ignore_case: false,
///     use_regex: false,
///     show_line_numbers: false,
/// };
///
/// if let Err(e) = run(config) {
///     eprintln!("Application error: {e}");
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut found_errors = false;
    let mut found_results = false;

    for file_path in &config.file_paths {
        let contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
                found_errors = true;
                continue;
            }
        };

        let results = if config.use_regex {
            search_with_regex(&config.query, &contents, file_path, config.ignore_case)?
        } else if config.ignore_case {
            search_case_insensitive(&config.query, &contents, file_path)
        } else {
            search(&config.query, &contents, file_path)
        };

        if !results.is_empty() {
            found_results = true;

            // Only print file header if searching multiple files
            if config.file_paths.len() > 1 {
                // Extract just the filename for cleaner output
                let filename = std::path::Path::new(file_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(file_path);
                println!("File: {}", filename);
            }

            for result in results {
                if config.show_line_numbers {
                    println!(
                        "{}:{}: {}",
                        result.file_path, result.line_number, result.line_content
                    );
                } else {
                    println!("{}", result.line_content);
                }
            }

            // Add a blank line between files for better readability
            if config.file_paths.len() > 1 {
                println!();
            }
        }
    }

    if found_errors {
        Err("One or more files could not be read".into())
    } else if !found_results {
        Err("No matches found".into())
    } else {
        Ok(())
    }
}

/// Performs a case-sensitive search for a query in the contents.
///
/// # Arguments
///
/// * `query` - The text pattern to search for.
/// * `contents` - The text content to search in.
/// * `file_path` - The path of the file being searched.
///
/// # Returns
///
/// * `Vec<SearchResult>` - A vector of search results that contain the query.
///
/// # Examples
///
/// ```
/// use grepster::{search, SearchResult};
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// let file_path = "example.txt";
///
/// let results = search(query, contents, file_path);
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0].line_content, "safe, fast, productive.");
/// assert_eq!(results[0].line_number, 2);
/// ```
pub fn search<'a>(query: &str, contents: &'a str, file_path: &'a str) -> Vec<SearchResult<'a>> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.contains(query) {
                Some(SearchResult {
                    file_path,
                    line_number: i + 1, // 1-indexed line numbers
                    line_content: line,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Performs a case-insensitive search for a query in the contents.
///
/// # Arguments
///
/// * `query` - The text pattern to search for (case doesn't matter).
/// * `contents` - The text content to search in.
/// * `file_path` - The path of the file being searched.
///
/// # Returns
///
/// * `Vec<SearchResult>` - A vector of search results that contain the query,
///   regardless of case.
///
/// # Examples
///
/// ```
/// use grepster::{search_case_insensitive, SearchResult};
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
/// let file_path = "example.txt";
///
/// let results = search_case_insensitive(query, contents, file_path);
/// assert_eq!(results.len(), 2);
/// assert_eq!(results[0].line_content, "Rust:");
/// assert_eq!(results[1].line_content, "Trust me.");
/// ```
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    file_path: &'a str,
) -> Vec<SearchResult<'a>> {
    let query = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.to_lowercase().contains(&query) {
                Some(SearchResult {
                    file_path,
                    line_number: i + 1, // 1-indexed line numbers
                    line_content: line,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Performs a search using regular expressions.
///
/// # Arguments
///
/// * `pattern` - The regex pattern to search for.
/// * `contents` - The text content to search in.
/// * `file_path` - The path of the file being searched.
/// * `ignore_case` - Whether the search should be case-insensitive.
///
/// # Returns
///
/// * `Result<Vec<SearchResult>, Box<dyn Error>>` - A vector of search results that match the pattern,
///   or an error if the regex pattern is invalid.
///
/// # Examples
///
/// ```
/// use grepster::search_with_regex;
///
/// let pattern = r"\w+:\s*\d+";  // Match word followed by colon and number
/// let contents = "\
/// Code: 123
/// Another line
/// Error: 404";
/// let file_path = "example.txt";
///
/// let results = search_with_regex(pattern, contents, file_path, false).unwrap();
/// assert_eq!(results.len(), 2);
/// assert_eq!(results[0].line_content, "Code: 123");
/// assert_eq!(results[1].line_content, "Error: 404");
/// ```
pub fn search_with_regex<'a>(
    pattern: &str,
    contents: &'a str,
    file_path: &'a str,
    ignore_case: bool,
) -> Result<Vec<SearchResult<'a>>, Box<dyn Error>> {
    let regex_options = format!("{}{}", if ignore_case { "(?i)" } else { "" }, pattern);

    let regex = Regex::new(&regex_options)?;

    Ok(contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if regex.is_match(line) {
                Some(SearchResult {
                    file_path,
                    line_number: i + 1, // 1-indexed line numbers
                    line_content: line,
                })
            } else {
                None
            }
        })
        .collect())
}
