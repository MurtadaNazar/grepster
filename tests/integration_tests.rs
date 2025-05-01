use assert_cmd::Command;
use tempfile::TempDir;

fn create_test_file(dir: &TempDir, filename: &str, content: &str) -> String {
    let file_path = dir.path().join(filename);
    std::fs::write(&file_path, content).unwrap();
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_cli_with_valid_arguments() {
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd.args(["rust", &file_path]).output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("This line has rust in it"));
    assert!(!stdout.contains("Rust is awesome"));
}

#[test]
fn test_cli_case_insensitive_search() {
    let temp_dir = TempDir::new().unwrap();
    let content = "Rust is awesome\nThis line has rust in it\nThis one doesn't\nRust again here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd
        .env("IGNORE_CASE", "1")
        .args(["rust", &file_path])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rust is awesome"));
    assert!(stdout.contains("This line has rust in it"));
    assert!(stdout.contains("Rust again here"));
}

#[test]
fn test_cli_with_missing_arguments() {
    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd.arg("query").output().unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Didn't get a file path"));
}

#[test]
fn test_cli_with_nonexistent_file() {
    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd
        .args(["query", "nonexistent_file.txt"])
        .output()
        .unwrap();

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

    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd
        .args(["content", &good_file, "nonexistent.txt"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stderr.contains("Error reading file"));
    assert!(stderr.contains("nonexistent.txt"));
    assert!(stdout.contains("Test content"));
}

#[test]
fn test_cli_no_matches_found() {
    let temp_dir = TempDir::new().unwrap();
    let content = "No matches here";
    let file_path = create_test_file(&temp_dir, "test.txt", content);

    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd.args(["pattern", &file_path]).output().unwrap();

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

    let mut cmd = Command::cargo_bin("grepster").unwrap();
    let output = cmd.args(["rust", &file1, &file2]).output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("file2.txt"));
    assert!(stdout.contains("Contains rust"));
    assert!(stdout.contains("Also contains rust"));
}
