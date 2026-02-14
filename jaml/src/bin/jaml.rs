use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use jaml::{
    formatter::{BinaryEncoding, Options, QuoteStyle, TimestampPrecision, format_with_opts},
    parse,
};

/// JAML - Just Another Markup Language CLI tool
#[derive(Parser)]
#[command(name = "jaml")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format and prettify JAML files
    #[command(alias = "fmt")]
    Format {
        /// Input file (use '-' or omit for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Output file (use '-' or omit for stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Quote style for strings
        #[arg(long, value_enum, default_value = "double")]
        quotes: QuoteStyleArg,

        /// Binary encoding format
        #[arg(long, value_enum, default_value = "base64")]
        binary: BinaryEncodingArg,

        /// Quote all object keys
        #[arg(long)]
        quote_keys: bool,

        /// Add leading plus sign to positive numbers
        #[arg(long)]
        leading_plus: bool,

        /// Don't sort object keys
        #[arg(long)]
        no_sort_keys: bool,

        /// Escape all non-ASCII characters as \uXXXX
        #[arg(long)]
        escape_unicode: bool,

        /// Use '+00:00' for UTC timestamps instead of 'Z'
        #[arg(long)]
        no_zulu: bool,

        /// Timestamp precision for fractional seconds
        #[arg(long, value_enum, default_value = "auto")]
        timestamp_precision: TimestampPrecisionArg,

        /// Check if file is already formatted (exit 1 if not)
        #[arg(long)]
        check_format: bool,
    },

    /// Check JAML syntax
    #[command(alias = "chk")]
    Check {
        /// Input files to validate (use '-' for stdin)
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,

        /// Show detailed parse tree on success
        #[arg(short, long)]
        verbose: bool,

        /// Suppress success messages, only show errors
        #[arg(short, long)]
        quiet: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum QuoteStyleArg {
    Double,
    Single,
    Prefer,
}

impl From<QuoteStyleArg> for QuoteStyle {
    fn from(arg: QuoteStyleArg) -> Self {
        match arg {
            QuoteStyleArg::Double => QuoteStyle::Double,
            QuoteStyleArg::Single => QuoteStyle::Single,
            QuoteStyleArg::Prefer => QuoteStyle::PreferDouble,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum BinaryEncodingArg {
    Base64,
    Hex,
}

impl From<BinaryEncodingArg> for BinaryEncoding {
    fn from(arg: BinaryEncodingArg) -> Self {
        match arg {
            BinaryEncodingArg::Base64 => BinaryEncoding::Base64,
            BinaryEncodingArg::Hex => BinaryEncoding::Hex,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum TimestampPrecisionArg {
    Auto,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl From<TimestampPrecisionArg> for TimestampPrecision {
    fn from(arg: TimestampPrecisionArg) -> Self {
        match arg {
            TimestampPrecisionArg::Auto => TimestampPrecision::Auto,
            TimestampPrecisionArg::Seconds => TimestampPrecision::Seconds,
            TimestampPrecisionArg::Milliseconds => TimestampPrecision::Milliseconds,
            TimestampPrecisionArg::Microseconds => TimestampPrecision::Microseconds,
            TimestampPrecisionArg::Nanoseconds => TimestampPrecision::Nanoseconds,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Format {
            input,
            output,
            quotes,
            binary,
            quote_keys,
            leading_plus,
            no_sort_keys,
            escape_unicode,
            no_zulu,
            timestamp_precision,
            check_format,
        } => cmd_fmt(
            input,
            output,
            quotes,
            binary,
            quote_keys,
            leading_plus,
            no_sort_keys,
            escape_unicode,
            no_zulu,
            timestamp_precision,
            check_format,
        ),
        Commands::Check {
            files,
            verbose,
            quiet,
        } => cmd_valid(files, verbose, quiet),
        Commands::Completions { shell } => {
            cmd_completions(shell);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

#[allow(clippy::too_many_arguments)]
fn build_format_options(
    quotes: QuoteStyleArg,
    binary: BinaryEncodingArg,
    quote_keys: bool,
    leading_plus: bool,
    no_sort_keys: bool,
    escape_unicode: bool,
    no_zulu: bool,
    timestamp_precision: TimestampPrecisionArg,
) -> Options {
    Options::new()
        .with_quote_style(quotes.into())
        .with_binary_encoding(binary.into())
        .with_unquoted_keys(!quote_keys)
        .with_leading_plus(leading_plus)
        .with_sort_keys(!no_sort_keys)
        .with_escape_unicode(escape_unicode)
        .with_use_zulu(!no_zulu)
        .with_timestamp_precision(timestamp_precision.into())
}

#[allow(clippy::too_many_arguments)]
fn cmd_fmt(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    quotes: QuoteStyleArg,
    binary: BinaryEncodingArg,
    quote_keys: bool,
    leading_plus: bool,
    no_sort_keys: bool,
    escape_unicode: bool,
    no_zulu: bool,
    timestamp_precision: TimestampPrecisionArg,
    check_format: bool,
) -> Result<()> {
    // Read input
    let input_content = read_input(input.as_deref())?;

    // Parse JAML
    let value = parse(&input_content).context("Failed to parse JAML")?;

    // Build formatting options
    let opts = build_format_options(
        quotes,
        binary,
        quote_keys,
        leading_plus,
        no_sort_keys,
        escape_unicode,
        no_zulu,
        timestamp_precision,
    );

    // Format
    let formatted = format_with_opts(&value, &opts);

    // Check mode: compare and exit
    if check_format {
        check_formatting(&input_content, &formatted, input.as_deref());
        return Ok(());
    }

    // Write output
    write_output(output.as_deref(), &formatted)?;

    Ok(())
}

fn cmd_valid(files: Vec<PathBuf>, verbose: bool, quiet: bool) -> Result<()> {
    if files.is_empty() {
        // Read from stdin
        return validate_file(None, verbose, quiet);
    }

    let mut all_valid = true;
    let mut error_count = 0;

    for file in &files {
        let file_path = parse_file_arg(file);

        match validate_file(file_path, verbose, quiet) {
            Ok(()) => {
                if !quiet {
                    println!("✓ {}", file.display());
                }
            }
            Err(e) => {
                eprintln!("✗ {}: {:#}", file.display(), e);
                all_valid = false;
                error_count += 1;
            }
        }
    }

    if !all_valid {
        eprintln!("\n{} file(s) failed validation", error_count);
        process::exit(1);
    } else if files.len() > 1 && !quiet {
        println!("\nAll {} file(s) are valid", files.len());
    }

    Ok(())
}

fn validate_file(path: Option<&Path>, verbose: bool, quiet: bool) -> Result<()> {
    let content = read_input(path)?;
    let value = parse(&content).context("Invalid JAML syntax")?;

    if verbose {
        println!("Valid JAML: {:#?}", value);
    } else if path.is_none() && !quiet {
        println!("Valid JAML");
    }

    Ok(())
}

fn check_formatting(input: &str, formatted: &str, path: Option<&Path>) {
    if input.trim() != formatted.trim() {
        let name = display_name(path);
        eprintln!("File '{}' is not formatted correctly", name);
        process::exit(1);
    }
}

fn display_name(path: Option<&Path>) -> &str {
    path.and_then(|p| p.to_str()).unwrap_or("stdin")
}

fn read_input(path: Option<&Path>) -> Result<String> {
    match path {
        Some(p) if p.to_str() != Some("-") => {
            fs::read_to_string(p).with_context(|| format!("Failed to read file: {}", p.display()))
        }
        _ => {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .context("Failed to read from stdin")?;
            Ok(content)
        }
    }
}

fn write_output(path: Option<&Path>, content: &str) -> Result<()> {
    match path {
        Some(p) if p.to_str() != Some("-") => {
            fs::write(p, content).with_context(|| format!("Failed to write file: {}", p.display()))
        }
        _ => writeln!(io::stdout(), "{}", content).context("Failed to write to stdout"),
    }
}

fn cmd_completions(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

fn parse_file_arg(file: &Path) -> Option<&Path> {
    if file.to_str() == Some("-") {
        None
    } else {
        Some(file)
    }
}
