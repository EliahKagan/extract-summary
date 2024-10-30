//! Extract summaries from the default (human-readable) output format of `cargo nextest`.

use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

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
    #[error("{path:?} seems to have multiple summary sections")]
    Ambiguous { path: PathBuf },
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to a file in the `cargo nextest` human-readable output format to parse.
    path: PathBuf,
}

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\n-{10,}\r?\n([ \t]+Summary)\b").expect("hard-coded regex pattern should be valid")
});

fn get_summary(path: PathBuf) -> Result<String, SummaryError> {
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => return Err(SummaryError::CannotRead { path, error }),
    };

    let start = PATTERN
        .captures(&text)
        .ok_or_else(|| SummaryError::NoSummary { path: path.clone() })?
        .get(1)
        .expect("regex should not have been able to match without the capture")
        .start();

    let summary = text[start..].trim_end();
    if PATTERN.is_match(summary) {
        Err(SummaryError::Ambiguous { path })
    } else {
        Ok(summary.into())
    }
}

fn main() -> Result<(), SummaryError> {
    let path = Cli::parse().path;
    println!("{}", get_summary(path)?);
    Ok(())
}
