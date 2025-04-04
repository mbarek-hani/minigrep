# Minigrep

Minigrep is a lightweight command-line tool for searching text within files. It mimics the functionality of `grep` but is implemented in Rust, making it fast and efficient.

## Features

-   Search for a specific string or pattern in a file using regular expressions.
-   Simple and intuitive usage.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/minigrep.git
    cd minigrep
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the executable:
    ```bash
    ./target/release/minigrep
    ```

## Usage

### Example Usages

1. Search for a word in a file:

    ```bash
    ./target/release/minigrep word filename.txt
    ```

2. Search for a pattern using regular expressions:

    ```bash
    ./target/release/minigrep [^A-Z] filename.txt
    ```

3. Display help information:
    ```bash
    ./target/release/minigrep --help
    ```
