use std::fs;
use std::path::PathBuf;

use clap::Parser;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
enum SummaryError {
    #[error("can't read {path:?} due to: {error}")]
    CannotRead {
        path: PathBuf,
        error: std::io::Error,
    },
    #[error("can't find summary section in {path:?}")]
    NoSummary { path: PathBuf },
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to a file in the `cargo nextest` human-readable output format to parse.
    path: PathBuf,
}

fn main() -> Result<(), SummaryError> {
    let re = Regex::new(r"\n-{10,}\r?\n([ \t]+Summary)\b")
        .expect("hard-coded regex pattern should be valid");
    let path = Cli::parse().path;
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => return Err(SummaryError::CannotRead { path, error }),
    };
    let start = re
        .captures(&text)
        .ok_or_else(|| SummaryError::NoSummary { path })?
        .get(1)
        .expect("regex should not have been able to match without the capture")
        .start();
    let summary = &text[start..];
    print!("{}", summary);
    if !summary.ends_with('\n') {
        println!();
    }
    Ok(())
}
