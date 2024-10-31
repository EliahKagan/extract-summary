# extract-summary - Extract summaries from `cargo nextest` output

This extracts the summary section that appears at the end of `cargo nextest` output.

It has:

- a `show` subcommand that takes the path to a file and displays the summary (and which is mostly just for testing), *and*
- a `batch` subcommand that paths to input and output directories and extracts summaries from multiple files and saves them to multiple associated files.

## Usage

```text
$ extract-summary help
Usage: extract-summary <COMMAND>

Commands:
  show   Read a single file and display its summary
  batch  Read a directory of files. Write their summaries in a (usually other) directory
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ extract-summary help show
Read a single file and display its summary

Usage: extract-summary show <INFILE>

Arguments:
  <INFILE>  Path to a file in the `cargo nextest` human-readable output format

Options:
  -h, --help  Print help
```

```text
$ extract-summary help batch
Read a directory of files. Write their summaries in a (usually other) directory

Usage: extract-summary batch <INDIR> <OUTDIR>

Arguments:
  <INDIR>   Path to a directory of files in the `cargo nextest` human-readable output format
  <OUTDIR>  Path to directory in which output files consisting only of summaries will be created

Options:
  -h, --help  Print help
```

## License

[0BSD](LICENSE)
