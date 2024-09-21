
# Small-Project-File-Decompressor

A simple and efficient command-line tool written in Rust for decompressing ZIP archives.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Example](#example)
- [Dependencies](#dependencies)
- [License](#license)

## Features

- **Easy to Use**: Requires only the ZIP file as an argument to decompress.
- **Fast Extraction**: Efficiently extracts files and directories from ZIP archives.
- **File Comments**: Displays comments associated with each file in the archive.
- **Cross-Platform**: Compatible with both Windows and Unix-like systems, handling file permissions appropriately.
- **Performance Metrics**: Reports the time taken to extract the archive.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Building from Source

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/Small-Project-File-Decompressor.git
   ```

2. **Navigate to the Project Directory**

   ```bash
   cd Small-Project-File-Decompressor
   ```

3. **Build the Project Using Cargo**

   ```bash
   cargo build --release <folder_name>
   ```

4. **Locate the Executable**

   The compiled executable will be located in the `target/release` directory.

## Example Output

```bash
File 0 extracted to "folder/" 
File 1 extracted to "folder/file.txt" (1024 bytes)
File 2 comment: This is a sample file
File 3 extracted to "another_folder/" 
File 4 extracted to "another_folder/image.png" (2048 bytes)
File extracted in 0.123456s
```

## Dependencies

- [zip](https://crates.io/crates/zip) - Handles ZIP archive operations.

Ensure that your `Cargo.toml` includes the following dependency:

```toml
[dependencies]
zip = "2.2.0"
```

## License

This project is licensed under the [MIT License](LICENSE).
