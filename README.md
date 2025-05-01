# Grepster

A simple command-line utility for searching text in files, inspired by the classic Unix grep tool.

## Features

- Search for text patterns in files
- Support for case-sensitive and case-insensitive searches
- Search multiple files at once
- Regular expression pattern matching
- Display line numbers in search results
- Simple and intuitive command-line interface

## Installation

### From crates.io

```bash
cargo install grepster
```

### From source

```bash
git clone https://github.com/MurtadaNazar/grepster.git
cd grepster
cargo install --path .
```

## Usage

### Basic search (case-sensitive)

```bash
grepster pattern file.txt
```

### Search multiple files

```bash
grepster pattern file1.txt file2.txt file3.txt
```

### Case-insensitive search

```bash
IGNORE_CASE=1 grepster pattern file.txt
```

### Regular expression search

```bash
USE_REGEX=1 grepster "^[A-Z].*\d+$" file.txt
```

### Display line numbers

```bash
SHOW_LINE_NUMBERS=1 grepster pattern file.txt
```

### Combine options

```bash
IGNORE_CASE=1 USE_REGEX=1 SHOW_LINE_NUMBERS=1 grepster pattern file.txt
```

## Examples

Search for "Rust" in a file named "programming.txt":

```bash
grepster Rust programming.txt
```

Search for "rust" in multiple files, ignoring case:

```bash
IGNORE_CASE=1 grepster rust *.txt
```

Search for lines starting with a function definition in Rust files:

```bash
USE_REGEX=1 grepster "^fn\s+\w+" *.rs
```

Search for error lines in log files and show line numbers:

```bash
SHOW_LINE_NUMBERS=1 grepster error *.log
```

## Using as a Library

You can also use grepster as a library in your Rust projects:

```rust
use std::env;
use grepster::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
```

## License

See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---
