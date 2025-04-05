# Echo - A Terminal-Based Text Editor

Echo is a lightweight, terminal-based text editor written in Rust. It provides a simple and efficient way to edit text files directly from the command line.

## Features

- Terminal-based interface
- Basic text editing capabilities
- File loading and saving
- Cursor movement (arrow keys, home, end, page up/down)
- Status bar showing file information
- Support for UTF-8 text
- Terminal resizing support

## Installation

### Prerequisites

- Rust and Cargo (Rust's package manager)
- A terminal that supports ANSI escape codes

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/echo.git
cd echo
```

2. Build the project:
```bash
cargo build --release
```

3. Run the editor:
```bash
cargo run --release [filename]
```

## Usage

### Basic Commands

- `Ctrl + q`: Quit the editor
- `s`: Save the current file
- Arrow keys: Move the cursor
- Home/End: Move to start/end of line
- Page Up/Page Down: Scroll through the document
- Backspace/Delete: Delete characters
- Enter: Insert new line
- Tab: Insert tab character

### Opening Files

You can open a file by passing its name as an argument:
```bash
cargo run --release path/to/file.txt
```

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/editor.rs`: Core editor functionality
- `src/terminal.rs`: Terminal handling and screen management
- `src/view.rs`: Document view and rendering
- `src/statusbar.rs`: Status bar implementation

## Dependencies

- `crossterm`: Terminal manipulation library
- `derive_more`: Additional derive macros
- `unicode-segmentation`: Unicode text segmentation
- `unicode-width`: Unicode character width calculation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 