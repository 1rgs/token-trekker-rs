# token_trekker_rs

`token_trekker_rs` is a command-line tool for counting the total number of tokens in all files within a directory or matching a glob pattern, using various tokenizers.

## Features

- Supports multiple tokenizer options
- Parallel processing for faster token counting
- Outputs results in a colorized table

## Installation

To install `token_trekker_rs` from crates.io, run:

```sh
cargo install token_trekker_rs
```

## Building from Source

To build token_trekker_rs from the source code, first clone the repository:

```sh
git clone https://github.com/1rgs/token_trekker_rs.git
cd token_trekker_rs
```

Then build the project using cargo:

```sh
cargo build --release
```

The compiled binary will be available at ./target/release/token-trekker.

## Usage

To count tokens in a directory or for files matching a glob pattern, run the following command:

```sh
token-trekker --path <path_or_glob_pattern> <tokenizer>
```

Replace <path_or_glob_pattern> with the path to the directory or the glob pattern of the files to process, and <tokenizer> with one of the available tokenizer options:

- p50k-base
- p50k-edit
- r50k-base
- cl100k-base
- gpt2

For example:

```sh
token_trekker_rs --path "path/to/files/*.txt" p50k-base
```
