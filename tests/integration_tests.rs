use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test directory structure
fn create_test_tree() -> TempDir {
    let dir = TempDir::new().unwrap();

    // Create directory structure
    fs::create_dir(dir.path().join("subdir")).unwrap();
    fs::create_dir(dir.path().join("subdir/nested")).unwrap();

    // Create files
    fs::write(dir.path().join("file1.txt"), "content1").unwrap();
    fs::write(dir.path().join("file2.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("subdir/file3.txt"), "content3").unwrap();
    fs::write(dir.path().join("subdir/nested/file4.md"), "# Documentation").unwrap();

    // Create hidden file
    fs::write(dir.path().join(".hidden"), "secret").unwrap();

    dir
}

#[test]
fn test_list_basic() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.rs"));
}

#[test]
fn test_list_max_depth() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--max-depth")
        .arg("1")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file4.md").not());
}

#[test]
fn test_list_hidden() {
    let test_dir = create_test_tree();

    // Without --hidden
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden").not());

    // With --hidden
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--hidden")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"));
}

#[test]
fn test_tree_basic() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("tree")
        .arg(test_dir.path())
        .arg("--no-color")
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"));
}

#[test]
fn test_find_by_extension() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("find")
        .arg(test_dir.path())
        .arg("--ext")
        .arg("rs")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("file2.rs"))
        .stdout(predicate::str::contains("file1.txt").not());
}

#[test]
fn test_find_by_name_glob() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("find")
        .arg(test_dir.path())
        .arg("--name")
        .arg("*.txt")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file3.txt"))
        .stdout(predicate::str::contains("file2.rs").not());
}

#[test]
fn test_find_by_regex() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("find")
        .arg(test_dir.path())
        .arg("--regex")
        .arg("^file[0-9]\\.txt$")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file3.txt"))
        .stdout(predicate::str::contains("file2.rs").not());
}

#[test]
fn test_size_basic() {
    let test_dir = create_test_tree();

    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("size")
        .arg(test_dir.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"size\""));
}

#[test]
fn test_size_top_n() {
    let test_dir = create_test_tree();

    let output = Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("size")
        .arg(test_dir.path())
        .arg("--top")
        .arg("2")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    assert!(output.status.success());

    // Parse JSON and verify we have at most 2 entries
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let entries = json.as_array().unwrap();
    assert!(entries.len() <= 2);
}

#[test]
fn test_output_formats() {
    let test_dir = create_test_tree();

    // JSON
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("["));

    // NDJSON
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("ndjson")
        .assert()
        .success()
        .stdout(predicate::str::contains("{\"path\""));

    // CSV
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("path,size,mtime,kind"));
}

#[test]
fn test_gitignore_respect() {
    use std::process::Command as StdCommand;

    let test_dir = TempDir::new().unwrap();

    // Initialize git repository (required for ignore crate to work)
    StdCommand::new("git")
        .args(&["init"])
        .current_dir(test_dir.path())
        .output()
        .expect("failed to initialize git repo");

    // Create .gitignore
    fs::write(test_dir.path().join(".gitignore"), "ignored.txt\n").unwrap();

    // Create ignored and non-ignored files
    fs::write(test_dir.path().join("ignored.txt"), "should be ignored").unwrap();
    fs::write(test_dir.path().join("visible.txt"), "should be visible").unwrap();

    // Without --no-gitignore (default)
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("visible.txt"))
        .stdout(predicate::str::contains("ignored.txt").not());

    // With --no-gitignore
    Command::cargo_bin("fexplorer")
        .unwrap()
        .arg("list")
        .arg(test_dir.path())
        .arg("--no-gitignore")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("visible.txt"))
        .stdout(predicate::str::contains("ignored.txt"));
}
