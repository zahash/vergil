use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use num_bigint::BigUint;
use num_traits::Num;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
enum Cli {
    Hex {
        #[arg(value_parser = num_parser)]
        input: BigUint,
    },
    Bin {
        #[arg(value_parser = num_parser)]
        input: BigUint,
    },
    Dec {
        #[arg(value_parser = num_parser)]
        input: BigUint,
    },
    Loc {
        path: PathBuf,
    },
}

impl Cli {
    fn run(self) -> anyhow::Result<()> {
        match self {
            Cli::Hex { input } => println!("{:X}", input),
            Cli::Bin { input } => println!("{:b}", input),
            Cli::Dec { input } => println!("{}", input),
            Cli::Loc { path } => {
                match path.is_file() {
                    true => println!("{}\t :: {:?}", loc(&std::fs::read_to_string(&path)?), path),
                    false => {
                        for entry in WalkDir::new(path)
                            .into_iter()
                            .filter_entry(|entry| {
                                entry
                                    .file_name()
                                    .as_encoded_bytes()
                                    .first()
                                    .map(|byte| *byte != 0x2E) // ascii value of dot (.)
                                    .unwrap_or(false)
                            })
                            .filter_map(|e| e.ok())
                            .filter(|entry| entry.path().is_file())
                        {
                            println!(
                                "{}\t :: {:?}",
                                loc(&std::fs::read_to_string(entry.path())?),
                                entry.path(),
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn num_parser(s: &str) -> anyhow::Result<BigUint> {
    match s.get(0..2) {
        Some("0x") => <BigUint as Num>::from_str_radix(&s[2..], 16).context("invalid number"),
        Some("0b") => <BigUint as Num>::from_str_radix(&s[2..], 2).context("invalid number"),
        _ => <BigUint as Num>::from_str_radix(&s, 10).context("invalid number"),
    }
}

fn loc(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
