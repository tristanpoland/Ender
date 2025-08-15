
# Ender

Ender is a command-line tool written in Rust for converting text files to use Linux line endings (LF) instead of Windows line endings (CRLF). It is designed for speed and efficiency, using multithreading to process many files in parallel.

## What the Code Does

- **Command-line Parsing:** Uses the `clap` crate to accept a file path, directory, or glob pattern as input.
- **File Collection:** Determines whether the input is a file, directory, or glob pattern. If a directory is given, it recursively finds all files using `walkdir`. If a glob pattern is given, it matches all files using `glob`.
- **Parallel Processing:** Uses the `rayon` crate to process files in parallel, making the conversion fast even for large numbers of files.
- **Line Ending Conversion:** For each file, reads its contents, replaces all CRLF (`\r\n`) sequences with LF (`\n`), and writes the result back only if changes are needed.
- **Error Handling:** Prints errors for files that cannot be processed, but continues with other files.

## How It Works

1. You run the tool and provide a path, directory, or glob pattern.
2. The tool finds all matching files.
3. Each file is read and checked for Windows line endings.
4. If Windows line endings are found, they are replaced with Linux line endings and the file is updated.
5. All files are processed in parallel for speed.

## Example Usage

```powershell
ender * --help
ender "src/*.rs"
ender "C:\path\to\directory"
```

## Implementation Details

- Main logic is in `src/main.rs`.
- Uses crates: `clap`, `glob`, `rayon`, and `walkdir`.
- The conversion function only rewrites files if changes are detected, minimizing unnecessary disk writes.

## License
See the LICENSE file for details.
