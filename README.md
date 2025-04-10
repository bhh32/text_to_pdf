# Text-to-PDF

`text_to_pdf` is a simple command-line utility written in Rust that converts plain text files into PDF documents. It supports batch processing of multiple text files and uses the Liberation Sans font for rendering. The input text files do not have to be of .txt, but can be any of any plain text format.

---

## Features

- Converts plain text files to PDF.
- Supports multiple input files in a single command.
- Automatically applies basic formatting, including margins.
- Outputs PDFs with the same base name as the input text files.

---

## Requirements

### Prerequisites
- **Rust**: Ensure you have Rust installed (version 1.65 or later recommended). You can install Rust using [rustup](https://rustup.rs/).
- **Liberation Fonts**: The utility uses Liberation Sans for rendering text. Make sure the font is installed and accessible at `/usr/share/fonts/liberation-fonts`. You can install it using your package manager:
  - On Debian/Ubuntu: `sudo apt install fonts-liberation`
  - On Fedora: `sudo dnf install liberation-fonts`
  - On macOS (with Homebrew): `brew install --cask font-liberation-sans`

### Dependencies
This utility uses the following Rust crates:
- [`genpdf`](https://docs.rs/genpdf): For PDF generation.
- [`clap`](https://docs.rs/clap): For command-line argument parsing.

---

## Installation

### From Source
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/text-to-pdf.git
   cd text-to-pdf
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Move the compiled binary to a directory in your `$PATH` for easy access:
   ```bash
   mv target/release/text_to_pdf /usr/local/bin/
   ```

Now, you can use the `text_to_pdf` command globally.

---

## Usage

The utility takes one or more text files as input and generates corresponding PDF files in the same directory.

### Command Syntax
```bash
text_to_pdf -f file.txt -f file.toml ...
```

### Options

| Option         | Description                                | Example                                  |
|----------------|--------------------------------------------|------------------------------------------|
| `-f`, `--files` | Specify one or more text files to convert | `-f file1.txt -f file2.txt`             |

### Example

Suppose you have two text files named `file1.txt` and `file2.txt`. To convert them to PDFs, run:

```bash
text_to_pdf -f file1.txt -f file2.txt
```

This will generate two PDF files in the same directory:
- `file1.pdf`
- `file2.pdf`

---

## How It Works

1. The program reads each specified text file line by line.
2. It uses the Liberation Sans font to render the text into a PDF document.
3. Each output PDF is saved in the same directory as its corresponding input file, with the `.pdf` extension replacing `.txt`.

For example:
- Input: `example.txt`
- Output: `example.pdf`

---

## Error Handling

The program will stop execution and display an error message in the following cases:
- The specified font directory or font files are missing.
- An input text file cannot be read (e.g., due to incorrect permissions or non-existent file).
- The output PDF file cannot be written (e.g., due to insufficient disk space or permissions).

---

## License

This project is licensed under the GPL-3 License.

---

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

---

## Acknowledgments

Special thanks to the following libraries that made this project possible:
- [genpdf](https://docs.rs/genpdf): For simplifying PDF generation in Rust.
- [clap](https://docs.rs/clap): For robust and user-friendly command-line argument parsing.

---

Enjoy converting your text files to PDFs! 😊