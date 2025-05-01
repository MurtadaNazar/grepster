# Minigrep

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
cargo install minigrep
```

### From source

```bash
git clone https://github.com/MurtadaNazar/minigrep.git
cd minigrep
cargo install --path .
```

## Usage

### Basic search (case-sensitive)

```bash
minigrep pattern file.txt
```

### Search multiple files

```bash
minigrep pattern file1.txt file2.txt file3.txt
```

### Case-insensitive search

```bash
IGNORE_CASE=1 minigrep pattern file.txt
```

### Regular expression search

```bash
USE_REGEX=1 minigrep "^[A-Z].*\d+$" file.txt
```

### Display line numbers

```bash
SHOW_LINE_NUMBERS=1 minigrep pattern file.txt
```

### Combine options

```bash
IGNORE_CASE=1 USE_REGEX=1 SHOW_LINE_NUMBERS=1 minigrep pattern file.txt
```

## Examples

Search for "Rust" in a file named "programming.txt":

```bash
minigrep Rust programming.txt
```

Search for "rust" in multiple files, ignoring case:

```bash
IGNORE_CASE=1 minigrep rust *.txt
```

Search for lines starting with a function definition in Rust files:

```bash
USE_REGEX=1 minigrep "^fn\s+\w+" *.rs
```

Search for error lines in log files and show line numbers:

```bash
SHOW_LINE_NUMBERS=1 minigrep error *.log
```

## Using as a Library

You can also use minigrep as a library in your Rust projects:

```rust
use std::env;
use minigrep::{Config, run};

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

MIT License

Copyright (c) 2025 MurtadaNazar

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
