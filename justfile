# Justfile for fexplorer development tasks

# Default command: show available recipes
default:
    @just --list

# Format code
fmt:
    cargo fmt --all

# Run clippy linter
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test --all-features

# Run tests with output
test-verbose:
    cargo test --all-features -- --nocapture

# Run unit tests only
test-unit:
    cargo test --lib

# Run integration tests only
test-integration:
    cargo test --test integration_tests

# Build debug binary
build:
    cargo build

# Build release binary
build-release:
    cargo build --release

# Build with all features
build-all:
    cargo build --release --all-features

# Run the application (pass args after --)
run *ARGS:
    cargo run -- {{ARGS}}

# Run with release optimizations
run-release *ARGS:
    cargo run --release -- {{ARGS}}

# Check code without building
check:
    cargo check --all-targets --all-features

# Full check: format, lint, test
check-all: fmt clippy test

# Clean build artifacts
clean:
    cargo clean

# Generate and open documentation
doc:
    cargo doc --all-features --open

# Run cargo audit
audit:
    cargo audit

# Install locally
install:
    cargo install --path . --all-features

# Benchmark (if we had benchmarks)
bench:
    cargo bench

# Example: list current directory with JSON output
example-list:
    cargo run -- list . --format json

# Example: tree view
example-tree:
    cargo run -- tree . --max-depth 2

# Example: find Rust files
example-find:
    cargo run -- find . --ext rs,toml

# Example: size analysis
example-size:
    cargo run -- size . --top 10
