use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use tiktoken_rs::{cl100k_base, p50k_base, p50k_edit, r50k_base};

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Tokenizer {
    Cl100kBase,
    P50kBase,
    P50kEdit,
    R50kBase,
}

/// Token Counter: Counts the total number of tokens in all files within a directory or matching a glob pattern.
#[derive(Parser, Debug)]
struct Args {
    /// Path to directory or glob pattern
    #[clap(short, long)]
    path: String,

    #[arg(value_enum)]
    tokenizer: Tokenizer,
}

fn count_tokens_in_file(file_path: &str, tokenizer: &Tokenizer) -> Result<usize> {
    let mut file = File::open(file_path).context("Failed to open file")?;
    let mut contents = String::new();
    let file_read = file
        .read_to_string(&mut contents)
        .context("Failed to read file");

    if file_read.is_err() {
        eprintln!("{}", file_read.unwrap_err());

        return Ok(0);
    }

    let tokens = match tokenizer {
        Tokenizer::Cl100kBase => cl100k_base().unwrap().encode_with_special_tokens(&contents),
        Tokenizer::P50kBase => p50k_base().unwrap().encode_with_special_tokens(&contents),
        Tokenizer::P50kEdit => p50k_edit().unwrap().encode_with_special_tokens(&contents),
        Tokenizer::R50kBase => r50k_base().unwrap().encode_with_special_tokens(&contents),
    };

    println!("{}: {}", file_path, tokens.len());

    Ok(tokens.len())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut total_tokens = 0;

    for entry in glob(&args.path).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => {
                let file_path = path.to_str().unwrap();
                println!("{}", file_path);

                let tokens = count_tokens_in_file(file_path, &args.tokenizer)?;
                total_tokens += tokens;
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    println!("Total tokens: {}", total_tokens);

    Ok(())
}
