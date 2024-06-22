# Yazu - A JSON Formatter in Rust
![Logo](./Yazu.png)

Yazu is a high-performance JSON formatter written in Rust. It is designed to be fast, efficient, and easy to use. Yazu features a custom lexer, parser, and transpiler to ensure the highest level of performance and flexibility.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Custom Lexer:** Efficiently tokenizes JSON input.
- **Custom Parser:** Parses JSON input into a structured format.
- **Custom Transpiler:** Transforms parsed JSON into a formatted string.
- **High Performance:** Written in Rust for speed and safety.
- **Open Source:** Contributions are welcomed!

## Installation

### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Building from Source

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/yazu.git
    cd yazu
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the tests to ensure everything is working:
    ```sh
    cargo test
    ```

## Usage

Yazu can be used as a command-line tool to format JSON files.

### Formatting a JSON File

1. To format a JSON file, use the following command:
    ```sh
    cargo run --release -- <path_to_json_file>
    ```

2. The formatted JSON will be output to the console. You can redirect this output to a file if needed:
    ```sh
    cargo run --release -- <path_to_json_file> > formatted.json
    ```

## Contributing

Yazu is still a work in progress, and contributions are greatly appreciated! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
    ```sh
    git checkout -b my-new-feature
    ```
3. Make your changes.
4. Ensure the tests pass:
    ```sh
    cargo test
    ```
5. Commit your changes:
    ```sh
    git commit -am 'Add new feature'
    ```
6. Push to the branch:
    ```sh
    git push origin my-new-feature
    ```
7. Create a new Pull Request.

Please make sure to update tests as appropriate and follow the existing code style.

## License

Yazu is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Thank you for using Yazu! If you encounter any issues, feel free to open an issue on GitHub.

