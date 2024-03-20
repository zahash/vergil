use clap::Parser;

use num_bigint::BigUint;
use num_traits::Num;

#[derive(Parser, Debug)]
enum Cli {
    Hex { input: String },
    Bin { input: String },
    Dec { input: String },
}

#[derive(Debug)]
enum CliError {
    InvalidNum,
}

fn invalid_num<E>(_: E) -> CliError {
    CliError::InvalidNum
}

impl Cli {
    fn run(self) -> Result<(), CliError> {
        match self {
            Cli::Hex { input } => println!("{:X}", parse(&input)?),
            Cli::Bin { input } => println!("{:b}", parse(&input)?),
            Cli::Dec { input } => println!("{}", parse(&input)?),
        }
        Ok(())
    }
}

fn parse(s: &str) -> Result<BigUint, CliError> {
    match s.get(0..2) {
        Some("0x") => <BigUint as Num>::from_str_radix(&s[2..], 16).map_err(invalid_num),
        Some("0b") => <BigUint as Num>::from_str_radix(&s[2..], 2).map_err(invalid_num),
        _ => <BigUint as Num>::from_str_radix(&s, 2).map_err(invalid_num),
    }
}

fn main() -> Result<(), CliError> {
    Cli::parse().run()
}
