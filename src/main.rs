//! # Minigrep CLI
//!
//! This is the command-line interface for the `minigrep` library.
//! It handles argument parsing and error reporting for the search functionality.
//!
//! ## Features
//!
//! - Search for text patterns in one or more files
//! - Support for case-sensitive and case-insensitive searches
//! - Regular expression pattern matching
//! - Line number output
//!
//! ## Usage
//!
//! ```bash
//! # Basic usage
//! $ minigrep <pattern> <file1> [file2 ...]
//!
//! # Case-insensitive search
//! $ IGNORE_CASE=1 minigrep <pattern> <file1> [file2 ...]
//!
//! # Regular expression search
//! $ USE_REGEX=1 minigrep <pattern> <file1> [file2 ...]
//!
//! # Display line numbers
//! $ SHOW_LINE_NUMBERS=1 minigrep <pattern> <file1> [file2 ...]
//!
//! # Combine options
//! $ IGNORE_CASE=1 USE_REGEX=1 SHOW_LINE_NUMBERS=1 minigrep <pattern> <file1> [file2 ...]
//! ```
use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

/// The main entry point for the minigrep command line tool.
///
/// This function parses command line arguments using `env::args()`,
/// constructs a `Config` object, and executes the search operation.
/// If any errors occur during argument parsing or execution,
/// they are reported to stderr and the program exits with a non-zero status code.
///
/// # Examples
///
/// ```bash
/// # Search for "pattern" in file.txt (case-sensitive)
/// $ minigrep pattern file.txt
///
/// # Search for "pattern" in multiple files (case-insensitive)
/// $ IGNORE_CASE=1 minigrep pattern file1.txt file2.txt
///
/// # Search using regex with line numbers
/// $ USE_REGEX=1 SHOW_LINE_NUMBERS=1 minigrep "^[a-z]+" file.txt
/// ```
fn main() {
    // Print usage information if no arguments are provided
    if env::args().len() <= 1 {
        print_usage();
        process::exit(1);
    }

    // Parse command line arguments into a Config struct
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        print_usage();
        process::exit(1);
    });

    // Display search configuration
    print_config(&config);

    // Run the search operation with the provided configuration
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// Print usage information to stdout
fn print_usage() {
    println!("Usage: minigrep <pattern> <file1> [file2 ...]");
    println!("Environment variables:");
    println!("  IGNORE_CASE=1        Perform case-insensitive search");
    println!("  USE_REGEX=1          Treat pattern as a regular expression");
    println!("  SHOW_LINE_NUMBERS=1  Display line numbers in search results");
    println!("\nExamples:");
    println!("  minigrep rust file.txt                  # Search for 'rust' in file.txt");
    println!("  IGNORE_CASE=1 minigrep rust *.rs        # Case-insensitive search for 'rust' in .rs files");
    println!(
        "  USE_REGEX=1 minigrep '^fn\\s+\\w+' *.rs  # Search for function definitions in .rs files"
    );
}

/// Display the current search configuration
///
/// # Arguments
///
/// * `config` - A reference to the Config struct containing search parameters
fn print_config(config: &Config) {
    eprintln!("Searching for: {}", config.query);
    eprintln!("In file(s): {}", config.file_paths.join(", "));
    eprintln!("Search options:");
    eprintln!(
        "  Case insensitive: {}",
        if config.ignore_case { "yes" } else { "no" }
    );
    eprintln!(
        "  Using regex: {}",
        if config.use_regex { "yes" } else { "no" }
    );
    eprintln!(
        "  Show line numbers: {}",
        if config.show_line_numbers {
            "yes"
        } else {
            "no"
        }
    );
    eprintln!("");
}
