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

// Helper function to get the binary path at runtime
fn get_binary_path() -> String {
    std::env::var("CARGO_BIN_EXE_grepster").expect("CARGO_BIN_EXE_grepster not set")
}

#[test]
fn test_cli_with_valid_arguments() {
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let output = Command::new(get_binary_path())
        .args(["rust", &file_path])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "This line has rust in it\n");
}

#[test]
fn test_cli_case_insensitive_search() {
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let output = Command::new(get_binary_path())
        .env("IGNORE_CASE", "1")
        .args(["rust", &file_path])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout,
        "Rust is awesome\nThis line has rust in it\nRust again here\n"
    );
}

#[test]
fn test_cli_with_missing_arguments() {
    let output = Command::new(get_binary_path())
        .arg("query")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Didn't get a file path"));
}

#[test]
fn test_cli_with_nonexistent_file() {
    let output = Command::new(get_binary_path())
        .args(["query", "nonexistent_file.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error reading file"));
    assert!(stderr.contains("nonexistent_file.txt"));
}

#[test]
fn test_cli_with_mixed_existing_and_nonexistent_files() {
    let temp_dir = TempDir::new().unwrap();
    let content = "Test content";
    let good_file = create_test_file(&temp_dir, "good.txt", content);

    let output = Command::new(get_binary_path())
        .args(["content", &good_file, "nonexistent.txt"])
        .output()
        .expect("Failed to execute command");

    // Should fail because one file couldn't be read
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show error for bad file
    assert!(stderr.contains("Error reading file"));
    assert!(stderr.contains("nonexistent.txt"));

    // Should still show results from good file
    assert!(stdout.contains("Test content"));
}

#[test]
fn test_cli_no_matches_found() {
    let temp_dir = TempDir::new().unwrap();
    let content = "No matches here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let output = Command::new(get_binary_path())
        .args(["pattern", &file_path])
        .output()
        .expect("Failed to execute command");

    // Should fail because no matches found
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("No matches found"));
}

#[test]
fn test_cli_multiple_files_with_results() {
    let temp_dir = TempDir::new().unwrap();
    let content1 = "First file\nContains rust\nEnd";
    let file1 = create_test_file(&temp_dir, "file1.txt", content1);
    let content2 = "Second file\nAlso contains rust\nEnd";
    let file2 = create_test_file(&temp_dir, "file2.txt", content2);

    let output = Command::new(get_binary_path())
        .args(["rust", &file1, &file2])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Extract just the filename from the path for comparison
    let file1_name = std::path::Path::new(&file1)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let file2_name = std::path::Path::new(&file2)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    assert!(stdout.contains(&format!("File: {}", file1_name)));
    assert!(stdout.contains(&format!("File: {}", file2_name)));
    assert!(stdout.contains("Contains rust"));
    assert!(stdout.contains("Also contains rust"));
}
