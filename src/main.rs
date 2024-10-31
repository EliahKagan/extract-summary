//! Extract summaries from the default (human-readable) output format of `cargo nextest`.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("can't read file {path:?} due to: {error}")]
    CannotReadFile {
        path: PathBuf,
        error: std::io::Error,
    },
    #[error("can't read directory {path:?} due to: {error}")]
    CannotReadDir {
        path: PathBuf,
        error: std::io::Error,
    },
    #[error("cannot create directory {path:?} due to: {error}")]
    CannotCreateDir {
        path: PathBuf,
        error: std::io::Error,
    },
    #[error("cannot write file {path:?} due to: {error}")]
    CannotWriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
    #[error("can't find summary section in {path:?}")]
    NoSummary { path: PathBuf },
    #[error("{path:?} seems to have multiple summary sections")]
    Ambiguous { path: PathBuf },
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
enum Command {
    /// Read a single file and display its summary.
    Show {
        /// Path to a file in the `cargo nextest` human-readable output format.
        infile: PathBuf,
    },
    /// Read a directory of files. Write their summaries in a (usually other) directory.
    Batch {
        /// Path to a directory of files in the `cargo nextest` human-readable output format.
        indir: PathBuf,
        /// Path to directory in which output files consisting only of summaries will be created.
        outdir: PathBuf,
    },
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\n-{10,}\r?\n([ \t]+Summary)\b").expect("hard-coded regex pattern should be valid")
});

fn get_summary(path: PathBuf) -> Result<String, Error> {
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => return Err(Error::CannotReadFile { path, error }),
    };

    let start = PATTERN
        .captures(&text)
        .ok_or_else(|| Error::NoSummary { path: path.clone() })?
        .get(1)
        .expect("regex should not have been able to match without the capture")
        .start();

    let summary = &text[start..];
    if PATTERN.is_match(summary) {
        Err(Error::Ambiguous { path })
    } else {
        Ok(summary.into())
    }
}

fn name_outfile(path: &Path) -> Option<OsString> {
    let extension = match path.extension() {
        Some(ext) if ["log", "txt"].map(OsStr::new).contains(&ext) => ext,
        _ => return None,
    };
    let mut outfile = path
        .file_stem()
        .expect("file name should exist since its extension exists")
        .to_os_string();
    outfile.push("-summary.");
    outfile.push(extension);
    Some(outfile)
}

fn process_entry(path: PathBuf, outdir: &Path) -> Result<(), Error> {
    if let Some(outfile) = name_outfile(&path) {
        let outpath = outdir.join(outfile);
        let summary = get_summary(path)?;
        fs::write(&outpath, summary).map_err(|error| Error::CannotWriteFile {
            path: outpath,
            error,
        })
    } else {
        Ok(()) // Skip unrelated files in the output directory.
    }
}

fn main() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Show { infile } => {
            let summary = get_summary(infile)?;
            println!("{}", summary.trim_end());
        }
        Command::Batch { indir, outdir } => {
            let entries = fs::read_dir(&indir).map_err(|error| Error::CannotReadDir {
                path: indir.clone(),
                error,
            })?;

            fs::create_dir_all(&outdir).map_err(|error| Error::CannotCreateDir {
                path: outdir.clone(),
                error,
            })?;

            for maybe_entry in entries {
                match maybe_entry {
                    Ok(entry) => process_entry(entry.path(), &outdir)?,
                    Err(error) => return Err(Error::CannotReadFile { path: indir, error }),
                }
            }
        }
    }
    Ok(())
}
