use clap::Parser;
use num_bigint::BigUint;
use num_traits::Num;

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
}

fn num_parser(s: &str) -> Result<BigUint, CliError> {
    match s.get(0..2) {
        Some("0x") => {
            <BigUint as Num>::from_str_radix(&s[2..], 16).map_err(|_| CliError::InvalidNum)
        }
        Some("0b") => {
            <BigUint as Num>::from_str_radix(&s[2..], 2).map_err(|_| CliError::InvalidNum)
        }
        _ => <BigUint as Num>::from_str_radix(&s, 10).map_err(|_| CliError::InvalidNum),
    }
}

#[derive(thiserror::Error, Debug)]
enum CliError {
    #[error("invalid number")]
    InvalidNum,
}

impl Cli {
    fn run(self) -> Result<(), CliError> {
        match self {
            Cli::Hex { input } => println!("{:X}", input),
            Cli::Bin { input } => println!("{:b}", input),
            Cli::Dec { input } => println!("{}", input),
        }
        Ok(())
    }
}

fn main() -> Result<(), CliError> {
    Cli::parse().run()
}
