use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

// Helper function to create test files
fn create_test_file(dir: &TempDir, filename: &str, content: &str) -> String {
    let file_path = dir.path().join(filename);
    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", content).unwrap();
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_cli_with_valid_arguments() {
    // Create a temp directory and test file
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    // Run the command
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .args(["rust", &file_path])
        .output()
        .expect("Failed to execute command");

    // Assert success and expected output
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "This line has rust in it\n");
}

#[test]
fn test_cli_case_insensitive_search() {
    // Create a temp directory and test file
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    // Run the command with IGNORE_CASE env var
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .env("IGNORE_CASE", "1")
        .args(["rust", &file_path])
        .output()
        .expect("Failed to execute command");

    // Assert success and expected output
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout,
        "Rust is awesome\nThis line has rust in it\nRust again here\n"
    );
}

#[test]
fn test_cli_with_missing_arguments() {
    // Run the command with missing file path
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .arg("query")
        .output()
        .expect("Failed to execute command");

    // Assert failure and expected error message
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Didn't get a file path"));
}

#[test]
fn test_cli_with_nonexistent_file() {
    // Run the command with a nonexistent file
    let output = Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .args(["query", "nonexistent_file.txt"])
        .output()
        .expect("Failed to execute command");

    // Assert failure and expected error message
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Application error"));
}
