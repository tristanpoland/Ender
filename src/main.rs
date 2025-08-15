use clap::Parser;
use glob::glob;
use rayon::prelude::*;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

/// CLI tool to convert files to Linux line endings (LF) using multithreading
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Glob pattern or file/directory path
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();
    let paths = collect_files(&args.path);
    println!("Found {} files", paths.len());
    paths.par_iter().for_each(|path| {
        if let Err(e) = convert_to_lf(path) {
            eprintln!("Failed to process {}: {}", path.display(), e);
        }
    });
}

fn collect_files(pattern: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = PathBuf::from(pattern);
    if path.is_file() {
        files.push(path);
    } else if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    } else {
        for entry in glob(pattern).expect("Invalid glob pattern") {
            if let Ok(path) = entry {
                if path.is_file() {
                    files.push(path);
                }
            }
        }
    }
    files
}

fn convert_to_lf(path: &PathBuf) -> io::Result<()> {
    let mut content = Vec::new();
    {
        let mut file = fs::File::open(path)?;
        file.read_to_end(&mut content)?;
    }
    let lf_content = replace_crlf(&content);
    if lf_content != content {
        let mut file = fs::File::create(path)?;
        file.write_all(&lf_content)?;
    }
    Ok(())
}

fn replace_crlf(content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'\r' && i + 1 < content.len() && content[i + 1] == b'\n' {
            out.push(b'\n');
            i += 2;
        } else {
            out.push(content[i]);
            i += 1;
        }
    }
    out
}
