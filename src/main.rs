use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(thiserror::Error, Debug, PartialEq)]
enum SummaryError {
    #[error("can't find summary section in {path:?}")]
    NotFound { path: PathBuf },
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to a file in the `cargo nextest` human-readable output format to parse.
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"\n-{10,}\r?\n([ \t]+Summary)\b")
        .expect("hard-coded regex pattern should be valid");
    let path = Cli::parse().path;
    let text = fs::read_to_string(&path)?; // TODO: Add a SummaryError case for this.
    let start = re
        .captures(&text)
        .ok_or_else(|| SummaryError::NotFound { path })?
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
