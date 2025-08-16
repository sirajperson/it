![it Logo](logo.jpg)

# it - A Command-Line Text Insertion Tool

`it` is a simple command-line tool written in Rust for inserting, appending, or clearing text in files at specified line numbers. It supports interactive input, backups, dry runs, and multiple file processing.

## Features

- Insert text at a specific line (`-i, --insert`).
- Append text to the end of a file (`-a, --append`).
- Clear lines from a start line to the end or a specific range (`-z, --clear`).
- Overwrite a specific line (`-o, --overwrite`).
- Create backups before modifying files (`-b, --backup`).
- Read text interactively from stdin (`-I, --interactive`).
- Preview changes without modifying files (`-d, --dry-run`).
- Process multiple files in one command.

## Installation

### Prerequisites
- Rust and Cargo (install via [rustup](https://rustup.rs/)).
- A Unix-like system (Linux, macOS) or Windows with a compatible shell.

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/it.git
   cd it
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Install the binary (optional):
   ```bash
   sudo cp target/release/it /usr/local/bin/
   ```

## Usage

Run `it --help` to see all options:

```bash
it 1.0.0
Inserts text at a given line location of a file

Usage: it [OPTIONS] <FILE>...

Arguments:
  <FILE>...  The file(s) to modify

Options:
  -h, --help                Shows help information
  -l, --line <NUMBER>       The line number to start inserting or clearing at
  -o, --overwrite           Overwrite the line instead of inserting
  -i, --insert <TEXT>       Inserts text at the line provided by the line flag (default: first line)
  -a, --append <TEXT>       Inserts text at the last line of the file
  -z, --clear <START[,END]> Clear to end of file from line given by --line or START. If START,END provided, clear range.
  -b, --backup              Create a backup of the original file (adds .bak extension)
  -I, --interactive         Read text to insert or append from stdin
  -d, --dry-run             Print changes to stdout without modifying the file
  -v, --version             Prints version information
```

### Examples

- Insert "New Line" at line 2:
  ```bash
  it -i "New Line" -l 2 file.txt
  ```

- Overwrite line 2 with "Overwritten":
  ```bash
  it -i "Overwritten" -l 2 -o file.txt
  ```

- Append "Appended" to the end:
  ```bash
  it -a "Appended" file.txt
  ```

- Clear from line 2 to the end:
  ```bash
  it -z 2 file.txt
  ```

- Clear from line 2 to line 3:
  ```bash
  it -z 2,3 file.txt
  ```

- Interactively append text:
  ```bash
  echo "Appended" | it -a -I file.txt
  ```

- Process multiple files with a backup:
  ```bash
  it -b -a "End" file1.txt file2.txt
  ```

- Preview changes without modifying:
  ```bash
  it -d -i "New Line" -l 2 file.txt
  ```


### Install via APT (Debian/Ubuntu)
1. Add the repo:
 ```bash
   sudo echo "deb [trusted=yes] https://sirajperson.github.io/it/ ./" > /etc/apt/sources.list.d/it.list
```

2. Update and install:
 ```bash
   sudo apt update
   sudo apt install it
```

## Contributing

Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please include tests for new features and follow the Rust style guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
