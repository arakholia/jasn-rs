use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    process,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use jasn::{
    formatter::{BinaryEncoding, Options, QuoteStyle, format_with_opts},
    parse,
};

/// JASN - Just Another Serialization Notation CLI tool
#[derive(Parser)]
#[command(name = "jasn")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format and prettify JASN files
    #[command(alias = "fmt")]
    Format {
        /// Input file (use '-' or omit for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Output file (use '-' or omit for stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Use compact format (no whitespace)
        #[arg(short, long)]
        compact: bool,

        /// Indentation string (default: 2 spaces)
        #[arg(long, default_value = "  ")]
        indent: String,

        /// Quote style for strings
        #[arg(long, value_enum, default_value = "double")]
        quotes: QuoteStyleArg,

        /// Binary encoding format
        #[arg(long, value_enum, default_value = "base64")]
        binary: BinaryEncodingArg,

        /// Disable trailing commas
        #[arg(long)]
        no_trailing_commas: bool,

        /// Quote all object keys
        #[arg(long)]
        quote_keys: bool,

        /// Don't sort object keys
        #[arg(long)]
        no_sort_keys: bool,

        /// Escape all non-ASCII characters as \uXXXX
        #[arg(long)]
        escape_unicode: bool,

        /// Check if file is already formatted (exit 1 if not)
        #[arg(long)]
        check_format: bool,
    },

    /// Check JASN syntax
    #[command(alias = "chk")]
    Check {
        /// Input files to validate (use '-' for stdin)
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,

        /// Show detailed parse tree on success
        #[arg(short, long)]
        verbose: bool,
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

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Format {
            input,
            output,
            compact,
            indent,
            quotes,
            binary,
            no_trailing_commas,
            quote_keys,
            no_sort_keys,
            escape_unicode,
            check_format,
        } => cmd_fmt(
            input,
            output,
            compact,
            indent,
            quotes,
            binary,
            no_trailing_commas,
            quote_keys,
            no_sort_keys,
            escape_unicode,
            check_format,
        ),
        Commands::Check { files, verbose } => cmd_valid(files, verbose),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn cmd_fmt(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    compact: bool,
    indent: String,
    quotes: QuoteStyleArg,
    binary: BinaryEncodingArg,
    no_trailing_commas: bool,
    quote_keys: bool,
    no_sort_keys: bool,
    escape_unicode: bool,
    check_format: bool,
) -> Result<()> {
    // Read input
    let input_content = read_input(input.as_deref())?;

    // Parse JASN
    let value = parse(&input_content).context("Failed to parse JASN")?;

    // Build formatting options
    let mut opts = if compact {
        Options::compact()
    } else {
        Options::pretty().with_indent(indent)
    };

    opts = opts
        .with_quote_style(quotes.into())
        .with_binary_encoding(binary.into())
        .with_trailing_commas(!no_trailing_commas)
        .with_unquoted_keys(!quote_keys)
        .with_sort_keys(!no_sort_keys)
        .with_escape_unicode(escape_unicode);

    // Format
    let formatted = format_with_opts(&value, &opts);

    // Check mode: compare and exit
    if check_format {
        let input_trimmed = input_content.trim();
        let formatted_trimmed = formatted.trim();

        if input_trimmed != formatted_trimmed {
            let input_name = input.as_ref().and_then(|p| p.to_str()).unwrap_or("stdin");
            eprintln!("File '{}' is not formatted correctly", input_name);
            process::exit(1);
        }
        return Ok(());
    }

    // Write output
    write_output(output.as_deref(), &formatted)?;

    Ok(())
}

fn cmd_valid(files: Vec<PathBuf>, verbose: bool) -> Result<()> {
    if files.is_empty() {
        // Read from stdin
        return validate_file(None, verbose);
    }

    let mut all_valid = true;
    let mut error_count = 0;

    for file in &files {
        let file_arg = if file.to_str() == Some("-") {
            None
        } else {
            Some(file.as_path())
        };

        match validate_file(file_arg, verbose) {
            Ok(()) => {
                println!("✓ {}", file.display());
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
    } else if files.len() > 1 {
        println!("\nAll {} file(s) are valid", files.len());
    }

    Ok(())
}

fn validate_file(path: Option<&std::path::Path>, verbose: bool) -> Result<()> {
    let content = read_input(path)?;
    let value = parse(&content).context("Invalid JASN syntax")?;

    if verbose {
        println!("Valid JASN: {:#?}", value);
    } else if path.is_none() {
        println!("Valid JASN");
    }

    Ok(())
}

fn read_input(path: Option<&std::path::Path>) -> Result<String> {
    match path {
        Some(p) if p.to_str() == Some("-") => {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .context("Failed to read from stdin")?;
            Ok(content)
        }
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display())),
        None => {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .context("Failed to read from stdin")?;
            Ok(content)
        }
    }
}

fn write_output(path: Option<&std::path::Path>, content: &str) -> Result<()> {
    match path {
        Some(p) if p.to_str() == Some("-") => {
            io::stdout()
                .write_all(content.as_bytes())
                .context("Failed to write to stdout")?;
            // Add newline for better terminal output
            println!();
            Ok(())
        }
        Some(path) => {
            fs::write(path, content)
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
            Ok(())
        }
        None => {
            io::stdout()
                .write_all(content.as_bytes())
                .context("Failed to write to stdout")?;
            // Add newline for better terminal output
            println!();
            Ok(())
        }
    }
}
