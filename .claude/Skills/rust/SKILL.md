# Rust Development in Claude Code

Expert guidance for writing idiomatic, safe, and performant Rust code.

## Core Principles

**Ownership System**: Rust's ownership model eliminates data races and memory bugs at compile time. Every value has one owner, and values are dropped when owners go out of scope.

**Zero-Cost Abstractions**: High-level code compiles to performance equivalent to hand-written low-level code.

**Fearless Concurrency**: Type system prevents data races by design.

## Project Structure

### Cargo Basics

```bash
# Create new project
cargo new project_name
cd project_name

# Create library
cargo new --lib lib_name

# Project structure:
# project_name/
# ├── Cargo.toml      # Project manifest
# ├── Cargo.lock      # Dependency lock file
# └── src/
#     └── main.rs     # Binary entry point
#     └── lib.rs      # Library entry point (if --lib)
```

### Cargo.toml Structure

```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
mockall = "0.12"

[profile.release]
opt-level = 3
lto = true
```

### Common Commands

```bash
cargo build              # Debug build
cargo build --release    # Optimized build
cargo run               # Build and run
cargo check             # Fast compilation check (no executable)
cargo test              # Run tests
cargo clippy            # Lint code
cargo fmt               # Format code
cargo doc --open        # Generate and open docs
```

## Error Handling

### Result Type - The Primary Pattern

```rust
// Result<T, E> for recoverable errors
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // ? operator propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// More concise with chaining
fn read_file_short(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
```

### The ? Operator

```rust
// ? operator: if Ok, unwrap value; if Err, return early
fn process() -> Result<String, io::Error> {
    let data = read_file("data.txt")?;  // Propagates error
    let processed = transform(data)?;
    Ok(processed)
}

// Works in functions returning Result or Option
fn find_last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

### Error Handling Patterns

```rust
// unwrap: panics on error (use only when certain or prototyping)
let file = File::open("config.toml").unwrap();

// expect: panics with custom message (better for debugging)
let file = File::open("config.toml")
    .expect("config.toml must exist in project root");

// unwrap_or: provide default on error
let contents = std::fs::read_to_string("data.txt")
    .unwrap_or_default();

// unwrap_or_else: compute default lazily
let data = fetch_data().unwrap_or_else(|_| load_cached_data());

// match: explicit handling
match File::open("data.txt") {
    Ok(file) => process(file),
    Err(e) => {
        eprintln!("Failed to open file: {}", e);
        return Err(e);
    }
}
```

### Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(String),
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl std::error::Error for AppError {}

// Convert from other error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
```

### Main Function Error Handling

```rust
// main can return Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    run_app(config)?;
    Ok(())
}
```

## Ownership and Borrowing

### Core Rules

1. Each value has exactly one owner
2. When owner goes out of scope, value is dropped
3. You can have either:
   - One mutable reference (`&mut T`)
   - Any number of immutable references (`&T`)
4. References must always be valid

### References and Borrowing

```rust
// Immutable borrow: read-only access
fn calculate_length(s: &String) -> usize {
    s.len()  // Can read but not modify
}

let s1 = String::from("hello");
let len = calculate_length(&s1);  // s1 still usable after

// Mutable borrow: exclusive write access
fn append_world(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
append_world(&mut s);

// Cannot have multiple mutable borrows simultaneously
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once
```

### Ownership Transfer (Move)

```rust
// Ownership moves by default for heap-allocated types
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 is no longer valid

// Clone for deep copy
let s1 = String::from("hello");
let s2 = s1.clone();  // Both s1 and s2 are valid

// Copy types (stack-only data) don't move
let x = 5;
let y = x;  // x is still valid (integers implement Copy trait)
```

### Common Patterns

```rust
// Return owned data instead of borrowing
fn create_string() -> String {
    String::from("created")
}

// Take ownership to consume
fn process_and_drop(s: String) {
    println!("{}", s);
    // s is dropped here
}

// Borrow when you don't need ownership
fn print_string(s: &str) {
    println!("{}", s);
}

// Mutable borrow for in-place modification
fn capitalize(s: &mut String) {
    *s = s.to_uppercase();
}
```

## Idiomatic Patterns

### Iterators Over Loops

```rust
// Prefer iterators over manual indexing
let numbers = vec![1, 2, 3, 4, 5];

// Good: functional style
let sum: i32 = numbers.iter().sum();
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// Chain operations
let result: i32 = numbers.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * x)
    .sum();
```

### Match Expressions

```rust
// Exhaustive pattern matching
enum Response {
    Success(String),
    Error(String),
    NotFound,
}

fn handle(response: Response) {
    match response {
        Response::Success(data) => println!("Success: {}", data),
        Response::Error(msg) => eprintln!("Error: {}", msg),
        Response::NotFound => eprintln!("Not found"),
    }
}

// if let for single pattern
if let Some(value) = maybe_value {
    println!("Got value: {}", value);
}

// Match guards
match value {
    x if x < 0 => println!("negative"),
    x if x > 0 => println!("positive"),
    _ => println!("zero"),
}
```

### Type Inference and Turbofish

```rust
// Rust infers types when possible
let numbers = vec![1, 2, 3];  // Vec<i32> inferred

// Use turbofish when inference needs help
let parsed: i32 = "42".parse().unwrap();
// or
let parsed = "42".parse::<i32>().unwrap();

// Collect requires type annotation
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
```

### Builder Pattern

```rust
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    debug: bool,
}

impl ConfigBuilder {
    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("host is required")?,
            port: self.port.unwrap_or(8080),
            debug: self.debug,
        })
    }
}

// Usage
let config = Config::builder()
    .host("localhost")
    .port(3000)
    .debug(true)
    .build()?;
```

## Testing

```rust
// Unit tests in same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_error_case() {
        let result = divide(10, 0);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        panic!("divide by zero");
    }
}

// Integration tests in tests/ directory
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    let result = my_crate::public_function();
    assert!(result.is_ok());
}
```

## Common Dependencies

```toml
[dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client
reqwest = { version = "0.12", features = ["json"] }

# Error handling
anyhow = "1.0"      # Simple error handling
thiserror = "1.0"   # Derive Error trait

# CLI
clap = { version = "4", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Performance Tips

1. **Use `cargo check`** during development for fast feedback
2. **Profile release builds**: `cargo build --release` for benchmarking
3. **Avoid unnecessary clones**: Use references when possible
4. **Use `&str` over `String`** for function parameters that don't need ownership
5. **Prefer iterators**: They can be optimized better than manual loops
6. **Use `cargo clippy`** to catch performance issues

## Documentation

```rust
/// Calculates the sum of two numbers.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Errors
///
/// This function never returns an error.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Key Rust Concepts

- **Macros**: Code generation with `macro_rules!` or proc macros
- **Traits**: Define shared behavior (like interfaces)
- **Lifetimes**: Explicit relationship between references (usually inferred)
- **Unsafe**: Opt-out of safety checks when necessary
- **Smart Pointers**: `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`
- **Zero-sized types**: Types with no runtime cost

## When to Use What

- `String` vs `&str`: Use `&str` for parameters, `String` when you need ownership
- `Vec<T>` vs `&[T]`: Use `&[T]` for parameters (slice), `Vec<T>` when you need ownership
- `Result` vs `Option`: Use `Result` for errors with context, `Option` for simple present/absent
- `unwrap()` vs `expect()` vs `?`: Use `?` in functions returning Result/Option, `expect()` with message for debugging, avoid `unwrap()` in production
