use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::Colorize;
use glob::glob;
use prettytable::{format, row, Table};
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use tiktoken_rs::{cl100k_base, p50k_base, p50k_edit, r50k_base, CoreBPE};

type TokenizerFunction = fn() -> Result<CoreBPE>;

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Tokenizer {
    Cl100kBase,
    P50kBase,
    P50kEdit,
    R50kBase,
    Gpt2,
}

impl Tokenizer {
    fn function(self) -> TokenizerFunction {
        match self {
            Tokenizer::Cl100kBase => cl100k_base,
            Tokenizer::P50kBase => p50k_base,
            Tokenizer::P50kEdit => p50k_edit,
            Tokenizer::R50kBase => r50k_base,
            Tokenizer::Gpt2 => r50k_base,
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    path: String,

    #[arg(value_enum)]
    tokenizer: Tokenizer,
}

fn count_tokens_in_file(file_path: &str, tokenizer_fn: TokenizerFunction) -> Option<usize> {
    if std::path::Path::new(file_path).is_dir() {
        return None;
    }

    let file = File::open(file_path);
    if file.is_err() {
        eprintln!("Failed to open file: {}", file_path);
        return None;
    }
    let mut contents = String::new();
    let file_read = file
        .unwrap()
        .read_to_string(&mut contents)
        .context("Failed to read file contents");

    if file_read.is_err() {
        eprintln!("Failed to read file: {}", file_path);
        return None;
    }

    let tokenizer = tokenizer_fn().unwrap();
    let tokens = tokenizer.encode_with_special_tokens(&contents);

    Some(tokens.len())
}
fn main() -> Result<()> {
    let args = Args::parse();

    let entries: Result<Vec<_>, _> = glob(&args.path)
        .context("Failed to read glob pattern")?
        .collect();
    let entries = entries?;

    let tokenizer_fn = args.tokenizer.function();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.add_row(row!["Filename".cyan(), "Tokens".cyan()]);

    let vec = entries
        .par_iter()
        .filter_map(|entry| {
            let entry = entry.to_str().unwrap();
            let tokens = count_tokens_in_file(entry, tokenizer_fn)?;
            return Some((entry, tokens));
        })
        .collect::<Vec<_>>();

    for (filename, tokens) in &vec {
        table.add_row(row![filename, tokens.to_string().bold().cyan()]);
    }

    let total = vec.iter().fold(0, |acc, (_, tokens)| acc + tokens);

    table.printstd();

    println!(
        "{}: {}",
        "Total tokens".bold(),
        total.to_string().bold().cyan()
    );

    Ok(())
}
